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

// Split blocking operations into separate functions
fn gather_registry_info() -> (String, String, String) {
    let mut machine_guid = "Not found".to_string();
    let mut product_id = "Not found".to_string();
    let mut computer_name = "Not found".to_string();

    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Cryptography") {
        machine_guid = hklm.get_value("MachineGuid").unwrap_or_else(|_| "Not found".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        product_id = hklm.get_value("ProductId").unwrap_or_else(|_| "Not found".to_string());
    }
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters") {
        computer_name = hklm.get_value("Hostname").unwrap_or_else(|_| "Not found".to_string());
    }

    (machine_guid, product_id, computer_name)
}

fn gather_volume_info() -> String {
    match Command::new("cmd").args(["/C", "vol C:"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str
                .lines()
                .find(|l| l.contains("Volume Serial Number"))
                .map(|line| line.split_at(23).1.trim().to_string())
                .unwrap_or_else(|| "Not found".to_string())
        }
        Err(_) => "Not found".to_string(),
    }
}

fn gather_mac_addresses() -> Vec<(String, String)> {
    match Command::new("getmac").output() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut adapters = Vec::new();
            for line in output_str.lines().skip(3) {
                let columns: Vec<&str> = line.split_whitespace().collect();
                if let Some(mac) = columns.get(0) {
                    adapters.push(("Network Adapter".to_string(), mac.to_string()));
                }
            }
            adapters
        }
        Err(_) => Vec::new(),
    }
}

fn gather_steam_files() -> Vec<String> {
    let mut files = Vec::new();

    if let Some(steam_root) = find_steam_root() {
        let config_path = steam_root.join("config").join("loginusers.vdf");
        if config_path.exists() {
            files.push(config_path.to_string_lossy().to_string());
        }

        // Limit the scan to prevent hanging on large directories
        if let Ok(entries) = fs::read_dir(&steam_root) {
            let mut count = 0;
            const MAX_FILES: usize = 1000; // Safety limit

            for entry in entries.flatten() {
                count += 1;
                if count > MAX_FILES {
                    break;
                }

                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("ssfn") {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    files
}

pub async fn gather_system_info() -> SystemInfo {
    // Use tokio::task::spawn_blocking to run blocking operations in a separate thread pool
    let registry_task = tokio::task::spawn_blocking(gather_registry_info);
    let volume_task = tokio::task::spawn_blocking(gather_volume_info);
    let mac_task = tokio::task::spawn_blocking(gather_mac_addresses);
    let steam_task = tokio::task::spawn_blocking(gather_steam_files);

    // Wait for all tasks to complete
    let (machine_guid, product_id, computer_name) = registry_task.await.unwrap_or((
        "Not found".to_string(),
        "Not found".to_string(),
        "Not found".to_string(),
    ));

    let volume_id = volume_task.await.unwrap_or_else(|_| "Not found".to_string());
    let network_adapters = mac_task.await.unwrap_or_else(|_| Vec::new());
    let steam_login_files = steam_task.await.unwrap_or_else(|_| Vec::new());

    SystemInfo {
        machine_guid,
        product_id,
        computer_name,
        volume_id,
        network_adapters,
        steam_login_files,
    }
}
