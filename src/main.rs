mod win1 {
    include!(env!("SLINT_INCLUDE_APP"));
}
mod win2 {
    include!(env!("SLINT_INCLUDE_DIALOG"));
}
slint::include_modules!();
use std::os::linux::raw::stat;
use win1::AppWindow;
use win2::DialogWindow as CustomDialogWindow;

mod api;

use api::Api;
use slint::{SharedString, Weak};

fn show_dialog(weak: &Weak<CustomDialogWindow>, dialog_type: String, text: String) {
    weak.unwrap().set_dialog_type(SharedString::from(dialog_type));
    weak.unwrap().set_dialog_text(SharedString::from(text));
    weak.unwrap().show();
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let dialog = CustomDialogWindow::new()?;

    let api = Api::new();
    app.set_window_title(SharedString::from("WARP"));

    let app_weak = app.as_weak();
    let dialog_weak = dialog.as_weak();
    let dialog_weak2 = dialog.as_weak();

    app.on_connection_switch_toggled(move || {
        if app_weak.unwrap().get_connection_switch_enabled() {
            if app_weak.unwrap().get_connection_switch_checked() {
                if api.disconnect() == true {
                    app_weak.unwrap().set_status(SharedString::from("Disconnected"));
                    app_weak.unwrap().set_connection_switch_checked(false);
                } else {
                    let status: String = api.status();
                    app_weak.unwrap().set_status(SharedString::from("Error"));
                    show_dialog(&dialog_weak, "error".to_string(), status);
                }
            } else {
                if api.connect() == true {
                    if api.is_connected() {
                        app_weak.unwrap().set_status(SharedString::from("Connected"));
                        app_weak.unwrap().set_connection_switch_checked(true);
                    } else {
                        let status: String = api.status();
                        app_weak.unwrap().set_status(SharedString::from("Disconnected"));
                        app_weak.unwrap().set_connection_switch_checked(false);
                        show_dialog(&dialog_weak, "error".to_string(), status);
                    }
                } else {
                    let status: String = api.status();
                    app_weak.unwrap().set_status(SharedString::from("Error"));
                    show_dialog(&dialog_weak, "error".to_string(), status);
                }
            }
        }
    });

    dialog.on_dialog_clicked_ok(move || {
        dialog_weak2.unwrap().hide().expect("Error: Failed to hide window");
    });

    app.run()
}