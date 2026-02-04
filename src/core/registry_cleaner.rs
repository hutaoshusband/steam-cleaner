// src/core/registry_cleaner.rs

use crate::core::file_cleaner::kill_process;
use rand::Rng;
use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_WRITE};
use winreg::RegKey;

pub fn clean_registry(dry_run: bool) -> io::Result<Vec<String>> {
    let mut logs = Vec::new();

    let actions: Vec<fn(bool, &mut Vec<String>) -> io::Result<()>> = vec![
        spoof_machine_guid,
        spoof_hw_profile_guid,
        spoof_windows_nt_info,
        delete_steam_registry,
        spoof_computer_name,
    ];

    for action in actions {
        action(dry_run, &mut logs)?;
    }

    if !dry_run {
        logs.push("[*] Restarting explorer.exe...".to_string());
        match Command::new("explorer").spawn() {
            Ok(_) => logs.push("[+] explorer.exe started.".to_string()),
            Err(e) => logs.push(format!("[-] Failed to start explorer.exe: {}", e)),
        }
    }

    Ok(logs)
}

pub fn clean_aggressive_registry(dry_run: bool) -> io::Result<Vec<String>> {
    let mut logs = Vec::new();

    let aggressive_actions: Vec<fn(bool, &mut Vec<String>) -> io::Result<()>> =
        vec![delete_more_valve_keys, clean_system_caches];

    for action in aggressive_actions {
        action(dry_run, &mut logs)?;
    }

    Ok(logs)
}

pub fn spoof_machine_guid(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let new_guid = Uuid::new_v4().to_string();
    let message = format!("[Registry] Would spoof MachineGuid to: {}", new_guid);
    if dry_run {
        logs.push(message);
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let crypto = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_WRITE)?;
    crypto.set_value("MachineGuid", &new_guid)?;
    logs.push(format!("[Registry] MachineGuid spoofed: {}", new_guid));
    Ok(())
}

pub fn spoof_hw_profile_guid(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let new_hw_guid = format!("{{{}}}", Uuid::new_v4());
    let message = format!("[Registry] Would spoof HwProfileGuid to: {}", new_hw_guid);
    if dry_run {
        logs.push(message);
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let profile = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\IDConfigDB\\Hardware Profiles\\0001",
        KEY_WRITE,
    )?;
    profile.set_value("HwProfileGuid", &new_hw_guid)?;
    logs.push(format!("[Registry] HwProfileGuid spoofed: {}", new_hw_guid));
    Ok(())
}

