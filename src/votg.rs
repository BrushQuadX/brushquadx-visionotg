mod camera;
mod draw;
mod model;
mod menu;

use eframe::egui;
use gstreamer::prelude::*;
use ndarray::Array3;
use std::error::Error;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::fs;
use std::io;
use std::path::Path;


struct Settings {
    // Camera index or v4l2src camera to run
    camera: String,
    // The chosen model (e.g., yolov8n.onnx)
    model: String,
    // List of available ONNX model files in the assets/models directory
    onnx_files: Vec<std::path::PathBuf>,
    // Input normalization method
    norm: model::Normalization,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera: "0".to_owned(),
            model: "yolov8n.onnx".to_owned(),
            onnx_files: Vec::new(),
            norm: model::Normalization::Unsigned,
        }
    }
}

fn configure_runtime() {
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

fn read_model_contents() -> Result<Vec<std::path::PathBuf>, io::Error> {
    // Point to the target directory
    let dir_path = "./assets/models";

    let entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let onnx_files: Vec<_> = entries
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "onnx"))
        .collect();

    Ok(onnx_files)
}

fn start_application(camera: &str, model: &str) -> Result<(), Box<dyn Error>> {
    let model_path = Path::new("./assets").join("models").join(model);

    configure_runtime();
    // Initialize GStreamer
    gstreamer::init()?;

    // Initialize detections placeholder
    let detections: model::SharedDetections =
        Arc::new(Mutex::new(Array3::<f32>::zeros((1, 300, 6))));

    // Define GStreamer pipeline: capture -> process -> overlay -> display
    let (pipeline, overlay) = camera::create_pipeline(camera)?;

    // Clone shared state into overlay callback
    let overlay_detections = detections.clone();

    overlay.connect("draw", false, move |args| {
        let cr: cairo::Context = args[1].get().expect("Failed to start Cairo context");
        draw::draw_overlay(&cr, &overlay_detections);
        None
    });

    let shutdown = Arc::new(AtomicBool::new(false));
    let (frame_tx, frame_rx) = mpsc::sync_channel::<camera::Frame>(1);

    {
        let shutdown = shutdown.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C received");
            shutdown.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");
    }

    let model_thread = model::inference_handler(
        model_path.to_string_lossy().into_owned(),
        model::Normalization::Unsigned,
        frame_rx,
        detections.clone(),
        shutdown.clone(),
    );
    let camera_thread = camera::appsink_handler(&pipeline, frame_tx, shutdown.clone());

    // Start input pipeline
    match pipeline.set_state(gstreamer::State::Playing) {
        Ok(_) => {}
        Err(err) => {
            panic!(
                "Failed to start GStreamer pipeline. Check if the camera '{}' exists: {}",
                camera, err
            );
        }
    }

    // -------------------------
    // Main loop (bus)
    // -------------------------
    let bus = pipeline
        .bus()
        .expect("Pipeline was initialized without bus");

    loop {
        if shutdown.load(Ordering::Relaxed) {
            break;
        }

        if let Some(msg) = bus.timed_pop(gstreamer::ClockTime::from_mseconds(100)) {
            match msg.view() {
                gstreamer::MessageView::Error(err) => {
                    eprintln!("Error: {:?}", err);
                    shutdown.store(true, Ordering::Relaxed);
                }
                gstreamer::MessageView::Eos(_) => {
                    shutdown.store(true, Ordering::Relaxed);
                }
                _ => {}
            }
        }
    }

    // Cleanup pipeline on exit
    println!("Stopping GStreamer Pipeline...");
    camera::cleanup(&pipeline);

    // Join threads
    println!("Joining camera thread");
    camera_thread.join().expect("Failed to join camera thread");

    println!("Joining model thread");
    model_thread.join().expect("Failed to join model thread");

    println!("Cleaning up...");
    drop(pipeline);
    drop(overlay);

    Ok(())

}

fn main() -> eframe::Result {
    // Initialization menu window
    let onnx_files = match read_model_contents() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error reading model contents: {}", e);
            Vec::new()
        }
    };
    println!("Found ONNX files: {:?}", onnx_files);

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "BrushQuadX VisionOTG",
        options,
        Box::new(move |cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(Settings {
                onnx_files,
                ..Default::default()
            }))
        }),
    )
}
