#!/bin/bash

set -e

# Navigate to the project root
cd "$(dirname "$0")/.."

# Clean up previous build
rm -rf build release
mkdir release

# Set up and build in production mode
meson setup build -Dmode=prod
ninja -C build

# Copy the binary to the release directory
cp build/asdcontrol-gnome release/

# Package the release directory into a tarball inside the release folder
tar -czvf release/asdcontrol-gnome.tar.gz -C release .

echo "Build complete. Release package: release/asdcontrol-gnome.tar.gz"
