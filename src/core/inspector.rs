// src/core/inspector.rs

use std::process::Command;
use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER};
use winreg::RegKey;


#[derive(Debug, Clone, Default)]
pub struct SystemInfo {
    pub machine_guid: String,
    pub product_id: String,
    pub computer_name: String,
    pub volume_id: String,
    pub network_adapters: Vec<(String, String)>, 
}

pub async fn gather_system_info() -> SystemInfo {
    let mut info = SystemInfo::default();

    // 1. Registry-Werte auslesen
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Cryptography") {
        info.machine_guid = hklm.get_value("MachineGuid").unwrap_or_else(|_| "Nicht gefunden".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        info.product_id = hklm.get_value("ProductId").unwrap_or_else(|_| "Nicht gefunden".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters") {
        info.computer_name = hklm.get_value("Hostname").unwrap_or_else(|_| "Nicht gefunden".to_string());
    }

    if let Ok(output) = Command::new("cmd").args(["/C", "vol C:"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = output_str.lines().find(|l| l.contains("Volumeseriennummer")) {
            info.volume_id = line.split_at(23).1.trim().to_string();
        }
    }

    if let Ok(output) = Command::new("getmac").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(3) {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if columns.len() >= 2 {
                let mac = columns[0].to_string();
                info.network_adapters.push(("Netzwerkadapter".to_string(), mac));
            }
        }
    }

    info
}
