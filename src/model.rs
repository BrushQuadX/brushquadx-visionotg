use serde_json;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::time::Duration;

use ndarray::{Array, Array1, Array3};
use ort::{
    inputs,
    session::{Session, builder::GraphOptimizationLevel},
    value::TensorRef,
};

use crate::camera::Frame;

pub type SharedDetections = Arc<Mutex<Array3<f32>>>; // 3D array containing model detections

#[derive(Clone, Debug, PartialEq)]
pub enum Normalization {
    Unsigned,
    Signed,
    Raw,
}

impl Normalization {
    // Returns a function pointer that takes an f32 and returns an f32
    fn get_lambda(&self) -> fn(f32) -> f32 {
        match self {
            Self::Unsigned => |x| x / 255.0,
            Self::Signed => |x| (x / 127.5) - 1.0,
            Self::Raw => |x| x,
        }
    }

    // Returns string representation of the normalization
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unsigned => "unsigned",
            Self::Signed => "signed",
            Self::Raw => "raw",
        }
    }

    // Returns the normalization type from a string
    pub fn from_str(norm: &str) -> Self {
        match norm {
            "unsigned" => Self::Unsigned,
            "signed" => Self::Signed,
            "raw" => Self::Raw,
            _ => Self::Raw,
        }
    }

    pub fn str_variants() -> Vec<String> {
        vec![
            "unsigned".to_string(),
            "signed".to_string(),
            "raw".to_string(),
        ]
    }
}

pub fn configure_runtime() {
    #[cfg(target_os = "windows")]
    {
        let Some(exe_dir) = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
        else {
            return;
        };

        let gstreamer_dir = exe_dir.join("gstreamer");
        let plugin_dir = gstreamer_dir.join("lib").join("gstreamer-1.0");
        let bin_dir = gstreamer_dir.join("bin");

        if plugin_dir.is_dir() {
            let plugin_path = plugin_dir.to_string_lossy().into_owned();
            let bin_path = bin_dir.to_string_lossy();
            let current_path = std::env::var_os("PATH").unwrap_or_default();
            let path = std::env::join_paths(
                std::iter::once(bin_path.into_owned().into())
                    .chain(std::env::split_paths(&current_path)),
            )
            .expect("Failed to construct the Windows runtime PATH");

            // Environment variables must be set before GStreamer is initialized.
            unsafe {
                std::env::set_var("PATH", path);
                std::env::set_var("GST_PLUGIN_PATH_1_0", &plugin_path);
                std::env::set_var("GST_PLUGIN_SYSTEM_PATH_1_0", &plugin_path);
            }
        }
    }
}

pub fn read_model_contents() -> Result<Vec<std::path::PathBuf>, io::Error> {
    let dir_path = model_directory();

    let entries = fs::read_dir(&dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let onnx_files: Vec<_> = entries
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "onnx"))
        .collect();

    Ok(onnx_files)
}

fn model_directory() -> std::path::PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let installed_models = exe_dir.join("models");
            if installed_models.is_dir() {
                return installed_models;
            }
        }
    }

    Path::new("assets").join("models")
}

pub fn initialize_model<'a>(model_path: &str) -> Result<Session, Box<dyn Error>> {
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .with_intra_threads(1)?
        .commit_from_file(model_path)?;
    Ok(session)
}

// Spawn a thread for model inference
pub fn inference_handler(
    model_path: String,
    norm: Normalization,
    model_rx: mpsc::Receiver<Frame>,
    detections: SharedDetections,
    shutdown: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("model_thread".into())
        .spawn(move || {
            eprintln!("Loading ONNX model: {}", model_path);
            let mut session =
                match initialize_model(&model_path) {
                    Ok(session) => session,
                    Err(err) => {
                        eprintln!("Failed to initialize YOLOv8 ONNX model '{}': {err}", model_path);
                        return;
                    }
                };
            let normalize = norm.get_lambda();

            let (input0_shape, channels) = {
                let meta = session
                    .metadata()
                    .expect("Failed to extract model metadata");

                let input0_shape: Vec<i32> = serde_json::from_str(
                    &meta
                        .custom("imgsz")
                        .expect("Model metadata is missing required 'imgsz' entry"),
                )
                .expect("Failed to parse imgsz metadata as JSON");

                let channels: i32 = meta
                    .custom("channels")
                    .expect("Model metadata is missing required 'channels' entry")
                    .parse()
                    .expect("Model metadata 'channels' must be a valid integer");

                (input0_shape, channels)
            };

            let height = input0_shape[0];
            let width = input0_shape[1];

            // Check model input/output compatibility
            assert_eq!(
                [1, channels, height, width],
                [1, 3, 640, 640],
                "Unexpected model input shape. Expected [1, 3, 640, 640], got {:?}",
                input0_shape
            );

            loop {
                if shutdown.load(Ordering::Relaxed) {
                    break;
                }

                match model_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(frame) => {
                        // YOLOv8n input takes CHW format.
                        let mut data = Array::zeros((
                            1usize,
                            channels as usize,
                            height as usize,
                            width as usize,
                        ));
                        let hw = frame.width * frame.height;
                        let out = data.as_slice_mut().expect("Failed to get mutable slice");
                        // Perform unsigned normalization
                        for (i, rgb) in frame.pixels.chunks_exact(3).enumerate() {
                            out[i] = normalize(rgb[0] as f32);
                            out[hw + i] = normalize(rgb[1] as f32);
                            out[2 * hw + i] = normalize(rgb[2] as f32);
                        }
                        let input_tensor = match TensorRef::from_array_view(&data) {
                            Ok(tensor) => tensor,
                            Err(err) => {
                                eprintln!("Failed to create ONNX input tensor: {err}");
                                break;
                            }
                        };
                        let outputs = match session.run(inputs!["images" => input_tensor]) {
                            Ok(outputs) => outputs,
                            Err(err) => {
                                eprintln!("ONNX model inference execution failed for '{}': {err}", model_path);
                                break;
                            }
                        };

                        let mut out = match outputs["output0"].try_extract_array::<f32>() {
                            Ok(array) => array.index_axis(ndarray::Axis(0), 0).to_owned(),
                            Err(err) => {
                                eprintln!("Failed to read model output 'output0' from '{}': {err}", model_path);
                                break;
                            }
                        };

                        if out.shape() != [300, 6] {
                            eprintln!(
                                "Unexpected ONNX model output shape for '{}': {:?}; expected [300, 6]",
                                model_path,
                                out.shape()
                            );
                            break;
                        }

                        // Normalize detections to [0, 1] for overlay drawing.
                        let scale = Array1::from(vec![
                            1.0 / frame.width as f32,
                            1.0 / frame.height as f32,
                            1.0 / frame.width as f32,
                            1.0 / frame.height as f32,
                            1.0,
                            1.0,
                        ]);
                        out *= &scale;

                        let mut shared = detections
                            .lock()
                            .expect("Failed to lock global detections placeholder");
                        shared.assign(&out);
                        drop(shared);
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                }
            }
            println!("model thread exiting");
        })
        .expect("Failed to spawn model inference thread")
}

pub fn start_model(
    model: &str,
    shutdown: Arc<AtomicBool>,
    model_rx: mpsc::Receiver<Frame>,
    detections: SharedDetections,
    norm: Normalization,
) -> Result<std::thread::JoinHandle<()>, Box<dyn Error>> {
    let model_path = model_directory().join(model);

    let model_thread = inference_handler(
        model_path.to_string_lossy().into_owned(),
        norm,
        model_rx,
        detections,
        shutdown,
    );

    Ok(model_thread)
}
