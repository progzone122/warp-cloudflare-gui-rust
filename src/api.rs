use std::borrow::Cow;
use std::process::{Command, Output};

#[derive(Clone, Copy)]
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