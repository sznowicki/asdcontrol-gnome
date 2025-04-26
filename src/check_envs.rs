use std::process::Command;
use gtk4::prelude::*;
use gtk4::{MessageDialog, MessageType, ButtonsType};

pub fn check_asdcontrol_command() {
    let asd_control = "aasdcontrol";

    if Command::new(asd_control).output().is_err() {
        let app = gtk4::Application::builder()
            .application_id("com.sznowicki.asdcontrol-gnome-check")
            .build();

        app.connect_activate(|_| {
            let dialog = MessageDialog::builder()
                .message_type(MessageType::Error)
                .buttons(ButtonsType::Ok)
                .text(t!("error.noasdcontrol"))
                .modal(true)         // Make the dialog modal
                .build();
            dialog.connect_response(|dialog, _| dialog.close());
            dialog.show();
        });

        app.run();
    }
}
