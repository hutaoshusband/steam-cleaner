// src/core/mac_spoofer.rs

use std::process::Command;
use rand::Rng;
use winreg::enums::*;
use winreg::RegKey;

fn generate_random_mac() -> String {
    let mut rng = rand::thread_rng();
    let mut mac_bytes: [u8; 6] = [0; 6];
    mac_bytes[0] = (rng.gen_range(0x00..=0xFF) & 0xFE) | 0x02;
    for i in 1..6 {
        mac_bytes[i] = rng.gen_range(0x00..=0xFF);
    }
    format!("{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}", 
            mac_bytes[0], mac_bytes[1], mac_bytes[2], 
            mac_bytes[3], mac_bytes[4], mac_bytes[5])
}

pub fn spoof_mac_all(dry_run: bool) -> std::io::Result<Vec<String>> {
    let mut logs = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let adapters_key_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";
    let adapters_key = hklm.open_subkey(adapters_key_path)?;

    let mut spoofed_count = 0;

    for res in adapters_key.enum_keys() {
        let sub_key_name = res?;
        if sub_key_name == "Properties" {
            continue;
        }

        let current_adapter_path = format!(r"{}\{}", adapters_key_path, sub_key_name);
        let current_adapter_key = match hklm.open_subkey(&current_adapter_path) {
            Ok(key) => key,
            Err(_) => continue,
        };

        let driver_desc: String = match current_adapter_key.get_value("DriverDesc") {
            Ok(val) => val,
            Err(_) => continue,
        };

        let lc_desc = driver_desc.to_lowercase();
        let blacklist = ["wan miniport", "tunnel", "ppoe", "loopback", "ras async", "virtual", "teredo", "pseudo"];
        if blacklist.iter().any(|b| lc_desc.contains(b)) {
            logs.push(format!("[–] Skipped: '{}'", driver_desc));
            continue;
        }

        logs.push(format!("[*] Adapter detected: '{}' (Key: {})", driver_desc, sub_key_name));

        let new_mac = generate_random_mac();
        logs.push(format!("    → New MAC: {}", new_mac));

        if dry_run {
            spoofed_count += 1;
            continue;
        }

        let adapter_full_path = format!(
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\{{4d36e972-e325-11ce-bfc1-08002be10318}}\{}",
            sub_key_name
        );

        let reg_result = Command::new("reg")
            .args(["add", &adapter_full_path, "/v", "NetworkAddress", "/d", &new_mac, "/f"])
            .status();

        match reg_result {
            Ok(status) if status.success() => {
                logs.push(format!("[+] Successfully changed: '{}'", driver_desc));
                spoofed_count += 1;
            }
            Ok(status) => {
                logs.push(format!(
                    "[-] Error with '{}'. Exit Code: {}",
                    driver_desc,
                    status.code().unwrap_or(-1)
                ));
            }
            Err(e) => {
                logs.push(format!("[-] Execution error with '{}': {}", driver_desc, e));
            }
        }
    }

    if spoofed_count == 0 {
        logs.push("[-] No suitable adapters found or changed.".to_string());
    } else {
        logs.push(format!("[✓] MAC spoofing complete. {} adapters changed. Reboot needed!", spoofed_count));
    }

    Ok(logs)
}
