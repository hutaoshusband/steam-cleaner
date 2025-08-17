use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn spoof_hkcu() -> io::Result<()> {
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
        match hkcu.delete_subkey_all(key_path) {
            Ok(_) => println!("[+] Deleted HKCU\\{}", key_path),
            Err(e) => println!("[-] Could not delete HKCU\\{}: {}", key_path, e),
        }
    }

    println!("[✓] HKCU cleanup complete.");
    Ok(())
}
