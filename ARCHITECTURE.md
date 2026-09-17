# VisionOTG Architecture

VisionOTG is a native Rust FPV camera application built around the
`gstreamer-cairo` pipeline model. GStreamer captures and displays the camera
stream, ONNX Runtime performs YOLO inference, and Cairo draws the detections
over the live video.

## Repository Structure

```text
.
├── Cargo.toml              Rust package and target-specific dependencies
├── .cargo/config.toml      Cargo linker configuration
├── src/
│   ├── votg.rs             Binary entry point and application lifecycle
│   ├── camera.rs           GStreamer pipeline and camera worker
│   ├── model.rs            ONNX Runtime session and inference worker
│   ├── draw.rs             Cairo detection overlay
│   └── menu.rs             Startup menu/UI when present in the checkout
├── assets/
│   ├── models/             ONNX model files
│   └── images/             Documentation images
├── packaging/              Linux and Windows packaging resources
└── target/                 Cargo build output (generated)
```

The exact contents of packaging and UI files may vary between branches. The
runtime path is defined by `src/votg.rs`, `src/camera.rs`, `src/model.rs`, and
`src/draw.rs`.

## Runtime Data Flow

```text
Camera source
		│
		▼
GStreamer pipeline
		│
		├── tee -> queue -> cairooverlay -> videoconvert -> autovideosink
		│                                      ▲
		│                                      │ Cairo draw callback reads detections
		│
		└── tee -> queue -> videoscale -> 640x640 RGB -> appsink
																							 │
																							 ▼
																	bounded sync_channel (capacity 1)
																							 │
																							 ▼
																			camera worker thread
																							 │ Frame { width, height, pixels }
																							 ▼
																			model worker thread
																	preprocess -> ort session.run()
																							 │
																							 ▼
																	Arc<Mutex<Array3<f32>>>
```

### Main thread

`votg.rs` parses the command-line arguments, initializes GStreamer, creates
the pipeline, installs the Cairo draw callback, watches the GStreamer bus,
and performs shutdown and thread joining.

### Camera worker

`camera.rs` pulls samples from the `appsink`, reads the RGB buffer into an
owned `Vec<u8>`, and sends a `Frame` through a bounded `mpsc::SyncSender`.
`try_send` deliberately drops a frame when inference is busy so latency does
not grow without limit.

The camera source is selected at compile time:

| Platform | Source | Camera argument |
| --- | --- | --- |
| Linux | `v4l2src` | Device path such as `/dev/video0` |
| Windows | `mfvideosrc` | Numeric device index such as `0` |
| macOS | `avfvideosrc` | Numeric device index |

### Inference worker

`model.rs` creates the ONNX Runtime `Session` inside the model thread. The
session remains owned by that thread; it is never moved through the channel
or shared with the UI. The worker converts the RGB frame to a 640x640 CHW
tensor, applies the selected normalization mode, runs the model, and writes
normalized detections to shared state.

The expected model metadata includes `imgsz` and `channels`. The current
implementation expects a three-channel 640x640 input and reads detections
from the `output0` output.

### Cairo overlay

`draw.rs` reads the latest detection array from `Arc<Mutex<_>>` during the
`cairooverlay` draw callback. It converts normalized boxes into display
coordinates and draws class labels and colored rectangles over the camera
view. The callback only reads inference results; it does not run inference.

## Shutdown

Ctrl+C sets the shared shutdown flag. The application then sets the GStreamer
pipeline to `Null`, waits for the camera and model workers to exit, and joins
both threads before returning.

## Design Rationale

* A bounded channel prevents an inference backlog from increasing display
	latency.
* Owned frame bytes allow the GStreamer buffer mapping to end before the
	inference thread uses the frame.
* Keeping the ONNX Runtime session on one thread avoids requiring the session
	to cross Rust thread boundaries.
* Sharing only the latest detection result keeps drawing responsive while
	inference continues independently.
