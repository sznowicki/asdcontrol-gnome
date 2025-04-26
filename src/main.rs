mod check_envs;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation, Scale};
use check_envs::check_asdcontrol_command;
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
            .default_height(300)
            .build();

        let slider = Scale::builder()
            .orientation(Orientation::Horizontal)
            .adjustment(&gtk4::Adjustment::new(0.0, 0.0, 100.0, 1.0, 1.0, 1.0))
            .build();

        slider.connect_value_changed(|s| {
            let value = s.value().ceil() as i32; // Normalize to integer using ceil
            println!("Slider value: {}", value);
        });

        let container = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        container.append(&slider);
        window.set_child(Some(&container));
        window.show();
    });
    check_asdcontrol_command();
    app.run();
}

