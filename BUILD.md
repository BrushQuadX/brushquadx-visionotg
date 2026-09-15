
This document describes the build files to setup the VisionOTG program across different platforms. The program is supported in both Windows and Raspberry Pi machines.

## Build Windows Package

The following project installation files are described below.

1. [download_runtime.ps1](./packaging/windows/download_runtime.ps1) downloads the runtime dependencies:
    * GStreamer runtime and GStreamer development files
    * ONNX Runtime DLL

2. [build_windows.ps1](./packaging/windows/build_windows.ps1) handles the Windows packaging:
    * sets the GStreamer environment variables
    * runs `cargo build --locked --release`
    * copies the built executable and runtime files into the packaging folder
    * copies the model/config assets into that same folder
    * invokes Inno Setup to build the installer

3. [installer.iss](./packaging/windows/installer.iss) defines the Windows installer:
    * install directory under Program Files
    * app shortcut on desktop/start menu
    * install of all bundled files
    * execution of the app after install

4. [windows.yml](./.github/workflows/windows.yml) is the GitHub Actions version of the same pipeline. It runs on Windows and uploads the final installer as an artifact or GitHub release. It produces three main outputs:
    * target/windows-runtime
        * downloaded runtimes
        * downloaded archives
        * extracted GStreamer/ONNX files
    * target/windows-bundle - a portable app folder containing:
        * votg.exe
        * onnxruntime.dll
        * gstreamer/
        * models/
    * target/windows-installer/VisionOTGSetup.exe
        * the final MSI-style installer built by Inno Setup

The key packaging flow:

build_windows.ps1 -> copies release binary + runtime + assets -> installer.iss -> creates installer executable

### Reproduce on a Fresh Windows PC

The following commands must be run on your PC with administrator permissions.

1. Install Rust:

    ```shell
    > winget install --id Rustlang.Rustup -e
    > rustup toolchain install stable
    > rustup default stable
    ```

2. Install Chocolatey install snippet with administrator permissions:

    ```shell
    > Set-ExecutionPolicy Bypass -Scope Process -Force
    > [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    > iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    ```

3. Install Inno Setup 6:

    ```shell
    > choco install innosetup --no-progress --yes
    ```

4. Install pkg-config compatibility for GStreamer. The `pkgconfiglite` package is typically available via Chocolatey; `winget` does not currently provide it in the default sources:

    ```powershell
    > choco install pkgconfiglite --no-progress --yes
    ```

5. Clone and enter the repo:

    ```shell
    > git clone https://github.com/BrushQuadX/brushquadx-visionotg.git
    > cd brushquadx-visionotg
    ```

6. Build the bundle and installer:

    ```shell
    > powershell -ExecutionPolicy Bypass -File .\packaging\windows\build_windows.ps1
    ```

    The script does all of the following automatically:
    * downloads and prepares the GStreamer runtime and development files
    * downloads the ONNX Runtime DLL
    * sets GStreamer env vars
    * builds release binary `votg.exe`
    * creates target/windows-bundle
    * copies `votg.exe`, `onnxruntime.dll`, `gstreamer/`, and `models/`
    * invokes Inno Setup
    * creates the final installer at: `target/windows-installer/VisionOTGSetup.exe`

    The installer version comes from the Git tag in GitHub Actions, with the leading `v` removed (for example, `v1.2.3` becomes `1.2.3`). Local and manually triggered builds default to version `1.0.0`; you can override it with `-Version`, for example `-Version 1.2.3`.

    To prepare or troubleshoot the runtime separately, you can run the downloader directly:

    ```shell
    > powershell -ExecutionPolicy Bypass -File .\packaging\windows\download_runtime.ps1
    ```

## Build Raspberry Pi Package

The Raspberry Pi package targets a 64-bit Raspberry Pi OS installation on Raspberry Pi 5 (`aarch64`). Run the commands below on the Pi, or on an equivalent native ARM64 Linux host. The setup step builds ONNX Runtime from source, so the first build can take a while.

1. Clone and enter the repository:

    ```shell
    $ git clone https://github.com/BrushQuadX/brushquadx-visionotg.git
    $ cd brushquadx-visionotg
    ```

2. Build the Debian package and install it:

    ```shell
    $ chmod +x packaging/linux/*.sh packaging/linux/visionotg-launcher
    $ ./packaging/linux/build_deb.sh --setup
    $ sudo apt install ./target/linux-packages/visionotg_1.0.0_arm64.deb
    ```

    The package installs the application under `/opt/visionotg`, the `votg` launcher under `/usr/bin`, and a desktop entry in `/usr/share/applications`. It includes the ARM64 ONNX Runtime library and model files; GStreamer and graphical runtime libraries are installed through Debian dependencies.

3. For later package rebuilds, skip dependency setup and optionally set the version:

    ```shell
    $ ./packaging/linux/build_deb.sh --skip-build --version 1.0.1
    ```

    `--skip-build` does not install dependencies, build ONNX Runtime, or compile the Rust application. Use it only when these files already exist:

    * `target/release/votg`
    * `onnxruntime/build/Linux/Release/libonnxruntime.so`

    Available `build_deb.sh` options:

    * `--setup` installs the native build dependencies, reuses or clones the top-level `onnxruntime` checkout, builds ONNX Runtime, and compiles VisionOTG. Use this for the first build or after removing the build outputs.
    * `--skip-build` packages the existing release executable and ONNX Runtime library without rebuilding them. This is useful when only the package metadata, version, or output location has changed.
    * `--version VERSION` overrides the Debian package version. Without it, the version is read from `Cargo.toml`.
    * `--output DIRECTORY` writes the `.deb` file to a custom directory instead of `target/linux-packages`.
    * `--help` prints the command usage and available options.

    For example, create a package with a custom version and output directory:

    ```shell
    $ ./packaging/linux/build_deb.sh --skip-build --version 1.0.1 --output ~/visionotg-packages
    ```

    Install or upgrade the resulting package with `sudo apt install ./target/linux-packages/visionotg_1.0.1_arm64.deb`.


## Convert Model

The program uses a model from [Ultralytics](https://github.com/ultralytics) that has been converted into ONNX format with embedded NMS. The instructions for [converting the model](./packaging/model.sh) can be found below.

1. Fetch a YOLOv8n model from Ultralytics.

    ```shell
    wget https://github.com/ultralytics/assets/releases/download/v8.4.0/yolov8n.pt
    ```

2. Build and activate Python environment to contain Ultralytics repository.

    ```shell
    python3 -m venv "$PWD/model-env"
    source "$PWD/model-env/bin/activate"
    ```

3. Install Ultralytics inside the environment (tested using ultralytics-8.4.104).

    ```shell
    pip install ultralytics
    ```

4. Export the YOLO model to ONNX with embedded NMS.

    ```shell
    yolo export model=yolov8n.pt format=onnx nms=True
    ```

## Software Development

Format the rust program with `cargo fmt`.
