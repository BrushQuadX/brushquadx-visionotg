#!/usr/bin/env bash
set -euo pipefail

# Build a native Debian package for Raspberry Pi 5 (aarch64).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPOSITORY_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_SCRIPT="$SCRIPT_DIR/build.sh"
ORT_BUILD_DIR="$SCRIPT_DIR/onnxruntime/build/Linux/Release"
PACKAGE_NAME="visionotg"
VERSION=""
OUTPUT_DIRECTORY="$REPOSITORY_ROOT/target/linux-packages"
DO_SETUP=false
SKIP_BUILD=false

usage() {
	echo "Usage: $0 [--setup] [--skip-build] [--version VERSION] [--output DIRECTORY]"
	echo
	echo "  --setup              install build dependencies and build ONNX Runtime"
	echo "  --skip-build         package the existing release binary and runtime"
	echo "  --version VERSION    Debian version (defaults to Cargo.toml version)"
	echo "  --output DIRECTORY   directory for the resulting .deb"
}

while [ "$#" -gt 0 ]; do
	case "$1" in
		--setup) DO_SETUP=true ;;
		--skip-build) SKIP_BUILD=true ;;
		--version)
			shift
			[ "$#" -gt 0 ] || { echo "--version requires a value" >&2; exit 1; }
			VERSION="$1"
			;;
		--output)
			shift
			[ "$#" -gt 0 ] || { echo "--output requires a directory" >&2; exit 1; }
			OUTPUT_DIRECTORY="$1"
			;;
		--help|-h) usage; exit 0 ;;
		*) echo "Unknown option: $1" >&2; usage >&2; exit 1 ;;
	esac
	shift
done

if [ "$(uname -m)" != "aarch64" ]; then
	echo "This script must run on an aarch64 Linux host (Raspberry Pi 5)." >&2
	exit 1
fi

command -v dpkg-deb >/dev/null 2>&1 || {
	echo "dpkg-deb is required. Install it with: sudo apt install dpkg-dev" >&2
	exit 1
}

if [ -z "$VERSION" ]; then
	VERSION="$(sed -n 's/^version = "\([0-9][0-9.]*\)"$/\1/p' "$REPOSITORY_ROOT/Cargo.toml" | head -n 1)"
fi

if [[ ! "$VERSION" =~ ^[0-9]+(\.[0-9]+){1,3}$ ]]; then
	echo "Version must contain two to four numeric components, for example 1.0.0." >&2
	exit 1
fi

BUILD_ARGS=()
if [ "$DO_SETUP" = true ]; then BUILD_ARGS+=(--setup); fi
if [ "$SKIP_BUILD" = true ]; then
	BINARY="$REPOSITORY_ROOT/target/release/votg"
else
	"$BUILD_SCRIPT" "${BUILD_ARGS[@]}"
	BINARY="$REPOSITORY_ROOT/target/release/votg"
fi

[ -x "$BINARY" ] || { echo "Release executable not found: $BINARY" >&2; exit 1; }

ORT_LIBRARY="$ORT_BUILD_DIR/libonnxruntime.so"
[ -f "$ORT_LIBRARY" ] || { echo "ONNX Runtime library not found: $ORT_LIBRARY" >&2; exit 1; }

PACKAGE_ROOT="$REPOSITORY_ROOT/target/linux-package/${PACKAGE_NAME}_${VERSION}_arm64"
rm -rf "$PACKAGE_ROOT"
mkdir -p \
	"$PACKAGE_ROOT/DEBIAN" \
	"$PACKAGE_ROOT/opt/visionotg/models" \
	"$PACKAGE_ROOT/opt/visionotg/lib" \
	"$PACKAGE_ROOT/usr/bin" \
	"$PACKAGE_ROOT/usr/share/applications"

sed \
	-e "s/@@VERSION@@/$VERSION/g" \
	"$SCRIPT_DIR/control" > "$PACKAGE_ROOT/DEBIAN/control"
cp "$SCRIPT_DIR/postinst" "$PACKAGE_ROOT/DEBIAN/postinst"
chmod 0755 "$PACKAGE_ROOT/DEBIAN/postinst"

cp "$BINARY" "$PACKAGE_ROOT/opt/visionotg/votg"
cp "$ORT_LIBRARY" "$PACKAGE_ROOT/opt/visionotg/lib/libonnxruntime.so"
cp "$REPOSITORY_ROOT/assets/models/"*.onnx "$PACKAGE_ROOT/opt/visionotg/models/"
cp "$SCRIPT_DIR/visionotg-launcher" "$PACKAGE_ROOT/usr/bin/votg"
cp "$SCRIPT_DIR/visionotg.desktop" "$PACKAGE_ROOT/usr/share/applications/visionotg.desktop"
chmod 0755 "$PACKAGE_ROOT/opt/visionotg/votg" "$PACKAGE_ROOT/usr/bin/votg"

mkdir -p "$OUTPUT_DIRECTORY"
OUTPUT_PACKAGE="$OUTPUT_DIRECTORY/${PACKAGE_NAME}_${VERSION}_arm64.deb"
dpkg-deb --build --root-owner-group "$PACKAGE_ROOT" "$OUTPUT_PACKAGE"
echo "Created $OUTPUT_PACKAGE"
