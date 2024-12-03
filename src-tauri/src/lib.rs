use std::borrow::Cow;
use std::process::{Command, Output};
use std::sync::Mutex;
use tauri::State;

#[derive(Clone, Copy, Debug)]
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Api {}
    }
    pub fn connect(&self) -> bool {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli connect")
            .output()
            .expect("failed to execute process");
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stdout : &str = stdout.trim_end_matches("\n");
            if stdout == "Success" {
                return true;
            }
        }
        false
    }
    pub fn disconnect(&self) -> bool {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli disconnect")
            .output()
            .expect("failed to execute process");
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stdout : &str = stdout.trim_end_matches("\n");
            if stdout == "Success" {
                return true;
            }
        }
        false
    }
    pub fn status(&self) -> String {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli status")
            .output()
            .expect("failed to execute process");

        if result.status.success() {
            let stdout: Cow<str> = String::from_utf8_lossy(&result.stdout);
            let parts: Vec<&str> = stdout.split(':').collect();

            if parts.len() > 2 {
                let lines: Vec<&str> = parts[2].split('\n').collect();
                return lines[0].trim().to_string();
            } else if parts.len() > 1 {
                let lines: Vec<&str> = parts[1].split('\n').collect();
                return lines[0].trim().to_string();
            }
        } else {
            let stderr: Cow<str> = String::from_utf8_lossy(&result.stderr);
            return stderr.to_string();
        }

        "Unexpected error".to_string()
    }
    pub fn is_connected(&self) -> bool {
        let status: String = self.status();
        if status == "Connecting" || status == "Connected" {
            return true;
        }
        false
    }

    pub fn get_mode(&self) -> i32 {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli settings")
            .output()
            .expect("failed to execute process");

        if result.status.success() {
            let stdout: Cow<str> = String::from_utf8_lossy(&result.stdout);
            let parts: Vec<&str> = stdout.split('\n').collect();
            let parts: Vec<&str> = parts[3].split(':').collect();
            let mode: &str = parts[1].trim();

            match mode {
                "Warp" => return 0, // warp
                "DnsOverHttps" => return 1, // https
                "WarpWithDnsOverHttps" => return 2, // warp+doh
                "DnsOverTls" => return 3, // dot
                "WarpWithDnsOverTls" => 4, // warp+dot
                "WarpProxy on port 40000" => 5, // proxy
                "TunnelOnly" => 6, // tunnel_only
                _ => return 0
            }
        } else {
            0
        }
    }
    pub fn set_mode(&self, mode: &str) {
        Command::new("sh")
            .arg("-c")
            .arg(format!("warp-cli set-mode {}", mode))
            .output()
            .expect("failed to execute process");
    }

    pub fn register_account(&self) -> Result<(), String> {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli registration new")
            .output()
            .expect("failed to execute process");

        return if result.status.success() {
            let result: Cow<str> = String::from_utf8_lossy(&result.stdout);
            let parts: Vec<&str> = result.split("\n").collect();

            if parts[0] == "Success" {
                Ok(())
            } else {
                Err(result.to_string())
            }
        } else {
            let error: Cow<str> = String::from_utf8_lossy(&result.stderr);
            Err(error.to_string())
        }
    }
    pub fn delete_account(&self) -> Result<(), String> {
        let result: Output = Command::new("sh")
            .arg("-c")
            .arg("warp-cli registration delete")
            .output()
            .expect("failed to execute process");

        return if result.status.success() {
            let result: Cow<str> = String::from_utf8_lossy(&result.stdout);
            let parts: Vec<&str> = result.split("\n").collect();

            if parts[0] == "Success" {
                Ok(())
            } else {
                Err(result.to_string())
            }
        } else {
            let error: Cow<str> = String::from_utf8_lossy(&result.stderr);
            Err(error.to_string())
        }
    }
}

#[tauri::command]
fn connect_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.connect()
}

#[tauri::command]
fn disconnect_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.disconnect()
}

#[tauri::command]
fn status_api(api: State<Mutex<Api>>) -> String {
    let api = api.lock().unwrap();
    api.status()
}

#[tauri::command]
fn is_connected_api(api: State<Mutex<Api>>) -> bool {
    let api = api.lock().unwrap();
    api.is_connected()
}

#[tauri::command]
fn get_mode_api(api: State<Mutex<Api>>) -> i32 {
    let api = api.lock().unwrap();
    api.get_mode()
}

#[tauri::command]
fn set_mode_api(api: State<Mutex<Api>>, mode: String) {
    let api = api.lock().unwrap();
    api.set_mode(&mode);
}

#[tauri::command]
fn register_account_api(api: State<Mutex<Api>>) -> Result<(), String> {
    let api = api.lock().unwrap();
    api.register_account()
}

#[tauri::command]
fn delete_account_api(api: State<Mutex<Api>>) -> Result<(), String> {
    let api = api.lock().unwrap();
    api.delete_account()
}

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
            get_mode_api,
            set_mode_api,
            register_account_api,
            delete_account_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}