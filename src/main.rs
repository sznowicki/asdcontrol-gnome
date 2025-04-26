mod check_envs;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use check_envs::check_asdcontrol_command;

fn main() {

    let app = Application::builder()
        .application_id("com.sznowicki.asdcontrol-gnome")
        .build();

    check_asdcontrol_command();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("ASDControl GNOME")
            .default_width(600)
            .default_height(300)
            .build();
        window.show();
    });

    app.run();
}

