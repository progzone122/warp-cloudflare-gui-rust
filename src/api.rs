use std::borrow::Cow;
use std::process::{Command, Output};
use std::result;

#[derive(Clone)]
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
        }

        "Unexpected error".to_string()
    }
    pub fn is_connected(&self) -> bool {
        let status: String = self.status();
        if status == "Connecting" {
            return true;
        }
        false
    }
}