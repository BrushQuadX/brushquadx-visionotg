use gstreamer::prelude::*;
use std::error::Error;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
};

#[derive(Clone)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGB pixel data
}

// -----------------------------------------------------
// PIPELINE: camera -> videoconvert -> tee -> overlay + AI
// -----------------------------------------------------
pub fn create_pipeline(camera: &str) -> Result<gstreamer::Pipeline, Box<dyn Error>> {
    // Choose camera source based on OS
    let (camera_src, camera_index, source_caps) = {
        #[cfg(target_os = "linux")]
        {
            (
                "v4l2src",
                format!("device={}", camera),
                "! video/x-raw,width=640,height=480,framerate=30/1",
            )
        }
        #[cfg(target_os = "macos")]
        {
            ("avfvideosrc", format!("device-index={}", camera), "")
        }
        #[cfg(target_os = "windows")]
        {
            ("mfvideosrc", format!("device-index={}", camera), "")
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            ("autovideosrc", format!("device-index={}", camera), "")
        }
    };

    let pipeline_str: String = format!(
        "{} {} {} \
        ! videoconvert \
        ! tee name=t \
        t. ! queue \
            ! video/x-raw,format=RGB \
            ! appsink name=display_sink emit-signals=true sync=false max-buffers=1 drop=true \
        t. ! queue \
            ! videoscale \
            ! videoconvert \
            ! video/x-raw,width=640,height=640,format=RGB \
            ! appsink name=model_sink emit-signals=true sync=false max-buffers=1 drop=true",
        camera_src,
        camera_index,
        source_caps
    );

    let pipeline = gstreamer::parse::launch(&pipeline_str)?
        .dynamic_cast::<gstreamer::Pipeline>()
        .expect("Failed to launch GStreamer pipeline");

    Ok(pipeline)
}

fn sample_to_frame(sample: &gstreamer::Sample) -> Option<Frame> {
    // Get video metadata
    let caps = sample
        .caps()
        .expect("Failed to get sample caps from the appsink");

    let info = gstreamer_video::VideoInfo::from_caps(caps).expect("Failed to parse VideoInfo");
    let width = info.width() as usize;
    let height = info.height() as usize;

    // Extract the buffer payload from the pulled sample
    let buffer = sample.buffer()?;
    let map = buffer.map_readable().ok()?;
    let pixels = map.as_slice().to_vec();

    Some(Frame {
        width,
        height,
        pixels,
    })
}

// Spawn a thread to handle appsink input samples for model inference
fn model_handler(
    pipeline: &gstreamer::Pipeline,
    model_tx: mpsc::SyncSender<Frame>,
    shutdown: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    // Use for AI detections: Extract the appsink elements by its string name
    let appsink = pipeline
        .by_name("model_sink")
        .expect("GStreamer element model_sink was not found")
        .dynamic_cast::<gstreamer_app::AppSink>()
        .expect("Failed to cast pipeline to AppSink");

    // Spawn a background thread to safely block and pull samples
    std::thread::Builder::new()
        .name("camera_thread".into())
        .spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                // pull_sample() blocks until a sample is ready or EOS occurs
                match appsink.pull_sample() {
                    Ok(sample) => {
                        if let Some(frame) = sample_to_frame(&sample) {
                            let _ = model_tx.try_send(frame);
                        }
                    }
                    Err(err) => {
                        if !shutdown.load(Ordering::Relaxed) {
                            eprintln!("Appsink stopped: {}", err);
                        }
                        break;
                    }
                }
            }
            println!("Camera thread exiting");
        })
        .expect("Failed to spawn camera thread")
}

// Spawn a thread to handle appsink samples for display at native camera resolution
fn display_handler(
    pipeline: &gstreamer::Pipeline,
    display_tx: mpsc::SyncSender<Frame>,
    shutdown: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    // Use for AI detections: Extract the appsink elements by its string name
    let appsink = pipeline
        .by_name("display_sink")
        .expect("GStreamer element display_sink was not found")
        .dynamic_cast::<gstreamer_app::AppSink>()
        .expect("Failed to cast pipeline to AppSink");

    // Spawn a background thread to safely block and pull samples
    std::thread::Builder::new()
        .name("display_thread".into())
        .spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                // pull_sample() blocks until a sample is ready or EOS occurs
                match appsink.pull_sample() {
                    Ok(sample) => {
                        if let Some(frame) = sample_to_frame(&sample) {
                            let _ = display_tx.try_send(frame);
                        }
                    }
                    Err(err) => {
                        if !shutdown.load(Ordering::Relaxed) {
                            eprintln!("Appsink stopped: {}", err);
                        }
                        break;
                    }
                }
            }
            println!("Display thread exiting");
        })
        .expect("Failed to spawn display thread")
}

pub fn start_camera(
    camera: &str,
    shutdown: Arc<AtomicBool>,
) -> Result<
    (
        gstreamer::Pipeline,
        mpsc::Receiver<Frame>,
        mpsc::Receiver<Frame>,
        std::thread::JoinHandle<()>,
        std::thread::JoinHandle<()>,
    ),
    Box<dyn Error>,
> {
    // Initialize GStreamer
    gstreamer::init()?;

    // Define GStreamer pipeline: capture -> process -> overlay -> display
    let pipeline = create_pipeline(camera)?;

    let (display_tx, display_rx) = mpsc::sync_channel::<Frame>(1);

    let (model_tx, model_rx) = mpsc::sync_channel::<Frame>(1);

    let camera_thread = model_handler(&pipeline, model_tx, shutdown.clone());

    let display_thread = display_handler(&pipeline, display_tx, shutdown.clone());

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

    Ok((
        pipeline,
        display_rx,
        model_rx,
        camera_thread,
        display_thread,
    ))
}

// Shutdown the pipeline on exit
pub fn cleanup(pipeline: &gstreamer::Pipeline) {
    println!("Stopping GStreamer Pipeline...");
    let result = pipeline
        .set_state(gstreamer::State::Null)
        .expect("Failed to set pipeline state to Null");
    println!("Pipeline state changed: {:?}", result);
}
