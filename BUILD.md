# Build and Test Guide

This project is based on a GStreamer/Cairo video pipeline and builds the
`votg` binary with Cargo. It uses GStreamer for capture and display, Cairo for
the overlay, and ONNX Runtime through the `ort` crate.

## Prerequisites

### Windows

Install:

* Visual Studio Build Tools with the Desktop C++ workload
* Rust via [rustup](https://rustup.rs/)
* GStreamer runtime and development packages for the MSVC toolchain
* ONNX models from `assets/models/` or another compatible model file

Confirm that `cargo`, `gst-launch-1.0`, and the GStreamer DLL directory are on
`PATH`. Git LFS is needed if large assets such as the demo GIF are tracked by
LFS.

Build and run from the repository root:

```powershell
cargo build --release
cargo run --release -- --camera 0 --model assets/models/yolov8n.onnx
```

If the model is copied beside the executable, the default `yolov8n.onnx`
argument can be used instead.

### Raspberry Pi

Install the native development dependencies:

```bash
sudo apt update
sudo apt install -y \
	build-essential \
	cmake \
	git \
	pkg-config \
	python3 \
	python3-full \
	python3-pip \
	libgstreamer1.0-dev \
	libgstreamer-plugins-base1.0-dev \
	libglib2.0-dev \
	libcairo2-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Install the runtime GStreamer plugins used by the test pipelines and the
application:

```bash
sudo apt install -y \
	gstreamer1.0-tools \
	gstreamer1.0-plugins-base \
	gstreamer1.0-plugins-good \
	gstreamer1.0-plugins-bad \
	gstreamer1.0-libav
```

## ONNX Runtime on ARM64

The `aarch64-unknown-linux-gnu` dependency uses `ort` with the `load-dynamic`
feature. Set `ORT_STRATEGY=system` and point `ORT_LIB_LOCATION` at a directory
containing the built `libonnxruntime.so` before running or linking the ARM
binary.

Build ONNX Runtime from source on a Raspberry Pi when a compatible prebuilt
library is unavailable:

```bash
git clone --recursive https://github.com/microsoft/onnxruntime.git
cd onnxruntime
python3 -m venv ~/cmake-env
source ~/cmake-env/bin/activate
python -m pip install --upgrade pip cmake
./build.sh \
	--config Release \
	--build_shared_lib \
	--parallel 4 \
	--skip_tests
```

Configure the application to use that build:

```bash
export ORT_STRATEGY=system
export ORT_LIB_LOCATION="$HOME/Repositories/onnxruntime/build/Linux/Release"
export LD_LIBRARY_PATH="$ORT_LIB_LOCATION:$LD_LIBRARY_PATH"
```

`ORT_LIB_LOCATION` must contain the shared ONNX Runtime library. Adjust the
path if the ONNX Runtime repository is elsewhere.

## Native Raspberry Pi Build

From the VisionOTG repository root:

```bash
export ORT_STRATEGY=system
export ORT_LIB_LOCATION="$HOME/Repositories/onnxruntime/build/Linux/Release"
export LD_LIBRARY_PATH="$ORT_LIB_LOCATION:$LD_LIBRARY_PATH"
cargo clean
cargo build --release
```

Run the program with a camera device and model:

```bash
cargo run --release -- \
	--camera /dev/video0 \
	--model assets/models/yolov8n.onnx
```

The supported normalization values are `unsigned`, `signed`, and `raw`:

```bash
cargo run --release -- --camera /dev/video0 --model yolov8n.onnx --norm unsigned
```

## Cross Compilation from Linux

Install the ARM64 cross compiler and target development packages on the build
machine. The package names below are for Debian/Ubuntu systems configured for
multiarch:

```bash
sudo rm -rf /var/lib/apt/lists/*
sudo dpkg --add-architecture arm64
sudo apt update
sudo apt install -y \
	gcc-aarch64-linux-gnu \
	pkg-config \
	pkg-config:arm64 \
	libssl-dev \
	libssl-dev:arm64 \
	libglib2.0-dev:arm64 \
	libcairo2-dev:arm64 \
	libgstreamer1.0-dev:arm64 \
	libgstreamer-plugins-base1.0-dev:arm64

rustup target add aarch64-unknown-linux-gnu
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig:/usr/share/pkgconfig
export ORT_STRATEGY=download
```

Add the ARM linker to `.cargo/config.toml` if it is not already present:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

Then build:

```bash
cargo clean
cargo build --release --target aarch64-unknown-linux-gnu
```

When using a locally built ONNX Runtime instead of the download strategy,
replace `ORT_STRATEGY=download` with:

```bash
export ORT_STRATEGY=system
export ORT_LIB_LOCATION="$HOME/Repositories/onnxruntime/build/Linux/Release"
```

The target binary is written to
`target/aarch64-unknown-linux-gnu/release/votg`.

## Camera Discovery

List available Linux cameras and supported formats:

```bash
v4l2-ctl --list-devices
v4l2-ctl --list-formats-ext -d /dev/video0
```

Use the device path that exposes a format supported by the GStreamer pipeline.

## GStreamer Smoke Tests

Test the camera independently of Rust before debugging the application:

```bash
gst-launch-1.0 v4l2src device=/dev/video1 ! videoconvert ! autovideosink
gst-launch-1.0 v4l2src device=/dev/video0 ! videoconvert ! \
	video/x-raw,format=BGR,width=640,height=480,framerate=30/1 ! \
	textoverlay text="Camera Stream" font-desc="Monospace, 16" ! \
	autovideosink
```

On Windows, test a Media Foundation camera with:

```powershell
gst-launch-1.0 ksvideosrc device-index=1 ! videoconvert ! autovideosink
```

If a pipeline fails, check that the relevant GStreamer source, sink, and
plugin packages are installed before changing Rust code.

## Packaging

Packaging resources live under `packaging/linux/` and `packaging/windows/`.
The generated Cargo binary and the runtime GStreamer and ONNX Runtime files
must be included in the final package. Keep the model path used by the
launcher or installer consistent with the `--model` argument passed to
`votg`.
