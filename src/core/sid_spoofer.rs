// src/core/sid_spoofer.rs

use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn spoof_hkcu(dry_run: bool) -> io::Result<Vec<String>> {
    spoof_hkcu_detailed(dry_run, true, true, true, true, true, true, &mut Vec::new())
        .map(|_| vec!["[✓] HKCU cleanup complete.".to_string()])
}

pub fn spoof_hkcu_detailed(
    dry_run: bool,
    delete_faceit: bool,
    delete_riot: bool,
    delete_esea: bool,
    delete_eac: bool,
    delete_battleye: bool,
    delete_run: bool,
    logs: &mut Vec<String>,
) -> io::Result<()> {
    let mut logs = Vec::new();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let suspicious_keys: Vec<&str> = vec![
        "Software\\FaceIt",
        "Software\\Faceit Ltd",
        "Software\\Riot Games",
        "Software\\ESEA",
        "Software\\EasyAntiCheat",
        "Software\\Battleye",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
    ];

    // Delete individual game tracking keys
    if delete_faceit {
        for key in ["Software\\FaceIt", "Software\\Faceit Ltd"] {
            if dry_run {
                logs.push(format!("[Registry] Would delete HKCU\\{}", key));
            } else {
                match hkcu.delete_subkey_all(key) {
                    Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                    Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
                }
            }
        }
    }

    if delete_riot {
        let key = "Software\\Riot Games";
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key));
        } else {
            match hkcu.delete_subkey_all(key) {
                Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
            }
        }
    }

    if delete_esea {
        let key = "Software\\ESEA";
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key));
        } else {
            match hkcu.delete_subkey_all(key) {
                Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
            }
        }
    }

    if delete_eac {
        let key = "Software\\EasyAntiCheat";
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key));
        } else {
            match hkcu.delete_subkey_all(key) {
                Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
            }
        }
    }

    if delete_battleye {
        let key = "Software\\Battleye";
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key));
        } else {
            match hkcu.delete_subkey_all(key) {
                Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
            }
        }
    }

    if delete_run {
        let key = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        if dry_run {
            logs.push(format!("[Registry] Would delete HKCU\\{}", key));
        } else {
            match hkcu.delete_subkey_all(key) {
                Ok(_) => logs.push(format!("[+] Deleted HKCU\\{}", key)),
                Err(e) => logs.push(format!("[-] Could not delete HKCU\\{}: {}", key, e)),
            }
        }
    }

    Ok(())
}
