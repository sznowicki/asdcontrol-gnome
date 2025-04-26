#!/bin/bash

# Navigate to the project root
cd "$(dirname "$0")/.."

# Ensure Meson is set up with development mode
if [ ! -d "build" ]; then
  meson setup build -Dmode=dev
fi

# Watch for changes and rebuild/run
while true; do
  inotifywait -e modify,create,delete -r src
  ninja -C build && ./build/asdcontrol-gnome
done
