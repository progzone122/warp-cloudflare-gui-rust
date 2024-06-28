use std::borrow::Cow;
use std::process::{Command, Output};
use std::result;

#[derive(Clone)]
pub struct Api {
    pub status: String
}

impl Api {
    pub fn new() -> Self {
        Api {
            status: "".to_string()
        }
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
            let stdout: Vec<&str> = stdout.split(":").collect();
            let stdout: Vec<&str> = stdout[1].split("\n").collect();
            let stdout: &str = &stdout[0][1..];

            return stdout.to_string();
        }

        "Unexpected error".to_string()
    }
}