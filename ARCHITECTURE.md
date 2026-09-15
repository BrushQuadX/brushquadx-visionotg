# VisionOTG Architecture

## System Overview

VisionOTG is a native Rust desktop application built around four runtime concerns:

1. `eframe` and `egui` own the window, startup controls, camera texture, and user interaction.
2. GStreamer captures frames and branches the stream into display and inference paths.
3. An inference worker preprocesses frames and executes the ONNX model through `ort`.
4. The drawing layer reads the latest detections and paints normalized bounding boxes over the displayed frame.

The UI thread does not perform camera capture or model inference. Communication uses bounded channels and shared state so stale frames are dropped instead of allowing latency to grow indefinitely.

## Runtime Flow

```text
Application entry
	|
	v
Runtime configuration -> model discovery -> eframe window
	|
	v
Startup screen: camera + model + normalization
	|
	v
GStreamer pipeline
	|
	+--> display_sink -> display thread -> bounded channel -> egui texture
	|
	+--> model_sink   -> camera thread -> bounded channel -> model thread
												   |
												   v
									  ONNX preprocessing and inference
												   |
												   v
									  shared normalized detections
												   |
												   v
									  egui detection overlay
```

## Module Responsibilities

### `src/votg.rs`

The entry point configures platform runtime paths before GStreamer or ONNX Runtime is initialized, discovers available ONNX files, and creates the maximized `eframe` window. It injects the discovered model list into `App`.

### `src/app.rs`

`App` owns the UI state and the handles required to stop the runtime cleanly. The startup screen collects:

* Camera input: a Linux device path such as `/dev/video0`, or a Windows camera index.
* Model selection from the discovered ONNX files.
* Input normalization mode.

When the user starts the camera, `App` starts the GStreamer pipeline and model worker. During rendering it consumes the newest display frame, updates an egui texture, and draws the latest detections over the image. On exit it sets the shared shutdown flag, stops the pipeline, joins all workers, and clears runtime resources.

### `src/camera.rs`

The camera pipeline selects the platform source at compile time:

* Linux: `v4l2src device=/dev/videoN`.
* Windows: `mfvideosrc device-index=N`.
* macOS: `avfvideosrc device-index=N`.

After `videoconvert`, a `tee` creates two branches:

* Display branch: RGB frames at the camera's native resolution.
* Model branch: scaled RGB frames at `640 x 640`.

Each branch ends in an `appsink` with a one-frame synchronous queue and `drop=true`. This keeps the application focused on current video rather than accumulating delayed frames. The camera and display handlers pull samples on separate named threads and send them through bounded channels.

### `src/model.rs`

This module discovers `.onnx` files, loads the selected model, and owns the inference loop. The model loader checks metadata for the image size and channel count, then expects an input shape of `[1, 3, 640, 640]`.

For each model frame, the worker:

1. Converts interleaved RGB bytes to CHW tensor layout.
2. Applies the selected normalization function.
3. Runs the model with input name `images`.
4. Reads output `output0`.
5. Converts box coordinates to values normalized against the frame width and height.
6. Replaces the shared detection array used by the drawing code.

On `aarch64` Linux, `ort` uses dynamic loading. The Debian launcher sets `ORT_DYLIB_PATH` to the packaged `/opt/visionotg/lib/libonnxruntime.so`. On Windows, `configure_runtime` adds the bundled GStreamer paths before the first GStreamer initialization.

### `src/draw.rs`

The drawing module renders the application background and translates normalized model detections into coordinates within the current camera image. It does not own model or camera state; it receives the shared detection data from `App`.

## Concurrency and Shutdown

The runtime uses one shared `AtomicBool` shutdown flag and three worker handles:

* Camera worker pulls model-branch samples.
* Display worker pulls display-branch samples.
* Model worker performs inference.

Channels are bounded to one frame. `try_send` drops a frame when the consumer is busy, which bounds memory use and display latency. Detection data is held in an `Arc<Mutex<Array3<f32>>>`; the model worker replaces it after inference and the UI reads it while painting.

Shutdown first signals the workers, then transitions the GStreamer pipeline to `Null`, joins each thread, and resets the application state. This ordering releases blocking GStreamer sample pulls before waiting for their threads.

## Asset and Path Contract

During development, models are loaded from `assets/models`. In a deployed bundle, the application first checks a `models` directory beside the executable. The packaging formats preserve that contract:

* Windows bundle: `votg.exe` and `models/` share a directory.
* Debian package: `/opt/visionotg/votg` and `/opt/visionotg/models/` share a directory.

The Debian launcher additionally exports `ORT_DYLIB_PATH` and `LD_LIBRARY_PATH` for the packaged ONNX Runtime library.

## Build and Packaging Boundaries

`packaging/linux/build.sh` is the source build step. With `--setup`, it installs Linux development dependencies, reuses or clones the top-level `onnxruntime` checkout, builds the shared ONNX Runtime library, and runs `cargo build --locked --release`.

`packaging/linux/build_deb.sh` is the distribution step. It stages the release executable, ONNX Runtime library, models, launcher, desktop entry, and Debian metadata, then produces an `arm64` `.deb` with `dpkg-deb`.

The Windows script follows the same separation concept but uses downloaded GStreamer and ONNX Runtime files and creates an Inno Setup installer.

## Design Constraints

* The Linux packaging target is native `aarch64`; the scripts intentionally reject other host architectures.
* The model interface is currently fixed to three channels and `640 x 640` input.
* Bounded queues favor fresh frames and predictable latency over processing every frame.
* Platform runtime configuration must happen before GStreamer or ONNX Runtime initialization.
