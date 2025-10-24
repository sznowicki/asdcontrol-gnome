mod asdcontrol_bind;
mod check_envs;
mod hiddev;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, Orientation, Scale};
#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

fn main() {
    let app = Application::builder()
        .application_id("com.sznowicki.asdcontrol-gnome")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("ASDControl GNOME")
            .default_width(600)
            .default_height(50)
            .build();

        let devices = check_envs::check_get_devices();

        let container = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        if devices.is_empty() {
            // No devices found - show error message
            let label = Label::new(Some("No Apple Studio Displays detected"));
            container.append(&label);
        } else {
            // Create a slider for each detected device
            for device_path in devices {
                // Get initial brightness for this device
                let bg_value = asdcontrol_bind::get_bg_value(&device_path);

                // Create label showing device path
                let device_label = Label::new(Some(&format!("Device: {}", device_path)));
                device_label.set_xalign(0.0); // Left align

                // Create slider for brightness control
                let slider = Scale::builder()
                    .orientation(Orientation::Horizontal)
                    .adjustment(&gtk4::Adjustment::new(
                        bg_value as f64,
                        0.0,
                        100.0,
                        1.0,
                        1.0,
                        1.0,
                    ))
                    .hexpand(true)
                    .build();

                // Clone device_path for the closure
                let device_clone = device_path.clone();
                slider.connect_value_changed(move |s| {
                    let value = s.value().ceil() as i32; // Normalize to integer using ceil
                    asdcontrol_bind::set_bg_value(&device_clone, value);
                    println!("Device {} brightness: {}", device_clone, value);
                });

                // Add label and slider to container
                container.append(&device_label);
                container.append(&slider);
            }
        }

        window.set_child(Some(&container));
        window.show();
    });
    app.run();
}
