use crate::hiddev::HidDevice;
use std::path::PathBuf;

pub fn check_get_devices() -> Vec<String> {
    find_all_devices()
}

fn find_all_devices() -> Vec<String> {
    let mut paths = Vec::new();
    create_hiddev_paths(&mut paths, "/dev/usb".to_string());
    create_hiddev_paths(&mut paths, "/dev".to_string());

    eprintln!("[DEBUG] Found {} hiddev paths", paths.len());

    let mut devices = Vec::new();

    // Check all paths that include "hiddev"
    for path in paths {
        if path.to_str().is_some_and(|p| p.contains("hiddev")) {
            let path_str = path.to_str().unwrap_or("");
            eprintln!("[DEBUG] Checking path: {}", path_str);

            // Test if this device is a valid Apple display
            if HidDevice::is_valid_device(path_str) {
                eprintln!("[DEBUG] Valid device found: {}", path_str);
                devices.push(path_str.to_string());
            }
        }
    }

    eprintln!("[DEBUG] Total valid devices: {}", devices.len());
    devices.sort();
    devices
}

fn create_hiddev_paths(paths: &mut Vec<PathBuf>, base: String) {
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if !path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("hiddev"))
            {
                paths.push(path);
            }
        }
    }
}
