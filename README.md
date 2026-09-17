# BrushQuadX VisionOTG

VisionOTG is a native Rust FPV camera application with real-time YOLO object
detection. It captures video through GStreamer, runs inference with ONNX
Runtime, and draws detection boxes with Cairo over the live camera stream.

## Features

* Live camera preview with a GStreamer pipeline.
* Linux camera capture through `v4l2src` and Windows capture through
	`mfvideosrc`.
* YOLO ONNX inference with configurable input normalization.
* Cairo detection overlay rendered in the video pipeline.
* Separate camera and model threads with a bounded frame queue.
* Linux ARM64 deployment support for Raspberry Pi.

## Repository Layout

```text
src/votg.rs       Application entry point and lifecycle
src/camera.rs     GStreamer capture, appsink, and camera worker
src/model.rs      ONNX Runtime session and inference worker
src/draw.rs       Cairo detection overlay
assets/models/    ONNX model files
packaging/        Linux and Windows packaging resources
```

## Requirements

* Rust and Cargo
* GStreamer runtime and development packages
* Cairo development libraries
* A compatible YOLO ONNX model
* ONNX Runtime, downloaded by `ort` or provided through `ORT_LIB_LOCATION`

See [BUILD.md](BUILD.md) for Windows, Raspberry Pi, cross-compilation, ONNX
Runtime, and camera troubleshooting instructions.

## Quick Start

### Linux or Raspberry Pi

Install the dependencies described in [BUILD.md](BUILD.md), then run:

```bash
cargo run --release -- \
		--camera /dev/video0 \
		--model assets/models/yolov8n.onnx
```

The default camera is `/dev/video0` and the default model name is
`yolov8n.onnx`. Pass an explicit path when the model is stored under
`assets/models/` or another directory.

### Windows

After installing Rust and GStreamer, run PowerShell from the repository root:

```powershell
cargo run --release -- --camera 0 --model assets/models/yolov8n.onnx
```

The Windows camera argument is a Media Foundation device index. Use the index
accepted by `mfvideosrc` on the machine.

## Command-Line Options

```text
--camera <CAMERA>    Camera index or device path
--model <MODEL>      YOLO ONNX model path
--norm <NORM>        unsigned, signed, or raw input normalization
```

For example:

```bash
cargo run --release -- \
		--camera /dev/video0 \
		--model assets/models/yolov8n.onnx \
		--norm unsigned
```

## Testing the Camera

Use GStreamer directly to determine whether the camera and plugins work:

```bash
gst-launch-1.0 v4l2src device=/dev/video0 ! videoconvert ! autovideosink
```

On Windows:

```powershell
gst-launch-1.0 ksvideosrc device-index=1 ! videoconvert ! autovideosink
```

## Architecture

The camera worker copies RGB samples from GStreamer into a bounded channel.
The inference worker owns the ONNX Runtime session and publishes the latest
detections through shared state. The Cairo callback reads those detections and
draws the overlay. This prevents a slow inference pass from blocking video
capture or allowing an unbounded frame backlog.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the complete data flow and module
responsibilities.

## Packaging

Build and installer resources are kept in `packaging/linux/` and
`packaging/windows/`. A release package must include the `votg` executable,
the required GStreamer runtime, the ONNX Runtime shared library when using
system loading, and the selected ONNX model.

## License

This project is licensed under [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/).
