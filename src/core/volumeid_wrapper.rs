use std::process::Command;
use std::path::Path;
use rand::Rng;
use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub fn change_volume_id(drive_letter: &str) -> std::io::Result<()> {
    accept_volumeid_eula()?;

    let new_id = generate_random_volume_id();
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

    println!("[*] Trying to change the Volume ID of {} to {}", drive_letter, new_id);

    let output = Command::new(volumeid_path)
        .args([&format!("{}:", drive_letter), &new_id])
        .output()?; 

    if output.status.success() {
        println!("[+] Volume ID changed to {} for {}", new_id, drive_letter);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!("[!] Failed to change Volume ID:");
        if !stdout.is_empty() {
            eprintln!("    Stdout: {}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("    Stderr: {}", stderr);
        }
        eprintln!("    Exit Code: {:?}", output.status.code());

        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "VolumeId change failed.",
        ));
    }

    Ok(())
}

fn generate_random_volume_id() -> String {
    let mut rng = rand::thread_rng();
    let part1 = rng.gen_range(0x0000..=0xFFFF);
    let part2 = rng.gen_range(0x0000..=0xFFFF);
    format!("{:04X}-{:04X}", part1, part2)
}

fn accept_volumeid_eula() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Ss";
    let (key, _) = hkcu.create_subkey(path)?;
    key.set_value("EulaAccepted", &1u32)?;
    Ok(())
}