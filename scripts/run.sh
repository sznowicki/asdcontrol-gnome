#!/bin/bash

set -e

BUILD_DIR="../build"

# Ensure build directory exists
if [ ! -d "$BUILD_DIR" ]; then
    meson setup "$BUILD_DIR"
fi

meson compile -C "$BUILD_DIR"
"$BUILD_DIR/asdcontrol-gnome"
