#!/bin/bash

# Navigate to the project root
cd "$(dirname "$0")/.."

# Ensure Meson is set up with production mode
if [ ! -d "build" ]; then
  meson setup build -Dmode=prod
fi

# Build and run the application
ninja -C build && ./build/asdcontrol-gnome
