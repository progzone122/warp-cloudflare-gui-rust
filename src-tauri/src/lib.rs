use anyhow::Result;
use std::sync::Mutex;
use tauri::State;

pub mod api;
use api::*;
#[tauri::command]
fn connect_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.connect().unwrap_or_else(|err| {
        eprintln!("Error connecting: {}", err);
        false
    })
}

#[tauri::command]
fn disconnect_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.disconnect().unwrap_or_else(|err| {
        eprintln!("Error disconnecting: {}", err);
        false
    })
}

#[tauri::command]
fn status_api(api: State<Mutex<Api>>) -> Result<Response, Response> {
    let api = api.lock().unwrap();
    match api.status() {
        Ok(resp) => Ok(resp),
        Err(err) => {
            eprintln!("Error getting status: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
fn is_connected_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.is_connected().unwrap_or_else(|err| {
        eprintln!("Error checking connection: {}", err);
        false
    })
}

// #[tauri::command]
// fn get_mode_api(api: State<Mutex<Api>>) -> i32 {
//     let api = api.lock().unwrap();
//     api.get_mode()
// }
//
// #[tauri::command]
// fn set_mode_api(api: State<Mutex<Api>>, mode: String) {
//     let api = api.lock().unwrap();
//     api.set_mode(&mode);
// }
//
#[tauri::command]
fn register_account_api(api: State<Mutex<Api>>) -> Result<Response, Response> {
    let api = api.lock().unwrap();
    match api.register_account() {
        Ok(resp) => Ok(resp),
        Err(err) => {
            eprintln!("Error register account: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
fn delete_account_api(api: State<Mutex<Api>>) -> Result<Response, Response> {
    let api = api.lock().unwrap();
    match api.delete_account() {
        Ok(resp) => Ok(resp),
        Err(err) => {
            eprintln!("Error delete account: {}", err);
            Err(err.into())
        }
    }
}

// Основная точка запуска приложения Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(Api::new()))
        .invoke_handler(tauri::generate_handler![
            connect_api,
            disconnect_api,
            status_api,
            is_connected_api,
            // get_mode_api,
            // set_mode_api,
            register_account_api,
            delete_account_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
