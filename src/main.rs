mod check_envs;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
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
        window.show();
    });
    check_asdcontrol_command();
    app.run();


}

