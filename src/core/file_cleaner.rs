// src/core/file_cleaner.rs

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;

use crate::core::steam;

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
        Err(e) => logs.push(format!(
            "[Process] Error terminating {}: {}",
            process_name, e
        )),
    }
}

pub fn try_delete_dir_contents(path: &str, dry_run: bool, logs: &mut Vec<String>) {
    let dir = Path::new(path);
    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let message = format!(
                    "[Directory] Would delete contents of: {}",
                    entry_path.display()
                );
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
                    Ok(_) => logs.push(format!(
                        "[Directory] Deleted contents of: {}",
                        entry_path.display()
                    )),
                    Err(e) => logs.push(format!(
                        "[Directory] Failed to delete {}: {}",
                        entry_path.display(),
                        e
                    )),
                }
            }
        }
    } else {
        logs.push(format!(
            "[Directory] Not found or not a directory: {}",
            path
        ));
    }
}

pub fn try_delete(path: &str, dry_run: bool, logs: &mut Vec<String>) {
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

fn extract_installdir(manifest_contents: &str, regex: &Regex) -> Option<String> {
    regex
        .captures(manifest_contents)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn collect_manifest_install_dirs(library_path: &Path) -> (HashSet<String>, usize) {
    let mut install_dirs = HashSet::new();
    let mut manifest_count = 0;
    let regex = match Regex::new(r#"(?i)\"installdir\"\s+\"([^\"]+)\""#) {
        Ok(regex) => regex,
        Err(_) => return (install_dirs, manifest_count),
    };

    if let Ok(entries) = fs::read_dir(library_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };
            if !file_name.starts_with("appmanifest_") || !file_name.ends_with(".acf") {
                continue;
            }

            manifest_count += 1;
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Some(installdir) = extract_installdir(&contents, &regex) {
                    install_dirs.insert(installdir.to_lowercase());
                }
            }
        }
    }

    (install_dirs, manifest_count)
}

fn clean_orphaned_game_folders(steam_root: &str, dry_run: bool, logs: &mut Vec<String>) {
    let steam_root_path = Path::new(steam_root);
    if !steam_root_path.exists() {
        logs.push(format!(
            "[Steam] Steam root not found at {}. Skipping orphaned folder scan.",
            steam_root
        ));
        return;
    }

    let libraries = steam::get_library_folders(steam_root_path);
    if libraries.is_empty() {
        logs.push("[Steam] No Steam libraries found. Skipping orphaned folder scan.".to_string());
        return;
    }

    let mut orphaned_total = 0;

    for library in libraries {
        let (install_dirs, manifest_count) = collect_manifest_install_dirs(&library);
        let common_path = library.join("common");
        if !common_path.exists() {
            continue;
        }

        if manifest_count == 0 {
            logs.push(format!(
                "[Steam] No appmanifest files found in {}. Treating all folders as orphaned.",
                library.display()
            ));
        }

        if let Ok(entries) = fs::read_dir(&common_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                let folder_name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(name) => name.to_string(),
                    None => continue,
                };

                if !install_dirs.contains(&folder_name.to_lowercase()) {
                    orphaned_total += 1;
                    logs.push(format!(
                        "[Steam] Orphaned game folder detected: {}",
                        path.display()
                    ));
                    try_delete(path.to_str().unwrap_or_default(), dry_run, logs);
                }
            }
        }
    }

    if orphaned_total == 0 {
        logs.push("[Steam] No orphaned game folders found.".to_string());
    } else {
        logs.push(format!(
            "[Steam] Orphaned game folder cleanup complete ({} folder(s)).",
            orphaned_total
        ));
    }
}

