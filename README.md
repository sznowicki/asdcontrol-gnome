# asdcontrol - GNOME GUI

![screenshot of the slider](./docs/screenshot.png)

This is a small GUI app that directly controls the brightness of Apple Studio Displays on Linux using the USB HID protocol.

Inspired by [Apple Display Brightness Control](https://github.com/nikosdion/asdcontrol), this app provides a native GTK interface for controlling your display brightness.

## Features

- **Multi-display support**: Automatically detects all connected Apple Studio Displays
- **Direct USB HID communication**: No external dependencies - talks directly to the displays via Linux hiddev
- **Simple GTK interface**: Clean, minimal sliders for brightness control
- **Per-display control**: Each detected display gets its own slider

On app start, it scans `/dev/hiddev*` and `/dev/usb/hiddev*` devices to find all compatible Apple displays (Studio Display 27" and Pro XDR Display 32").

If no displays are found, a message is shown. Otherwise, you'll see a brightness slider for each detected display.

## Build

I experimented with different languages for GTK, but settled on Rust as it seems to be the nicest of all available ones.

And I didn't want to do js as it's my job-language and I wanted to do something different.

To build it, ensure you have rust development stuff on your machine, then build it. Luckily in Rust it's that easy.

## Contribute

If you want to improve it, feel free to propose a PR.

## udev rules

(Exact rule location may vary by distro)

**For Apple Studio Display (2022, 27")**

Create `/etc/udev/rules.d/50-apple-studio.rules`:

```bash
sudo tee /etc/udev/rules.d/50-apple-studio.rules <<EOF
KERNEL=="hiddev*", ATTRS{idVendor}=="05ac", ATTRS{idProduct}=="1114", GROUP="users", OWNER="root", MODE="0660"
EOF
```

**For Apple Pro XDR Display (2019, 32")**

Create `/etc/udev/rules.d/50-apple-xdr.rules`:

```bash
sudo tee /etc/udev/rules.d/50-apple-xdr.rules <<EOF
KERNEL=="hiddev*", ATTRS{idVendor}=="05ac", ATTRS{idProduct}=="9243", GROUP="users", OWNER="root", MODE="0660"
EOF
```
