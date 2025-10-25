use crate::hiddev::HidDevice;

// Brightness range for Apple Studio Display
const BRIGHTNESS_MIN: i32 = 400;
const BRIGHTNESS_MAX: i32 = 60000;
const BRIGHTNESS_RANGE: i32 = BRIGHTNESS_MAX - BRIGHTNESS_MIN; // 59600

/// Get brightness value (0-100 scale)
pub fn get_bg_value(device: &str) -> i32 {
    eprintln!("[DEBUG] get_bg_value for device: {}", device);
    match HidDevice::open(device) {
        Ok(hid) => {
            match hid.get_brightness() {
                Ok(raw_value) => {
                    // Convert from 400-60000 range to 0-100 percentage
                    let normalized = ((raw_value - BRIGHTNESS_MIN) * 100) / BRIGHTNESS_RANGE;
                    eprintln!(
                        "[DEBUG] Got brightness: {}% (raw: {})",
                        normalized, raw_value
                    );
                    normalized.clamp(0, 100)
                }
                Err(e) => {
                    eprintln!("[DEBUG] Failed to get brightness: {}", e);
                    50 // Default to mid-range
                }
            }
        }
        Err(e) => {
            eprintln!("[DEBUG] Failed to open device: {}", e);
            50 // Default to mid-range
        }
    }
}

/// Set brightness value (0-100 scale)
pub fn set_bg_value(device: &str, percentage: i32) {
    // Convert from 0-100 percentage to 400-60000 range
    let clamped = percentage.clamp(0, 100);
    let raw_value = (clamped * BRIGHTNESS_RANGE / 100) + BRIGHTNESS_MIN;

    eprintln!(
        "[DEBUG] set_bg_value for device: {}, value: {}% (raw: {})",
        device, clamped, raw_value
    );

    match HidDevice::open(device) {
        Ok(hid) => match hid.set_brightness(raw_value) {
            Ok(_) => eprintln!("[DEBUG] Brightness set successfully"),
            Err(e) => eprintln!("[DEBUG] Failed to set brightness: {}", e),
        },
        Err(e) => eprintln!("[DEBUG] Failed to open device: {}", e),
    }
}
