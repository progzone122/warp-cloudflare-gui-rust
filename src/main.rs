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
use slint::{SharedString, Model, Weak, Timer, TimerMode};
use std::time::Duration;

fn show_dialog(weak: &Weak<CustomDialogWindow>, dialog_type: &str, text: &str) {
    if let Some(dialog) = weak.upgrade() {
        dialog.set_dialog_type(SharedString::from(dialog_type.to_string()));
        dialog.set_dialog_text(SharedString::from(text.to_string()));
        dialog.show().expect("Error: Failed to show window");
    }
}

const APP_VERSION: &str = "0.0.1";

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let dialog = CustomDialogWindow::new()?;
    let settings = CustomSettingsWindow::new()?;

    let api: Api = Api::new();

    app.set_window_title(SharedString::from("WARP"));

    let app_weak = app.as_weak();
    let dialog_weak = dialog.as_weak();
    let setting_weak = settings.as_weak();

    // Set app version
    setting_weak.unwrap().set_app_version(SharedString::from(APP_VERSION));

    // Set default switch On/Off value
    if app_weak.upgrade().unwrap().get_connection_switch_enabled() {
        let connected_status: bool = api.is_connected();
        let app_strong = app_weak.upgrade().unwrap();
        app_strong.set_connection_switch_checked(connected_status);

        if connected_status {
            app_strong.set_status(SharedString::from("Connected"));
        } else {
            app_strong.set_status(SharedString::from("Disconnected"));
        }
    }

    // Switch On/Off connection
    let app_weak_clone = app_weak.clone();
    let dialog_weak_clone = dialog_weak.clone();
    app.on_connection_switch_toggled(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            if app.get_connection_switch_enabled() {
                if app.get_connection_switch_checked() {
                    if api.disconnect() {
                        app.set_status(SharedString::from("Disconnected"));
                        app.set_connection_switch_checked(false);
                    } else {
                        let status: String = api.status();
                        app.set_status(SharedString::from("Error"));
                        show_dialog(&dialog_weak_clone, "error", &status);
                    }
                } else {
                    if api.connect() {
                        if api.is_connected() {
                            app.set_status(SharedString::from("Connected"));
                            app.set_connection_switch_checked(true);
                        } else {
                            let status: String = api.status();
                            app.set_status(SharedString::from("Disconnected"));
                            app.set_connection_switch_checked(false);
                            show_dialog(&dialog_weak_clone, "error", &status);
                        }
                    } else {
                        let status: String = api.status();
                        app.set_status(SharedString::from("Error"));
                        show_dialog(&dialog_weak_clone, "error", &status);
                    }
                }
            }
        }
    });

    // Clicked the settings button
    let setting_weak_clone = setting_weak.clone();
    app.on_settings_clicked_button(move || {
        if let Some(setting) = setting_weak_clone.upgrade() {
            setting.show().expect("Error: Failed to show window");
        }
    });

    // Set default warp mode
    if let Some(setting) = setting_weak.upgrade() {
        setting.set_warp_mode_setting_index(api.get_mode());
    }

    // Set warp mode
    let setting_weak_clone = setting_weak.clone();
    settings.on_warp_mode_setting_choose(move || {
        if let Some(setting) = setting_weak_clone.upgrade() {
            let modes_model: slint::ModelRc<SharedString> = setting.get_warp_mode_setting_model();
            let modes_index: i32 = setting.get_warp_mode_setting_index();

            if let Some(modes) = modes_model.as_any().downcast_ref::<slint::VecModel<SharedString>>() {
                let modes: Vec<SharedString> = modes.iter().collect();
                api.set_mode(&modes[modes_index as usize].as_str());
            }
        }
    });

    // Register account
    let setting_weak_clone = setting_weak.clone();
    let dialog_weak_clone_register = dialog_weak.clone();
    let timer1 = Timer::default();
    settings.on_account_register_clicked(move || {
        if let Some(setting) = setting_weak_clone.upgrade() {
            let dialog_weak_clone_register2 = dialog_weak_clone_register.clone();

            // The timer is needed for a delay so that the buttons have time to deactivate after pressing
            timer1.start(TimerMode::SingleShot, Duration::from_millis(500), move || {
                match api.register_account() {
                    Ok(_) => {
                        show_dialog(&dialog_weak_clone_register2, "warning", "Account successfully registered");
                    }
                    Err(e) => {
                        show_dialog(&dialog_weak_clone_register2, "error", &e);
                    }
                };
                setting.set_account_register_enabled(true);
                setting.set_account_delete_enabled(true);
            });
        }
    });

    // Delete account
    let setting_weak_clone = setting_weak.clone();
    let dialog_weak_clone_delete = dialog_weak.clone();
    let timer2 = Timer::default();
    settings.on_account_delete_clicked(move || {
        if let Some(setting) = setting_weak_clone.upgrade() {
            let dialog_weak_clone_delete2 = dialog_weak_clone_delete.clone();

            // The timer is needed for a delay so that the buttons have time to deactivate after pressing
            timer2.start(TimerMode::SingleShot, Duration::from_millis(500), move || {
                match api.delete_account() {
                    Ok(_) => {
                        show_dialog(&dialog_weak_clone_delete2, "warning", "Account successfully deleted");
                    }
                    Err(e) => {
                        show_dialog(&dialog_weak_clone_delete2, "error", &e);
                    }
                };
                setting.set_account_register_enabled(true);
                setting.set_account_delete_enabled(true);
            });
        }
    });

    // Close dialog
    let dialog_weak_clone = dialog_weak.clone();
    dialog.on_dialog_clicked_ok(move || {
        if let Some(dialog_weak) = dialog_weak_clone.upgrade() {
            dialog_weak.hide().expect("Error: Failed to hide window");
        }
    });

    app.run()
}