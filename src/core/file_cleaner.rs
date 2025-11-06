// src/core/file_cleaner.rs

use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io;

pub fn kill_process(process_name: &str, dry_run: bool, logs: &mut Vec<String>) {
    let message = format!("[Process] Would terminate: {}", process_name);
    if dry_run {
        logs.push(message);
        return;
    }

    logs.push(format!("[Process] Terminating: {}", process_name));
    match Command::new("taskkill")
        .args(["/F", "/IM", process_name])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                logs.push(format!("[Process] Terminated: {}", process_name));
            } else {
                logs.push(format!(
                    "[Process] Failed to terminate {}: {}",
                    process_name,
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Err(e) => logs.push(format!("[Process] Error terminating {}: {}", process_name, e)),
    }
}

fn try_delete_dir_contents(path: &str, dry_run: bool, logs: &mut Vec<String>) {
    let dir = Path::new(path);
    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let message = format!("[Directory] Would delete contents of: {}", entry_path.display());
                if dry_run {
                    logs.push(message);
                    continue;
                }
                
                let _ = Command::new("attrib")
                    .args(&["-R", "-S", "-H", entry_path.to_str().unwrap_or("")])
                    .output();
                let result = if entry_path.is_dir() {
                    fs::remove_dir_all(&entry_path)
                } else {
                    fs::remove_file(&entry_path)
                };
                match result {
                    Ok(_) => logs.push(format!("[Directory] Deleted contents of: {}", entry_path.display())),
                    Err(e) => logs.push(format!("[Directory] Failed to delete {}: {}", entry_path.display(), e)),
                }
            }
        }
    } else {
        logs.push(format!("[Directory] Not found or not a directory: {}", path));
    }
}

fn try_delete(path: &str, dry_run: bool, logs: &mut Vec<String>) {
    let p = Path::new(path);
    if p.exists() {
        let message = format!("[File/Dir] Would delete: {}", path);
        if dry_run {
            logs.push(message);
            return;
        }

        let _ = Command::new("attrib")
            .args(&["-R", "-S", "-H", path])
            .output();
        let result = if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            fs::remove_file(p)
        };
        match result {
            Ok(_) => logs.push(format!("[File/Dir] Deleted: {}", path)),
            Err(e) => logs.push(format!("[File/Dir] Failed to delete {}: {}", path, e)),
        }
    } else {
        logs.push(format!("[File/Dir] Not found: {}", path));
    }
}

