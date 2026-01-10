// src/core/volumeid_wrapper.rs

use std::process::Command;
use std::path::Path;
use rand::Rng;
use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn change_volume_id(drive_letter: &str, dry_run: bool) -> io::Result<String> {
    let new_id = generate_random_volume_id();
    
    if dry_run {
        return Ok(format!("[Volume ID] Would change Volume ID of {} to {}", drive_letter, new_id));
    }

    accept_volumeid_eula()?;

    let tool_paths = vec![
        Path::new("volumeid64.exe"),
        Path::new("volumeid.exe"),
        Path::new("./tools/volumeid64.exe"),
        Path::new("./tools/volumeid.exe"),
    ];

    let volumeid_path = tool_paths
        .into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No working volumeid(.exe|64.exe) found.",
        ))?;

    let output = Command::new(volumeid_path)
        .args([&format!("{}:", drive_letter), &new_id])
        .output()?; 

    if output.status.success() {
        Ok(format!("[+] Volume ID changed to {} for {}", new_id, drive_letter))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let error_message = format!(
            "[!] Failed to change Volume ID:\n    Stdout: {}\n    Stderr: {}\n    Exit Code: {:?}",
            stdout, stderr, output.status.code()
        );
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        ))
    }
}

fn generate_random_volume_id() -> String {
    let mut rng = rand::thread_rng();
    let part1 = rng.gen_range(0x0000..=0xFFFF);
    let part2 = rng.gen_range(0x0000..=0xFFFF);
    format!("{:04X}-{:04X}", part1, part2)
}

fn accept_volumeid_eula() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // The key path is "Software\Sysinternals\VolumeId"
    // We need to create the subkey "Software\Sysinternals" first
    let sysinternals_path = "Software\\Sysinternals";
    let (sysinternals_key, _) = hkcu.create_subkey(sysinternals_path)?;
    let (key, _) = sysinternals_key.create_subkey("VolumeId")?;
    key.set_value("EulaAccepted", &1u32)?;
    Ok(())
}

/// Wendet eine spezifische Volume ID aus einem Profil an
pub fn change_volume_id_to_specific(drive_letter: &str, volume_id: &str, dry_run: bool) -> io::Result<String> {
    if dry_run {
        return Ok(format!("[Volume ID] Would change Volume ID of {} to {}", drive_letter, volume_id));
    }

    accept_volumeid_eula()?;

    let tool_paths = vec![
        Path::new("volumeid64.exe"),
        Path::new("volumeid.exe"),
        Path::new("./tools/volumeid64.exe"),
        Path::new("./tools/volumeid.exe"),
    ];

    let volumeid_path = tool_paths
        .into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No working volumeid(.exe|64.exe) found.",
        ))?;

    let output = Command::new(volumeid_path)
        .args([&format!("{}:", drive_letter), volume_id])
        .output()?; 

    if output.status.success() {
        Ok(format!("[+] Volume ID changed to {} for {} (from profile)", volume_id, drive_letter))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let error_message = format!(
            "[!] Failed to change Volume ID:\n    Stdout: {}\n    Stderr: {}\n    Exit Code: {:?}",
            stdout, stderr, output.status.code()
        );
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        ))
    }
}

