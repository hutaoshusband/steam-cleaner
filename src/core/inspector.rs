// src/core/inspector.rs

use std::process::Command;
use winreg::enums::{HKEY_LOCAL_MACHINE};
use winreg::RegKey;
use std::path::{PathBuf};
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct SystemInfo {
    pub machine_guid: String,
    pub product_id: String,
    pub computer_name: String,
    pub volume_id: String,
    pub network_adapters: Vec<(String, String)>,
    pub steam_login_files: Vec<String>,
}

fn find_steam_root() -> Option<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
    let possible_paths = [
        PathBuf::from("C:\\Program Files (x86)\\Steam"),
        PathBuf::from("C:\\Program Files\\Steam"),
        PathBuf::from(local_app_data).join("Steam"),
    ];

    possible_paths.into_iter().find(|p| p.exists())
}


pub async fn gather_system_info() -> SystemInfo {
    let mut info = SystemInfo::default();

    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Cryptography") {
        info.machine_guid = hklm.get_value("MachineGuid").unwrap_or_else(|_| "Not found".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        info.product_id = hklm.get_value("ProductId").unwrap_or_else(|_| "Not found".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters") {
        info.computer_name = hklm.get_value("Hostname").unwrap_or_else(|_| "Not found".to_string());
    }

    if let Ok(output) = Command::new("cmd").args(["/C", "vol C:"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = output_str.lines().find(|l| l.contains("Volume Serial Number")) {
            info.volume_id = line.split_at(23).1.trim().to_string();
        }
    }

    if let Ok(output) = Command::new("getmac").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(3) {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if let Some(mac) = columns.get(0) {
                info.network_adapters.push(("Network Adapter".to_string(), mac.to_string()));
            }
        }
    }

    if let Some(steam_root) = find_steam_root() {
        let config_path = steam_root.join("config").join("loginusers.vdf");
        if config_path.exists() {
            info.steam_login_files.push(config_path.to_string_lossy().to_string());
        }

        if let Ok(entries) = fs::read_dir(&steam_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("ssfn") {
                        info.steam_login_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }


    info
}