pub fn clean_cache(dry_run: bool, delete_orphaned_game_folders: bool) -> io::Result<Vec<String>> {
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
    let appdata =
        std::env::var("APPDATA").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
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
        format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent",
            userprofile
        ),
        format!("{}\\AppData\\Local\\CrashDumps", userprofile),
        format!("{}\\AppData\\LocalLow\\Temp", userprofile),
        format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\WebCache",
            userprofile
        ),
        format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\INetCache",
            userprofile
        ),
        format!("{}\\Tracing", userprofile),
        format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations",
            userprofile
        ),
        format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations",
            userprofile
        ),
        format!("{}\\AppData\\Local\\Temp", userprofile),
        format!("{}\\AppData\\Local\\Packages", userprofile),
        format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Caches",
            userprofile
        ),
        format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Explorer",
            userprofile
        ),
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

    if delete_orphaned_game_folders {
        clean_orphaned_game_folders(&steam_root, dry_run, &mut logs);
    }

    try_delete_dir_contents(&format!("{}\\Temp", local), dry_run, &mut logs);
    try_delete_dir_contents(
        &format!("{}\\AppData\\Local\\Temp", userprofile),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!("{}\\AppData\\LocalLow\\Temp", userprofile),
        dry_run,
        &mut logs,
    );

    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\INetCache",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\WebCache",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Caches",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Explorer",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!("{}\\AppData\\Local\\CrashDumps", userprofile),
        dry_run,
        &mut logs,
    );

    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations",
            userprofile
        ),
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        &format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations",
            userprofile
        ),
        dry_run,
        &mut logs,
    );

    try_delete_dir_contents(&format!("{}\\Tracing", userprofile), dry_run, &mut logs);
    try_delete_dir_contents("C:\\Windows\\Temp", dry_run, &mut logs);
    try_delete_dir_contents(
        "C:\\ProgramData\\Microsoft\\Windows\\Caches",
        dry_run,
        &mut logs,
    );
    try_delete_dir_contents(
        "C:\\ProgramData\\NVIDIA Corporation\\NV_Cache",
        dry_run,
        &mut logs,
    );
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

