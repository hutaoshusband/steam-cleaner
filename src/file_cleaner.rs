use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io;

pub fn kill_process(process_name: &str) {
    println!("Terminating process: {}", process_name);
    match Command::new("taskkill")
        .args(["/F", "/IM", process_name])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Terminated: {}", process_name);
            } else {
                println!(
                    "Failed to terminate {}: {}",
                    process_name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => println!("Error terminating {}: {}", process_name, e),
    }
}
fn try_delete_dir_contents(path: &str) {
    let dir = Path::new(path);
    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let _ = Command::new("attrib")
                    .args(&["-R", "-S", "-H", entry_path.to_str().unwrap_or("")])
                    .output();
                let result = if entry_path.is_dir() {
                    fs::remove_dir_all(&entry_path)
                } else {
                    fs::remove_file(&entry_path)
                };
                match result {
                    Ok(_) => println!("Deleted from: {}", entry_path.display()),
                    Err(e) => println!("Failed to delete {}: {}", entry_path.display(), e),
                }
            }
        }
    } else {
        println!("Temp dir not found or not a dir: {}", path);
    }
}

fn try_delete(path: &str) {
    let p = Path::new(path);
    if p.exists() {
        let _ = Command::new("attrib")
            .args(&["-R", "-S", "-H", path])
            .output();
        let result = if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            fs::remove_file(p)
        };
        match result {
            Ok(_) => println!("Deleted: {}", path),
            Err(e) => println!("Failed to delete {}: {}", path, e),
        }
    } else {
        println!("Not found: {}", path);
    }
}

pub fn clean_cache() -> io::Result<()> {
    // Kill r
    let processes = [
        "steam.exe",
        "steamwebhelper.exe",
        "GameOverlayUI.exe",
        "steamerrorreporter.exe",
    ];
    for proc in processes.iter() {
        kill_process(proc);
        kill_process("explorer.exe");
    }

    // Wait
    println!("Waiting 10 seconds to ensure processes are closed...");
    sleep(Duration::from_secs(10));

    let local = std::env::var("LOCALAPPDATA")
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

let appdata = std::env::var("APPDATA")
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

let userprofile = std::env::var("USERPROFILE")
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;


let _possible_steam_paths = [
    "C:\\Program Files (x86)\\Steam",
    "C:\\Program Files\\Steam",
    &format!("{}\\Steam", local),
];
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
    println!("Checking for Steam file: {}", path);
    if Path::new(&path).exists() {
        println!("Found Steam file: {}", path);
        try_delete(&path);
    } else {
        println!("Steam file not found: {}", path);
    }
}


    // Delete ssfn files
    if let Ok(entries) = fs::read_dir(&steam_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("ssfn") {
                    try_delete(path.to_str().unwrap_or_default());
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
    ];

    for path in &paths {
        try_delete(path);
    }
    for file in harmless_files.iter() {
    try_delete(file);
    }
    try_delete_dir_contents(&format!("{}\\Temp", local));
    try_delete_dir_contents(&format!("{}\\AppData\\Local\\Temp", userprofile));
    try_delete_dir_contents(&format!("{}\\Temp", local)); // %LOCALAPPDATA%\Temp
try_delete_dir_contents(&format!("{}\\AppData\\Local\\Temp", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\LocalLow\\Temp", userprofile));

try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\INetCache", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\WebCache", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\Caches", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Local\\Microsoft\\Windows\\Explorer", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Local\\CrashDumps", userprofile));

try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\AutomaticDestinations", userprofile));
try_delete_dir_contents(&format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\CustomDestinations", userprofile));

try_delete_dir_contents(&format!("{}\\Tracing", userprofile));
try_delete_dir_contents("C:\\Windows\\Temp");
try_delete_dir_contents("C:\\ProgramData\\Microsoft\\Windows\\Caches");
try_delete_dir_contents("C:\\ProgramData\\NVIDIA Corporation\\NV_Cache");
try_delete_dir_contents(&format!("{}\\appcache", steam_root));
try_delete_dir_contents(&format!("{}\\logs", steam_root));
try_delete_dir_contents(&format!("{}\\dump", steam_root));
try_delete_dir_contents(&format!("{}\\shadercache", steam_root));
try_delete_dir_contents(&format!("{}\\userdata", steam_root));
try_delete_dir_contents(&format!("{}\\config", steam_root));

let _ = Command::new("explorer").spawn();

    println!("Cache cleanup complete.");
    Ok(())
}
