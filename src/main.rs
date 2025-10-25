mod asdcontrol_bind;
mod check_envs;
mod hiddev;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, Orientation, Scale};

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

const DEFAULT_MARGIN: i32 = 12;

fn make_label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_xalign(0.0); // Left align
    label.set_margin_start(DEFAULT_MARGIN);
    label.set_margin_end(DEFAULT_MARGIN);
    label.set_margin_top(DEFAULT_MARGIN);
    label.set_margin_bottom(DEFAULT_MARGIN);

    label
}

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
            .spacing(0)
            .build();


        if devices.is_empty() {
            // No devices found - show error message
            let label = make_label("No Apple Studio Displays detected. Restart the application after connecting the display.");
            container.append(&label);
        } else {
            // Create a slider for each detected device
            for device_path in devices {
                // Get initial brightness for this device
                let bg_value = asdcontrol_bind::get_bg_value(&device_path);

                // Create label showing device path
                let device_label = make_label(&format!("Device: {}", device_path));
                device_label.set_margin_bottom(0);

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
                slider.set_margin_top(0); // Remove top spacing
                slider.set_margin_bottom(10); // Add bottom spacing
                slider.set_margin_start(0); // Add left spacing
                slider.set_margin_end(0); // Add right spacing
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
