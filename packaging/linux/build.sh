#!/usr/bin/env bash
set -e

# Build VisionOTG Package on aarch64

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ -f "$HOME/.cargo/env" ]; then
  source "$HOME/.cargo/env"
fi

DO_SETUP=false
for arg in "$@"; do
  case "$arg" in
    --setup)
      DO_SETUP=true
      ;;
    --help|-h)
      echo "Usage: $0 [--setup]"
      echo
      echo "  --setup    install dependencies, clone ONNX Runtime, and build the runtime"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg"
      echo "Usage: $0 [--setup]"
      exit 1
      ;;
  esac
done

if [ "$DO_SETUP" = true ]; then
  # Update package list
  sudo apt update

  # Install Rust if needed
  if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  else
    echo "Rust already installed; skipping rustup install"
  fi

  # Install GStreamer packages
  sudo apt install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-libav \
    libglib2.0-dev \
    libcairo2-dev \
    pkg-config

  # Install Build and Python packages
  sudo apt install -y \
    build-essential \
    cmake \
    python3 \
    python3-pip \
    python3-venv \
    python3-full \
    git \
    libcurl4-openssl-dev \
    libssl-dev \
    libsqlite3-dev

  # Clone or update ONNX Runtime repository
  if [ -d "$SCRIPT_DIR/onnxruntime/.git" ]; then
    echo "Using existing onnxruntime checkout"
    cd "$SCRIPT_DIR/onnxruntime"
    git fetch --all --tags --prune
    git pull --ff-only || true
  else
    if [ -e "$SCRIPT_DIR/onnxruntime" ]; then
      echo "Found existing onnxruntime directory but it is not a git checkout. Remove it and rerun --setup."
      exit 1
    fi
    git clone --recursive --branch v1.28.0 https://github.com/microsoft/onnxruntime.git
    cd "$SCRIPT_DIR/onnxruntime"
  fi

  # Create build environment
  python3 -m venv "$PWD/cmake-env"
  source "$PWD/cmake-env/bin/activate"
  pip install --upgrade pip setuptools wheel cmake
  pip install -r requirements.txt

  # Build ONNX Runtime package
  rm -rf ~/.cache/ort.pyke.io
  export CXXFLAGS="${CXXFLAGS:-} -Wno-error=calloc-transposed-args"

  ./build.sh \
    --config Release \
    --build_shared_lib \
    --parallel 4 \
    --skip_tests \
    --cmake_extra_defines \
      onnxruntime_BUILD_UNIT_TESTS=OFF \
      onnxruntime_USE_TELEMETRY=OFF

  # "$PWD/cmake-env/bin/cmake" \
  #   --build build/Linux/Release \
  #   --target onnxruntime \
  #   -j2

  cd "$SCRIPT_DIR"
fi

# Build VisionOTG
export ORT_STRATEGY=system
export ORT_LIB_LOCATION="$SCRIPT_DIR/onnxruntime/build/Linux/Release"
export LD_LIBRARY_PATH="$SCRIPT_DIR/onnxruntime/build/Linux/Release:$LD_LIBRARY_PATH"

cargo build --release
