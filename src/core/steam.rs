use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::fs;

#[cfg(windows)]
pub fn get_steam_root() -> Option<PathBuf> {
    let local = std::env::var("LOCALAPPDATA").ok()?;
    let default_path = PathBuf::from(&local).join("Steam");
    
    let possible_paths = vec![
        PathBuf::from(r"C:\Program Files (x86)\Steam"),
        PathBuf::from(r"C:\Program Files\Steam"),
        default_path.clone(),
    ];

    possible_paths.into_iter().find(|p| p.exists())
}

#[cfg(not(windows))]
pub fn get_steam_root() -> Option<PathBuf> {
    None
}

#[cfg(windows)]
pub fn get_library_folders(steam_root: &Path) -> Vec<PathBuf> {
    let mut libraries = Vec::new();
    
    // Always add the main steamapps folder
    let main_lib = steam_root.join("steamapps");
    if main_lib.exists() {
        libraries.push(main_lib.clone());
    }

    let vdf_path = main_lib.join("libraryfolders.vdf");
    if vdf_path.exists() {
        if let Ok(content) = fs::read_to_string(vdf_path) {
            // Regex to match "path" "..."
            // We use a relatively simple regex. VDF files can contain comments, but usually libraryfolders.vdf is clean.
            // Using regex to capture the path value.
            if let Ok(re) = regex::Regex::new(r#"(?i)"path"\s+"([^"]+)""#) {
                for cap in re.captures_iter(&content) {
                    if let Some(path_match) = cap.get(1) {
                        let path_str = path_match.as_str().replace("\\\\", "\\");
                        let path = PathBuf::from(path_str);
                        let lib_path = path.join("steamapps");
                        if lib_path.exists() {
                            libraries.push(lib_path);
                        }
                    }
                }
            }
        }
    }
    
    // Deduplicate
    libraries.sort();
    libraries.dedup();
    
    libraries
}

#[cfg(not(windows))]
pub fn get_library_folders(_steam_root: &Path) -> Vec<PathBuf> {
    Vec::new()
}
