use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE, HKEY_CURRENT_USER};
use winreg::RegKey;
use uuid::Uuid;
use rand::Rng;
use std::io;
use std::time::Duration;
use std::thread::sleep;
use super::file_cleaner::kill_process; // 'super' geht von registry_cleaner.rs zu core/mod.rs
use std::process::Command; 


pub fn clean_registry() -> io::Result<()> {
    let actions: Vec<fn() -> io::Result<()>> = vec![
        spoof_machine_guid,
        spoof_hw_profile_guid,
        spoof_windows_nt_info,
        delete_steam_registry,
        spoof_computer_name,
    ];

    for (i, action) in actions.iter().enumerate() {
        println!("[*] Running registry action #{}...", i + 1);
        match action() {
            Ok(_) => println!("[+] Action #{} completed successfully.", i + 1),
            Err(e) => eprintln!("[-] Error in registry action #{}: {}", i + 1, e),
        }
    }

    println!("[*] All registry spoofing actions processed.");

    println!("[*] Restarting explorer.exe...");
    match Command::new("explorer").spawn() {
        Ok(_) => println!("[+] explorer.exe started."),
        Err(e) => eprintln!("[-] Failed to start explorer.exe: {}", e),
    }

    Ok(())
}

fn spoof_machine_guid() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let crypto = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_WRITE)?;
    let new_guid = Uuid::new_v4().to_string();
    crypto.set_value("MachineGuid", &new_guid)?;
    println!("[+] MachineGuid spoofed: {}", new_guid);
    Ok(())
}

fn spoof_hw_profile_guid() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let profile = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\IDConfigDB\\Hardware Profiles\\0001",
        KEY_WRITE,
    )?;
    let new_hw_guid = format!("{{{}}}", Uuid::new_v4());
    profile.set_value("HwProfileGuid", &new_hw_guid)?;
    println!("[+] HwProfileGuid spoofed: {}", new_hw_guid);
    Ok(())
}

fn spoof_windows_nt_info() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        KEY_WRITE,
    )?;

    let product_id = format!(
        "{}-{}-{}-{}",
        rand_digits(5),
        rand_digits(3),
        rand_digits(7),
        rand_digits(5)
    );
    current_version.set_value("ProductId", &product_id)?;
    println!("[+] ProductId spoofed: {}", product_id);

    let owner = rand_string(8);
    let org = rand_string(10);
    current_version.set_value("RegisteredOwner", &owner)?;
    current_version.set_value("RegisteredOrganization", &org)?;
    println!("[+] RegisteredOwner: {}, RegisteredOrganization: {}", owner, org);

    let install_date: u32 = rand::thread_rng().gen_range(1_600_000_000..1_700_000_000);
    current_version.set_value("InstallDate", &install_date)?;
    println!("[+] InstallDate spoofed: {}", install_date);

    Ok(())
}

fn delete_steam_registry() -> io::Result<()> {
    let processes = [
        "steam.exe",
        "steamwebhelper.exe",
        "GameOverlayUI.exe",
        "steamerrorreporter.exe",
    ];

    for proc in processes.iter() {
        kill_process(proc);
    }

    kill_process("explorer.exe");

    sleep(Duration::from_secs(10));

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key_path = "Software\\Valve\\Steam";

    match hkcu.delete_subkey_all(key_path) {
        Ok(_) => println!("[+] Deleted Steam registry key: {}", key_path),
        Err(_) => println!(
            "[-] Steam registry key not found or could not be deleted: {}",
            key_path
        ),
    }

    Ok(())
}


fn spoof_computer_name() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let tcpip_params = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters",
        KEY_WRITE,
    )?;

    let new_name = rand_string(10);
    tcpip_params.set_value("Hostname", &new_name)?;
    tcpip_params.set_value("NV Hostname", &new_name)?;
    println!("[+] Computer name spoofed: {}", new_name);

    Ok(())
}

fn rand_digits(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(0..10).to_string()).collect()
}

fn rand_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

