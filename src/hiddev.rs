// Linux HID device bindings
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::path::Path;

// HID report types
const HID_REPORT_TYPE_FEATURE: u32 = 3;

// HID constants
const BRIGHTNESS_CONTROL: u32 = 1;
const USAGE_CODE: u32 = 0x820001;

// Supported devices
const APPLE_VENDOR: u16 = 0x05ac;
const STUDIO_DISPLAY_27: u16 = 0x1114;
const PRO_XDR_DISPLAY_32: u16 = 0x9243;

// Brightness limits for Apple displays
const BRIGHTNESS_MIN: i32 = 400;
const BRIGHTNESS_MAX: i32 = 60000;

// HID device info structure
#[repr(C)]
struct HiddevDevinfo {
    bustype: u32,
    busnum: u32,
    devnum: u32,
    ifnum: u32,
    vendor: u16,
    product: u16,
    version: u16,
    num_applications: u32,
}

// HID report info structure
#[repr(C)]
struct HiddevReportInfo {
    report_type: u32,
    report_id: u32,
    num_fields: u32,
}

// HID usage reference structure
#[repr(C)]
struct HiddevUsageRef {
    report_type: u32,
    report_id: u32,
    field_index: u32,
    usage_index: u32,
    usage_code: u32,
    value: i32,
}

// ioctl definitions
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << 30) | (ty << 8) | nr | (size << 16)
}

const fn io(ty: u32, nr: u32) -> u32 {
    ioc(0, ty, nr, 0)
}

const fn ior<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ, ty, nr, size_of::<T>() as u32)
}

const fn iow<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, ty, nr, size_of::<T>() as u32)
}

const fn iowr<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, ty, nr, size_of::<T>() as u32)
}

const HIDIOCGDEVINFO: u32 = ior::<HiddevDevinfo>(b'H' as u32, 0x03);
const HIDIOCGUSAGE: u32 = iowr::<HiddevUsageRef>(b'H' as u32, 0x0B);
const HIDIOCSUSAGE: u32 = iow::<HiddevUsageRef>(b'H' as u32, 0x0C);
const HIDIOCSREPORT: u32 = iow::<HiddevReportInfo>(b'H' as u32, 0x08);
const HIDIOCINITREPORT: u32 = io(b'H' as u32, 0x05);
const HIDIOCAPPLICATION: u32 = io(b'H' as u32, 0x02);

// Safe ioctl wrapper
unsafe fn ioctl(fd: i32, request: u32, arg: *mut libc::c_void) -> i32 { unsafe {
    libc::ioctl(fd, request as libc::c_ulong, arg)
}}

pub struct HidDevice {
    file: File,
    brightness_min: i32,
    brightness_max: i32,
}

impl HidDevice {
    /// Open a HID device and verify it's a supported Apple display
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(|e| format!("Failed to open device: {}", e))?;

        let fd = file.as_raw_fd();

        // Get device info
        let mut devinfo: HiddevDevinfo = unsafe { std::mem::zeroed() };
        unsafe {
            if ioctl(
                fd,
                HIDIOCGDEVINFO,
                &mut devinfo as *mut _ as *mut libc::c_void,
            ) < 0
            {
                return Err("Failed to get device info".to_string());
            }
        }

        // Check if it's a supported Apple display
        let vendor = devinfo.vendor;
        let product = devinfo.product;

        if vendor != APPLE_VENDOR {
            return Err(format!("Not an Apple device (vendor: 0x{:04x})", vendor));
        }

        if product != STUDIO_DISPLAY_27 && product != PRO_XDR_DISPLAY_32 {
            return Err(format!(
                "Unsupported Apple display (product: 0x{:04x})",
                product
            ));
        }

        // Check if it's a USB monitor (application 0x80)
        let mut is_monitor = false;
        for appl_num in 0..devinfo.num_applications {
            // HIDIOCAPPLICATION takes the application number as a value and returns the application ID
            let application =
                unsafe { libc::ioctl(fd, HIDIOCAPPLICATION as libc::c_ulong, appl_num) };

            if application >= 0 {
                if ((application >> 16) & 0xFF) == 0x80 {
                    is_monitor = true;
                    eprintln!("[DEBUG] Found USB Monitor application!");
                    break;
                }
            } else {
                eprintln!("[DEBUG] Failed to get application {} info", appl_num);
            }
        }

        if !is_monitor {
            eprintln!("[DEBUG] Not a USB monitor - no 0x80 application found");
            return Err("Device is not a USB monitor".to_string());
        }

        // Initialize report structures
        unsafe {
            if libc::ioctl(fd, HIDIOCINITREPORT as libc::c_ulong, 0) < 0 {
                return Err("Failed to initialize report structures".to_string());
            }
        }

        Ok(HidDevice {
            file,
            brightness_min: BRIGHTNESS_MIN,
            brightness_max: BRIGHTNESS_MAX,
        })
    }

    /// Get current brightness value
    pub fn get_brightness(&self) -> Result<i32, String> {
        let fd = self.file.as_raw_fd();

        let mut usage_ref = HiddevUsageRef {
            report_type: HID_REPORT_TYPE_FEATURE,
            report_id: BRIGHTNESS_CONTROL,
            field_index: 0,
            usage_index: 0,
            usage_code: USAGE_CODE,
            value: 0,
        };

        unsafe {
            let ret = libc::ioctl(fd, HIDIOCGUSAGE as libc::c_ulong, &mut usage_ref);
            if ret < 0 {
                let err = std::io::Error::last_os_error();
                eprintln!("[DEBUG] HIDIOCGUSAGE failed: {} (ret={})", err, ret);
                return Err(format!("Failed to get usage: {}", err));
            }
        }

        Ok(usage_ref.value)
    }

    /// Set brightness value
    pub fn set_brightness(&self, brightness: i32) -> Result<(), String> {
        let fd = self.file.as_raw_fd();

        // Clamp brightness to valid range
        let brightness = brightness.max(self.brightness_min).min(self.brightness_max);

        let mut usage_ref = HiddevUsageRef {
            report_type: HID_REPORT_TYPE_FEATURE,
            report_id: BRIGHTNESS_CONTROL,
            field_index: 0,
            usage_index: 0,
            usage_code: USAGE_CODE,
            value: brightness,
        };

        let mut rep_info = HiddevReportInfo {
            report_type: HID_REPORT_TYPE_FEATURE,
            report_id: BRIGHTNESS_CONTROL,
            num_fields: 1,
        };

        unsafe {
            if libc::ioctl(fd, HIDIOCSUSAGE as libc::c_ulong, &mut usage_ref) < 0 {
                let err = std::io::Error::last_os_error();
                return Err(format!("Failed to set usage: {}", err));
            }

            if libc::ioctl(fd, HIDIOCSREPORT as libc::c_ulong, &mut rep_info) < 0 {
                let err = std::io::Error::last_os_error();
                return Err(format!("Failed to write report: {}", err));
            }
        }

        Ok(())
    }

    /// Check if a path is a valid Apple display device
    pub fn is_valid_device<P: AsRef<Path>>(path: P) -> bool {
        Self::open(path).is_ok()
    }
}
