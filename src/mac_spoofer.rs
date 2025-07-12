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


fn find_network_adapter_key(search_term: &str) -> std::io::Result<Option<String>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let adapters_key_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";

    let adapters_key = hklm.open_subkey(adapters_key_path)?;

    for res in adapters_key.enum_keys() {
        let sub_key_name = res?;
        if sub_key_name == "Properties" { continue; } 

        let current_adapter_path = format!(r"{}\{}", adapters_key_path, sub_key_name);
        let current_adapter_key = hklm.open_subkey(&current_adapter_path)?;

        let driver_desc: String = match current_adapter_key.get_value("DriverDesc") {
            Ok(desc) => desc,
            Err(_) => continue, 
        };

        println!("[*] Prüfe Adapter: '{}' (Schlüssel: {})", driver_desc, sub_key_name);

        if driver_desc.to_lowercase().contains(&search_term.to_lowercase()) {
            println!("[+] Passender Adapter gefunden: '{}'", driver_desc);
            return Ok(Some(sub_key_name));
        }
    }
    Ok(None)
}


pub fn spoof_mac_all() -> std::io::Result<()> {
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
            println!("[–] Übersprungen: '{}'", driver_desc);
            continue;
        }

        println!("[*] Adapter erkannt: '{}' (Schlüssel: {})", driver_desc, sub_key_name);

        let new_mac = generate_random_mac();
        println!("    → Neue MAC: {}", new_mac);

        let adapter_full_path = format!(
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\{{4d36e972-e325-11ce-bfc1-08002be10318}}\{}",
            sub_key_name
        );

        let reg_result = Command::new("reg")
            .args(["add", &adapter_full_path, "/v", "NetworkAddress", "/d", &new_mac, "/f"])
            .status();

        match reg_result {
            Ok(status) if status.success() => {
                println!("[+] Erfolgreich geändert: '{}'", driver_desc);
                spoofed_count += 1;
            }
            Ok(status) => {
                eprintln!(
                    "[-] Fehler bei '{}'. Exit Code: {}",
                    driver_desc,
                    status.code().unwrap_or(-1)
                );
            }
            Err(e) => {
                eprintln!("[-] Ausführungsfehler bei '{}': {}", driver_desc, e);
            }
        }
    }

    if spoofed_count == 0 {
        eprintln!("[-] Keine geeigneten Adapter gefunden oder geändert.");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No adapters spoofed"));
    }

    println!("[✓] MAC-Spoofing abgeschlossen. {} Adapter geändert. Reboot nötig!", spoofed_count);
    Ok(())
}