pub fn spoof_windows_nt_info(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let product_id = format!(
        "{}-{}-{}-{}",
        rand_digits(5),
        rand_digits(3),
        rand_digits(7),
        rand_digits(5)
    );
    let owner = rand_string(8);
    let org = rand_string(10);
    let install_date: u32 = rand::thread_rng().gen_range(1_600_000_000..1_700_000_000);

    if dry_run {
        logs.push(format!(
            "[Registry] Would spoof ProductId to: {}",
            product_id
        ));
        logs.push(format!(
            "[Registry] Would set RegisteredOwner to: {}",
            owner
        ));
        logs.push(format!(
            "[Registry] Would set RegisteredOrganization to: {}",
            org
        ));
        logs.push(format!(
            "[Registry] Would spoof InstallDate to: {}",
            install_date
        ));
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version =
        hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", KEY_WRITE)?;

    current_version.set_value("ProductId", &product_id)?;
    logs.push(format!("[Registry] ProductId spoofed: {}", product_id));

    current_version.set_value("RegisteredOwner", &owner)?;
    current_version.set_value("RegisteredOrganization", &org)?;
    logs.push(format!(
        "[Registry] RegisteredOwner: {}, RegisteredOrganization: {}",
        owner, org
    ));

    current_version.set_value("InstallDate", &install_date)?;
    logs.push(format!("[Registry] InstallDate spoofed: {}", install_date));

    Ok(())
}

pub fn delete_steam_registry(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let processes = [
        "steam.exe",
        "steamwebhelper.exe",
        "GameOverlayUI.exe",
        "steamerrorreporter.exe",
    ];

    for proc in processes.iter() {
        kill_process(proc, dry_run, logs);
    }
    kill_process("explorer.exe", dry_run, logs);

    if !dry_run {
        sleep(Duration::from_secs(10));
    }

    let key_path = "Software\\Valve\\Steam";
    if dry_run {
        logs.push(format!(
            "[Registry] Would delete Steam registry key: {}",
            key_path
        ));
        return Ok(());
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.delete_subkey_all(key_path) {
        Ok(_) => logs.push(format!(
            "[Registry] Deleted Steam registry key: {}",
            key_path
        )),
        Err(_) => logs.push(format!(
            "[Registry] Steam registry key not found or could not be deleted: {}",
            key_path
        )),
    }

    Ok(())
}

pub fn spoof_registered_owner(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let owner = rand_string(8);
    let org = rand_string(10);

    if dry_run {
        logs.push(format!(
            "[Registry] Would set RegisteredOwner to: {}",
            owner
        ));
        logs.push(format!(
            "[Registry] Would set RegisteredOrganization to: {}",
            org
        ));
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version =
        hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", KEY_WRITE)?;

    current_version.set_value("RegisteredOwner", &owner)?;
    current_version.set_value("RegisteredOrganization", &org)?;
    logs.push(format!(
        "[Registry] RegisteredOwner: {}, RegisteredOrganization: {}",
        owner, org
    ));

    Ok(())
}

pub fn spoof_install_date(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let install_date: u32 = rand::thread_rng().gen_range(1_600_000_000..1_700_000_000);

    if dry_run {
        logs.push(format!(
            "[Registry] Would spoof InstallDate to: {}",
            install_date
        ));
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version =
        hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", KEY_WRITE)?;

    current_version.set_value("InstallDate", &install_date)?;
    logs.push(format!("[Registry] InstallDate spoofed: {}", install_date));

    Ok(())
}

pub fn spoof_computer_name(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let new_name = rand_string(10);
    if dry_run {
        logs.push(format!("[Registry] Would spoof Hostname to: {}", new_name));
        return Ok(());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let tcpip_params = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters",
        KEY_WRITE,
    )?;
    tcpip_params.set_value("Hostname", &new_name)?;
    tcpip_params.set_value("NV Hostname", &new_name)?;
    logs.push(format!("[Registry] Computer name spoofed: {}", new_name));

    Ok(())
}

pub fn delete_more_valve_keys(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hku = RegKey::predef(HKEY_USERS);

    let paths_to_delete = vec![
        "SOFTWARE\\Valve".to_string(),
        "SOFTWARE\\Wow6432Node\\Valve".to_string(),
    ];

    for path in paths_to_delete {
        if dry_run {
            logs.push(format!("[Registry] Would delete HKLM\\{}", path));
        } else {
            if let Err(e) = hklm.delete_subkey_all(&path) {
                logs.push(format!("[Registry] Could not delete HKLM\\{}: {}", path, e));
            } else {
                logs.push(format!("[Registry] Deleted HKLM\\{}", path));
            }
        }
    }

    for sid in hku.enum_keys().filter_map(Result::ok) {
        let path = format!("{}\\{}", sid, "Software\\Valve\\Steam");
        if dry_run {
            logs.push(format!("[Registry] Would delete HKU\\{}", path));
        } else {
            if let Err(e) = hku.delete_subkey_all(&path) {
                logs.push(format!("[Registry] Could not delete HKU\\{}: {}", path, e));
            } else {
                logs.push(format!("[Registry] Deleted HKU\\{}", path));
            }
        }
    }

    Ok(())
}

pub fn clean_system_caches(dry_run: bool, logs: &mut Vec<String>) -> io::Result<()> {
    let paths_to_delete = vec![
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\AppCompatCache",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\ShimCache",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags",
    ];

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    for path in paths_to_delete {
        if dry_run {
            logs.push(format!("[Registry] Would delete HKLM\\{}", path));
        } else {
            if let Err(e) = hklm.delete_subkey_all(&path) {
                logs.push(format!("[Registry] Could not delete HKLM\\{}: {}", path, e));
            } else {
                logs.push(format!("[Registry] Deleted HKLM\\{}", path));
            }
        }
    }

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
