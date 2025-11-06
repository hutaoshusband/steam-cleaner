// src/core/sid_spoofer.rs

use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn spoof_hkcu(dry_run: bool) -> io::Result<Vec<String>> {
    let mut logs = Vec::new();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let suspicious_keys = vec![
        "Software\\Valve",
        "Software\\FaceIt",
        "Software\\Faceit Ltd",
        "Software\\Riot Games",
        "Software\\ESEA",
        "Software\\EasyAntiCheat",
        "Software\\Battleye",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
    ];

    for key_path in suspicious_keys {
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key_path));
            continue;
        }

        match hkcu.delete_subkey_all(key_path) {
            Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key_path)),
            Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key_path, e)),
        }
    }

    logs.push("[✓] HKCU cleanup complete.".to_string());
    Ok(logs)
}
