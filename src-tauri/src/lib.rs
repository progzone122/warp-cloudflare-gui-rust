use std::borrow::Cow;
use std::process::{Command, Output};
use std::sync::Mutex;
use tauri::State;
use anyhow::{anyhow, Result};
use anyhow::Error as AnyhowError;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[derive(Debug)]
pub enum StatusCode {
    UnknownError,
    UnexpectedError,
    ParsingError,
    DaemonError,
    Success,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct Response {
    pub message: String,
    pub details: String,
    pub code: StatusCode,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl From<AnyhowError> for Response {
    fn from(err: AnyhowError) -> Self {
        Response::new(StatusCode::UnknownError, "An error occurred", &format!("{:?}", err))
    }
}

impl Response {
    pub fn new(code: StatusCode, message: &str, details: &str) -> Self {
        Self {
            message: message.to_string(),
            details: details.to_string(),
            code,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Api {}
    }
    pub fn connect(&self) -> Result<bool> {
        let result = Command::new("sh")
            .arg("-c")
            .arg("warp-cli connect")
            .output()
            .map_err(|e| anyhow!("Failed to execute warp-cli connect command: {}", e))?;
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            if stdout.trim() == "Success" {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn disconnect(&self) -> Result<bool> {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli disconnect")
            .output()
            .map_err(|e| anyhow!("Failed to execute warp-cli connect command: {}", e))?;

        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stdout : &str = stdout.trim_end_matches("\n");
            if stdout == "Success" {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn status(&self) -> Result<Response> {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli status")
            .output()
            .map_err(|e| anyhow!("Failed to execute warp-cli command: {}", e))?;

        if result.status.success() {
            let stdout: Cow<str> = String::from_utf8_lossy(&result.stdout);
            let parts: Vec<&str> = stdout.split(':').collect();

            if parts.len() > 2 {
                let lines: Vec<&str> = parts[2].split('\n').collect();
                return Ok(Response::new(
                    StatusCode::Success,
                    "",
                    lines[0].trim(),
                ));
            } else if parts.len() > 1 {
                let lines: Vec<&str> = parts[1].split('\n').collect();
                return Ok(Response::new(
                    StatusCode::Success,
                    "",
                    lines[0].trim(),
                ));
            }
        } else {
            let stderr: Cow<str> = String::from_utf8_lossy(&result.stderr);

            if stderr.contains("Connection refused (os error 111)") {
                return Err(anyhow!(Response::new(
                    StatusCode::DaemonError,
                    "Unable to connect to the CloudflareWARP daemon. Maybe the daemon is not running?",
                    stderr.as_ref(),
                )));
            } else {
                return Err(anyhow!(Response::new(
                    StatusCode::UnknownError,
                    "Check the logs for details",
                    stderr.as_ref(),
                )));
            }
        }

        Err(anyhow!(Response::new(
            StatusCode::ParsingError,
            "Unexpected error while parsing warp-cli output",
            "No details available",
        ).to_string()))
    }
    pub fn is_connected(&self) -> Result<bool> {
        let status = self.status()?; // Обрабатываем возможные ошибки
        Ok(status.details == "Connecting" || status.details == "Connected")
    }

    // pub fn get_mode(&self) -> i32 {
    //     let result: Output = Command::new("sh")
    //         .arg("-c")
    //         .arg("warp-cli settings")
    //         .output()
    //         .expect("failed to execute process");
    //
    //     if result.status.success() {
    //         let stdout: Cow<str> = String::from_utf8_lossy(&result.stdout);
    //         let parts: Vec<&str> = stdout.split('\n').collect();
    //         let parts: Vec<&str> = parts[3].split(':').collect();
    //         let mode: &str = parts[1].trim();
    //
    //         match mode {
    //             "Warp" => return 0, // warp
    //             "DnsOverHttps" => return 1, // https
    //             "WarpWithDnsOverHttps" => return 2, // warp+doh
    //             "DnsOverTls" => return 3, // dot
    //             "WarpWithDnsOverTls" => 4, // warp+dot
    //             "WarpProxy on port 40000" => 5, // proxy
    //             "TunnelOnly" => 6, // tunnel_only
    //             _ => return 0
    //         }
    //     } else {
    //         0
    //     }
    // }
    // pub fn set_mode(&self, mode: &str) {
    //     Command::new("sh")
    //         .arg("-c")
    //         .arg(format!("warp-cli set-mode {}", mode))
    //         .output()
    //         .expect("failed to execute process");
    // }
    //
    // pub fn register_account(&self) -> Result<(), String> {
    //     let result: Output = Command::new("sh")
    //         .arg("-c")
    //         .arg("warp-cli registration new")
    //         .output()
    //         .expect("failed to execute process");
    //
    //     return if result.status.success() {
    //         let result: Cow<str> = String::from_utf8_lossy(&result.stdout);
    //         let parts: Vec<&str> = result.split("\n").collect();
    //
    //         if parts[0] == "Success" {
    //             Ok(())
    //         } else {
    //             Err(result.to_string())
    //         }
    //     } else {
    //         let error: Cow<str> = String::from_utf8_lossy(&result.stderr);
    //         Err(error.to_string())
    //     }
    // }
    // pub fn delete_account(&self) -> Result<(), String> {
    //     let result: Output = Command::new("sh")
    //         .arg("-c")
    //         .arg("warp-cli registration delete")
    //         .output()
    //         .expect("failed to execute process");
    //
    //     return if result.status.success() {
    //         let result: Cow<str> = String::from_utf8_lossy(&result.stdout);
    //         let parts: Vec<&str> = result.split("\n").collect();
    //
    //         if parts[0] == "Success" {
    //             Ok(())
    //         } else {
    //             Err(result.to_string())
    //         }
    //     } else {
    //         let error: Cow<str> = String::from_utf8_lossy(&result.stderr);
    //         Err(error.to_string())
    //     }
    // }
}

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
// #[tauri::command]
// fn register_account_api(api: State<Mutex<Api>>) -> Result<(), String> {
//     let api = api.lock().unwrap();
//     api.register_account()
// }
//
// #[tauri::command]
// fn delete_account_api(api: State<Mutex<Api>>) -> Result<(), String> {
//     let api = api.lock().unwrap();
//     api.delete_account()
// }

// Основная точка запуска приложения Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Api::new())) // Передаём экземпляр Api в приложение
        .invoke_handler(tauri::generate_handler![
            connect_api,
            disconnect_api,
            status_api,
            is_connected_api,
            // get_mode_api,
            // set_mode_api,
            // register_account_api,
            // delete_account_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}