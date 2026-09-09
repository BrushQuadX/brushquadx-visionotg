# Building and Packaging VisionOTG

This document describes the recommended workflow for building and distributing VisionOTG for both **Windows** and **Raspberry Pi (aarch64/Linux)**.

The goal is to keep the repository clean by storing only source code while automatically producing distributable installers through GitHub Actions.

---

# Overview

The build pipeline follows this workflow:

```text
GitHub Repository
        │
        ▼
Source Code
        │
        ▼
GitHub Actions (CI)
        │
        ▼
Install Build Dependencies
        │
        ▼
Build Rust Application
        │
        ▼
Download Runtime Dependencies
        │
        ▼
Package Application
        │
        ▼
Create Installer / Package
        │
        ▼
Upload Release Artifact
```

Only the source code is stored in the repository.

Third-party runtimes (GStreamer and ONNX Runtime) are downloaded during the build process.

---

# Repository Structure

A recommended project layout is:

```text
VisionOTG/

├── src/
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── assets/
│   ├── models/
│   │   └── yolov8n.onnx
│   └── config/
│       └── settings.json
│
├── packaging/
│   ├── windows/
│   │   ├── installer.iss
│   │   ├── build_windows.ps1
│   │   └── download_runtime.ps1
│   │
│   └── linux/
│       ├── build_deb.sh
│       ├── control
│       └── postinst
│
├── .github/
│   └── workflows/
│       ├── windows.yml
│       └── raspberrypi.yml
│
└── docs/
    └── steps_installer.md
```

Notice that **GStreamer** and **ONNX Runtime** are **not** committed into the repository.

---

# Windows Build Workflow

## Step 1 — Checkout Repository

GitHub Actions clones the latest repository.

```text
git clone ...
```

---

## Step 2 — Install Rust

Install the Rust toolchain.

```text
rustup toolchain install stable
```

---

## Step 3 — Build VisionOTG

Compile a release build.

```text
cargo build --release
```

Output:

```text
target/release/VisionOTG.exe
```

---

## Step 4 — Download Runtime Dependencies

Download:

- GStreamer MSVC Runtime
- ONNX Runtime (Windows x64)

These are downloaded automatically during CI.

Nothing is stored in Git.

---

## Step 5 — Create Bundle Directory

Create a temporary bundle directory.

```text
bundle/

├── VisionOTG.exe
├── onnxruntime.dll
├── gstreamer/
├── models/
└── config/
```

Copy:

- executable
- DLLs
- models
- configuration files

---

## Step 6 — Build Installer

Use **Inno Setup**.

The installer is described by

```text
installer.iss
```

This script specifies:

- application name
- version
- install location
- desktop shortcut
- files to copy
- uninstall information

The output becomes

```text
VisionOTGSetup.exe
```

---

## Step 7 — Upload Artifact

GitHub Actions uploads

```text
VisionOTGSetup.exe
```

as a build artifact or attaches it to a GitHub Release.

---

# Raspberry Pi (Linux ARM64)

Linux packaging differs from Windows.

Instead of creating an installer, Linux distributions use packages.

Ubuntu/Debian packages use

```text
.deb
```

files.

---

## Step 1 — Checkout Repository

Clone repository.

---

## Step 2 — Install Dependencies

Install build dependencies.

Example:

```bash
sudo apt update

sudo apt install \
    build-essential \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev
```

---

## Step 3 — Build Release

```bash
cargo build --release
```

Produces

```text
target/release/visionotg
```

---

## Step 4 — Prepare Package Directory

```text
package/

├── DEBIAN/
│   └── control
│
├── usr/
│   ├── bin/
│   │   └── visionotg
│   │
│   └── share/
│       └── visionotg/
│           ├── models/
│           ├── config/
│           └── libonnxruntime.so
```

Unlike Windows, Linux applications are typically installed into standard filesystem locations.

---

## Step 5 — Build Debian Package

Generate

```text
visionotg_3.0_arm64.deb
```

using

```bash
dpkg-deb --build package
```

---

## Step 6 — Upload Artifact

Upload

```text
visionotg_3.0_arm64.deb
```

to GitHub Releases.

---

# Windows Runtime Layout

After installation:

```text
Program Files/

└── VisionOTG/

    VisionOTG.exe

    onnxruntime.dll

    gstreamer/
        bin/
        lib/
        etc/
        share/

    models/

    config/
```

---

# Linux Runtime Layout

After installation:

```text
/usr/bin/visionotg

/usr/share/visionotg/

    models/

    config/

    libonnxruntime.so
```

---

# Important Differences

| Windows | Linux |
|----------|--------|
| Installer (.exe) | Debian package (.deb) |
| Inno Setup | dpkg-deb |
| Bundle GStreamer | Depend on system GStreamer packages |
| Bundle onnxruntime.dll | Bundle libonnxruntime.so (recommended) |
| Install to Program Files | Install under /usr |

---

# Packaging Scripts

## installer.iss

Extension:

```text
.iss
```

An Inno Setup Script describing how to generate the Windows installer.

Responsibilities:

- Copy files
- Create shortcuts
- Register uninstall information
- Choose installation directory

---

## build_windows.ps1

Extension:

```text
.ps1
```

A Windows PowerShell script.

Responsibilities:

- Build Rust
- Download runtime libraries
- Copy DLLs
- Create bundle
- Launch Inno Setup

Instead of manually typing many commands, packaging becomes:

```powershell
.\build_windows.ps1
```

---

## download_runtime.ps1

A helper script that downloads:

- GStreamer Runtime
- ONNX Runtime

This avoids storing hundreds of megabytes of binary files in Git.

---

## build_deb.sh

A Linux shell script.

Responsibilities:

- Build Rust
- Create Debian directory structure
- Copy application files
- Build the .deb package

Run with:

```bash
./build_deb.sh
```

---

# GitHub Actions

GitHub Actions automates the build process.

Typical Windows workflow:

```text
Checkout Repository

↓

Install Rust

↓

Build Release

↓

Download GStreamer

↓

Download ONNX Runtime

↓

Create Bundle

↓

Build Installer

↓

Upload VisionOTGSetup.exe
```

Typical Raspberry Pi workflow:

```text
Checkout Repository

↓

Install Rust

↓

Install apt Dependencies

↓

Build Release

↓

Package Files

↓

Build Debian Package

↓

Upload visionotg_3.0_arm64.deb
```

---

# Future Improvements

As the project grows, the CI/CD pipeline can be extended to include:

- Automatic version numbering
- GitHub Releases
- SHA256 checksum generation
- Code signing (Windows)
- Automatic update support
- Cross-platform release builds
- Nightly development builds

These additions can be integrated without changing the overall project structure described in this document.