/// Granular cleaning function that accepts individual options for each cleaning step
pub fn clean_granular(
    dry_run: bool,
    kill_steam_processes: bool,
    kill_explorer: bool,
    delete_login_users_vdf: bool,
    delete_config_vdf: bool,
    delete_localconfig_vdf: bool,
    delete_steam_appdata_vdf: bool,
    delete_ssfn_files: bool,
    delete_libraryfolders_vdf: bool,
    delete_userdata_dir: bool,
    delete_config_dir: bool,
    delete_logs_dir: bool,
    delete_appcache_dir: bool,
    delete_dump_dir: bool,
    delete_shadercache_dir: bool,
    delete_depotcache_dir: bool,
    delete_orphaned_game_folders: bool,
    delete_steam_appdata_dir: bool,
    delete_valve_locallow_dir: bool,
    delete_d3d_cache: bool,
    delete_d3d_cache_contents: bool,
    delete_local_temp: bool,
    delete_local_low_temp: bool,
    delete_local_temp_contents: bool,
    delete_user_temp: bool,
    delete_user_temp_contents: bool,
    delete_windows_temp: bool,
    delete_windows_temp_contents: bool,
    delete_crash_dumps: bool,
    delete_web_cache: bool,
    delete_web_cache_contents: bool,
    delete_inet_cache: bool,
    delete_inet_cache_contents: bool,
    delete_windows_caches: bool,
    delete_windows_caches_contents: bool,
    delete_windows_explorer: bool,
    delete_windows_explorer_contents: bool,
    delete_recent: bool,
    delete_recent_contents: bool,
    delete_automatic_destinations: bool,
    delete_automatic_destinations_contents: bool,
    delete_custom_destinations: bool,
    delete_custom_destinations_contents: bool,
    delete_tracing_dir: bool,
    delete_tracing_dir_contents: bool,
    delete_nvidia_cache: bool,
    delete_nvidia_cache_contents: bool,
    delete_windows_prefetch: bool,
    delete_my_games: bool,
    delete_easyanticheat: bool,
    delete_battleye: bool,
    delete_faceit: bool,
) -> Vec<String> {
    let mut logs = Vec::new();

    // Kill processes
    let steam_processes = [
        "steam.exe",
        "steamwebhelper.exe",
        "GameOverlayUI.exe",
        "steamerrorreporter.exe",
    ];

    if kill_steam_processes {
        for proc in steam_processes.iter() {
            kill_process(proc, dry_run, &mut logs);
        }
    }
    if kill_explorer {
        kill_process("explorer.exe", dry_run, &mut logs);
    }

    if (kill_steam_processes || kill_explorer) && !dry_run {
        logs.push("Waiting 10 seconds to ensure processes are closed...".to_string());
        sleep(Duration::from_secs(10));
    }

    // Get paths
    let local = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| String::new());
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| String::new());
    let userprofile = std::env::var("USERPROFILE").unwrap_or_else(|_| String::new());
    let locallow = local.replace("Local", "LocalLow");

    // Find Steam root
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

    // Delete Steam login files
    if delete_login_users_vdf {
        let path = format!("{}\\loginusers.vdf", steam_config);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_config_vdf {
        let path = format!("{}\\config.vdf", steam_config);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_localconfig_vdf {
        let path = format!("{}\\localconfig.vdf", steam_config);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_steam_appdata_vdf {
        let path = format!("{}\\SteamAppData.vdf", steam_config);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_libraryfolders_vdf {
        let path = format!("{}\\libraryfolders.vdf", steam_root);
        try_delete(&path, dry_run, &mut logs);
    }

    // Delete SSFN files
    if delete_ssfn_files {
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
    }

    // Delete Steam directories
    if delete_userdata_dir {
        let path = format!("{}\\userdata", steam_root);
        if delete_userdata_dir {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        }
    }
    if delete_config_dir {
        try_delete_dir_contents(&steam_config, dry_run, &mut logs);
    }
    if delete_logs_dir {
        let path = format!("{}\\logs", steam_root);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_appcache_dir {
        let path = format!("{}\\appcache", steam_root);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_dump_dir {
        let path = format!("{}\\dump", steam_root);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_shadercache_dir {
        let path = format!("{}\\shadercache", steam_root);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_depotcache_dir {
        let path = format!("{}\\depotcache", steam_root);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_orphaned_game_folders {
        clean_orphaned_game_folders(&steam_root, dry_run, &mut logs);
    }

    // Delete Steam AppData directories
    if delete_steam_appdata_dir {
        let path = format!("{}\\Steam", appdata);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_valve_locallow_dir {
        let path = format!("{}\\Valve\\Steam", locallow);
        try_delete(&path, dry_run, &mut logs);
    }

    // Delete D3D cache
    if delete_d3d_cache {
        let path = format!("{}\\D3DSCache", local);
        if delete_d3d_cache_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }

    // Delete temp directories
    if delete_local_temp {
        let path = format!("{}\\Temp", local);
        if delete_local_temp_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_local_low_temp {
        let path = format!("{}\\AppData\\LocalLow\\Temp", userprofile);
        try_delete(&path, dry_run, &mut logs);
    }
    if delete_user_temp {
        let path = format!("{}\\AppData\\Local\\Temp", userprofile);
        if delete_user_temp_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_windows_temp {
        let path = "C:\\Windows\\Temp";
        if delete_windows_temp_contents {
            try_delete_dir_contents(path, dry_run, &mut logs);
        } else {
            try_delete(path, dry_run, &mut logs);
        }
    }

    // Delete crash dumps
    if delete_crash_dumps {
        let path = format!("{}\\AppData\\Local\\CrashDumps", userprofile);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }

    // Delete Windows Explorer caches
    if delete_web_cache {
        let path = format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\WebCache",
            userprofile
        );
        if delete_web_cache_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_inet_cache {
        let path = format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\INetCache",
            userprofile
        );
        if delete_inet_cache_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_windows_caches {
        let path = format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Caches",
            userprofile
        );
        if delete_windows_caches_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_windows_explorer {
        let path = format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Explorer",
            userprofile
        );
        if delete_windows_explorer_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }

    // Delete recent files
    if delete_recent {
        let path = format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent",
            userprofile
        );
        if delete_recent_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_automatic_destinations {
        let path = format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations",
            userprofile
        );
        if delete_automatic_destinations_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }
    if delete_custom_destinations {
        let path = format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations",
            userprofile
        );
        if delete_custom_destinations_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }

    // Delete tracing directory
    if delete_tracing_dir {
        let path = format!("{}\\Tracing", userprofile);
        if delete_tracing_dir_contents {
            try_delete_dir_contents(&path, dry_run, &mut logs);
        } else {
            try_delete(&path, dry_run, &mut logs);
        }
    }

    // Delete NVIDIA cache
    if delete_nvidia_cache {
        let path = "C:\\ProgramData\\NVIDIA Corporation\\NV_Cache";
        if delete_nvidia_cache_contents {
            try_delete_dir_contents(path, dry_run, &mut logs);
        } else {
            try_delete(path, dry_run, &mut logs);
        }
    }

    // Deep cleaning options
    if delete_windows_prefetch {
        try_delete_dir_contents("C:\\Windows\\Prefetch", dry_run, &mut logs);
    }
    if delete_my_games {
        let path = format!("{}\\Documents\\My Games", userprofile);
        try_delete_dir_contents(&path, dry_run, &mut logs);
    }
    if delete_easyanticheat {
        try_delete("C:\\ProgramData\\EasyAntiCheat", dry_run, &mut logs);
    }
    if delete_battleye {
        try_delete("C:\\ProgramData\\BattlEye", dry_run, &mut logs);
    }
    if delete_faceit {
        try_delete("C:\\ProgramData\\Faceit", dry_run, &mut logs);
    }

    if !dry_run {
        let _ = Command::new("explorer").spawn();
        logs.push("Cache cleanup complete.".to_string());
    }

    logs
}
