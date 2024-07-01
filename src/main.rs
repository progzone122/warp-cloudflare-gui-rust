mod win1 {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}
mod win2 {
    include!(concat!(env!("OUT_DIR"), "/dialog.rs"));
}
mod win3 {
    include!(concat!(env!("OUT_DIR"), "/settings.rs"));
}

slint::include_modules!();
use win1::AppWindow;
use win2::DialogWindow as CustomDialogWindow;
use win3::SettingsWindow as CustomSettingsWindow;

mod api;

use api::Api;
use slint::{SharedString, Weak};

fn show_dialog(weak: &Weak<CustomDialogWindow>, dialog_type: String, text: String) {
    weak.unwrap().set_dialog_type(SharedString::from(dialog_type));
    weak.unwrap().set_dialog_text(SharedString::from(text));
    weak.unwrap().show().expect("Error: Failed to show window");
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let dialog = CustomDialogWindow::new()?;
    let settings = CustomSettingsWindow::new()?;

    let api = Api::new();
    app.set_window_title(SharedString::from("WARP"));

    let app_weak = app.as_weak();
    let dialog_weak = dialog.as_weak();
    let dialog_weak2 = dialog.as_weak();
    let setting_weak = settings.as_weak();

    // Set default switch On/Off value
    if app_weak.unwrap().get_connection_switch_enabled() {
        let connected_status: bool = api.is_connected();
        app_weak.unwrap().set_connection_switch_checked(connected_status);

        if connected_status {
            app_weak.unwrap().set_status(SharedString::from("Connected"));
        } else {
            app_weak.unwrap().set_status(SharedString::from("Disconnected"));
        }
    }

    // Switch On/Off connection
    app.on_connection_switch_toggled(move || {
        if app_weak.unwrap().get_connection_switch_enabled() {
            if app_weak.unwrap().get_connection_switch_checked() {
                if api.disconnect() {
                    app_weak.unwrap().set_status(SharedString::from("Disconnected"));
                    app_weak.unwrap().set_connection_switch_checked(false);
                } else {
                    let status: String = api.status();
                    app_weak.unwrap().set_status(SharedString::from("Error"));
                    show_dialog(&dialog_weak, "error".to_string(), status);
                }
            } else {
                if api.connect() {
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

    // Clicked the settings button
    app.on_settings_clicked_button(move || {
        setting_weak.unwrap().show().expect("Error: Failed to show window");
    });

    // Close dialog
    dialog.on_dialog_clicked_ok(move || {
        dialog_weak2.unwrap().hide().expect("Error: Failed to hide window");
    });

    app.run()
}