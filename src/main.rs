mod api;

use api::Api;
use slint::SharedString;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let api = Api::new();
    app.set_window_title(SharedString::from("WARP"));

    let app_weak = app.as_weak();

    app.on_connection_switch_toggled(move || {
        if (app_weak.unwrap().get_connection_switch_enabled()) {
            if (app_weak.unwrap().get_connection_switch_checked()) {
                if api.disconnect() == true {
                    let status: String = api.status();
                    app_weak.unwrap().set_status(SharedString::from(status));
                    app_weak.unwrap().set_connection_switch_checked(false);
                }
            } else {
                if api.connect() == true {
                    let status: String = api.status();
                    if status == "Connecting" {
                        app_weak.unwrap().set_status(SharedString::from("Connected"));
                        app_weak.unwrap().set_connection_switch_checked(true);
                    } else {
                        app_weak.unwrap().set_status(SharedString::from(status));
                    }
                }
            }
        }
    });

    app.run()
}