pub fn clean_cache(dry_run: bool) -> io::Result<Vec<String>> {
    let mut logs = Vec::new();

    let processes = [
        "steam.exe",
        "steamwebhelper.exe",
        "GameOverlayUI.exe",
        "steamerrorreporter.exe",
    ];
    for proc in processes.iter() {
        kill_process(proc, dry_run, &mut logs);
        kill_process("explorer.exe", dry_run, &mut logs);
    }

    if !dry_run {
        logs.push("Waiting 10 seconds to ensure processes are closed...".to_string());
        sleep(Duration::from_secs(10));
    }

    let local = std::env::var("LOCALAPPDATA")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let appdata = std::env::var("APPDATA")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let userprofile = std::env::var("USERPROFILE")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let locallow = local.replace("Local", "LocalLow");

    let default_path = format!("{}\\Steam", local);
    let possible_steam_paths = vec![
        "C:\\Program Files (x86)\\Steam".to_string(),
        "C:\\Program Files\\Steam".to_string(),
        default_path.clone(),
    ];
    let steam_root = possible_steam_paths
        .iter()
        .find(|p| Path::new(p).exists())
        .unwrap_or(&default_path)
        .to_string();

    let steam_config = format!("{}\\config", steam_root);

    let login_files = [
        "loginusers.vdf",
        "config.vdf",
        "localconfig.vdf",
        "SteamAppData.vdf",
    ];

    for file in &login_files {
        let path = format!("{}\\{}", steam_config, file);
        if Path::new(&path).exists() {
            try_delete(&path, dry_run, &mut logs);
        } else {
            logs.push(format!("Steam file not found: {}", path));
        }
    }

    if let Ok(entries) = fs::read_dir(&steam_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("ssfn") {
                    try_delete(path.to_str().unwrap_or_default(), dry_run, &mut logs);
                }
            }
        }
    }

    let harmless_files = [
        format!("{}\\steamapps\\libraryfolders.vdf", steam_root),
        format!("{}\\steamapps\\appmanifest_*.acf", steam_root), // Vorsicht!
    ];
    let paths = vec![
        format!("{}\\steamapps\\libraryfolders.vdf", steam_root),
        format!("{}\\userdata", steam_root),
        steam_config.clone(),
        format!("{}\\logs", steam_root),
        format!("{}\\appcache", steam_root),
        format!("{}\\dump", steam_root),
        format!("{}\\shadercache", steam_root),
        format!("{}\\appcache\\shadercache", steam_root),
        format!("{}\\Steam", appdata),
        format!("{}\\D3DSCache", local),
        format!("{}\\Temp", local),
        "C:\\ProgramData\\NVIDIA Corporation\\NV_Cache".to_string(),
        format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent", userprofile),
        format!("{}\\AppData\\Local\\CrashDumps", userprofile),
        format!("{}\\AppData\\LocalLow\\Temp", userprofile),
        format!("{}\\AppData\\Local\\Microsoft\\Windows\\WebCache", userprofile),
        format!("{}\\AppData\\Local\\Microsoft\\Windows\\INetCache", userprofile),
        format!("{}\\Tracing", userprofile),
        format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations", userprofile),
        format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations", userprofile),
        format!("{}\\AppData\\Local\\Temp", userprofile),
        format!("{}\\AppData\\Local\\Packages", userprofile),
        format!("{}\\AppData\\Local\\Microsoft\\Windows\\Caches", userprofile),
        format!("{}\\AppData\\Local\\Microsoft\\Windows\\Explorer", userprofile),
        // New paths for intensified cleaning
        format!("{}\\depotcache", steam_root),
        format!("{}\\Steam\\htmlcache", local),
        format!("{}\\Steam\\cefdata", local),
        format!("{}\\Steam\\logged_in_user", local),
        format!("{}\\Valve\\Steam", locallow),
        // Deep cleaning paths
        "C:\\Windows\\Prefetch".to_string(),
        format!("{}\\Documents\\My Games", userprofile),
        "C:\\ProgramData\\EasyAntiCheat".to_string(),
        "C:\\ProgramData\\BattlEye".to_string(),
        "C:\\ProgramData\\Faceit".to_string(),
    ];

    for path in &paths {
        try_delete(path, dry_run, &mut logs);
    }
    for file in harmless_files.iter() {
        try_delete(file, dry_run, &mut logs);
    }
    
    try_delete_dir_contents(&format!("{}\\Temp", local), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Temp", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\LocalLow\\Temp", userprofile), dry_run, &mut logs);

    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\INetCache", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\WebCache", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\Caches", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\Explorer", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\CrashDumps", userprofile), dry_run, &mut logs);

    try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations", userprofile), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations", userprofile), dry_run, &mut logs);

    try_delete_dir_contents(&format!("{}\\Tracing", userprofile), dry_run, &mut logs);
    try_delete_dir_contents("C:\\Windows\\Temp", dry_run, &mut logs);
    try_delete_dir_contents("C:\\ProgramData\\Microsoft\\Windows\\Caches", dry_run, &mut logs);
    try_delete_dir_contents("C:\\ProgramData\\NVIDIA Corporation\\NV_Cache", dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\appcache", steam_root), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\logs", steam_root), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\dump", steam_root), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\shadercache", steam_root), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\userdata", steam_root), dry_run, &mut logs);
    try_delete_dir_contents(&format!("{}\\config", steam_root), dry_run, &mut logs);
    
    if !dry_run {
        let _ = Command::new("explorer").spawn();
        logs.push("Cache cleanup complete.".to_string());
    }

    Ok(logs)
}
