use std::path::{Path, PathBuf};
use walkdir::WalkDir;
#[cfg(windows)]
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistCategory {
    CommonRedist,
    DirectX,
    DotNet,
    VCRedist,
    Installers,
}

impl RedistCategory {
    pub fn description(&self) -> &str {
        match self {
            RedistCategory::CommonRedist => "Common Redistributables (_CommonRedist)",
            RedistCategory::DirectX => "DirectX Installers",
            RedistCategory::DotNet => ".NET Framework Installers",
            RedistCategory::VCRedist => "Visual C++ Redistributables",
            RedistCategory::Installers => "Other Installers (Support, Redist, Prereq)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedistItem {
    pub path: PathBuf,
    pub category: RedistCategory,
    pub size: u64,
}

#[cfg(windows)]
pub fn scan_redistributables(libraries: &[PathBuf], active_categories: &[RedistCategory]) -> Vec<RedistItem> {
    let mut results = Vec::new();

    for lib in libraries {
        let common_path = lib.join("common");
        if !common_path.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(common_path) {
            for entry in entries.flatten() {
                let game_path = entry.path();
                if !game_path.is_dir() {
                    continue;
                }

                // Recursively search game directory (max depth 5 to avoid infinite loops or deep structures)
                let walker = WalkDir::new(&game_path).max_depth(5).into_iter();

                // We collect matches to avoid modifying the walker during iteration or dealing with ownership issues
                // Also, we want to avoid adding subdirectories of already matched directories.
                // But simplified approach first: find all candidates.

                for entry in walker.filter_map(|e| e.ok()) {
                    if !entry.file_type().is_dir() {
                        continue;
                    }

                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();

                    let category = if name.eq_ignore_ascii_case("_CommonRedist") {
                        Some(RedistCategory::CommonRedist)
                    } else if name.eq_ignore_ascii_case("DirectX") || name.eq_ignore_ascii_case("DXRedist") || name.eq_ignore_ascii_case("DirectX9") {
                        Some(RedistCategory::DirectX)
                    } else if name.eq_ignore_ascii_case("DotNet") || name.eq_ignore_ascii_case("dotnet") || name.starts_with("Microsoft.NET") {
                        Some(RedistCategory::DotNet)
                    } else if name.eq_ignore_ascii_case("VCRedist") || name.eq_ignore_ascii_case("vcredist") {
                        Some(RedistCategory::VCRedist)
                    } else if name.eq_ignore_ascii_case("Installers") || name.eq_ignore_ascii_case("Support") || name.eq_ignore_ascii_case("Redist") || name.eq_ignore_ascii_case("Prereq") {
                        Some(RedistCategory::Installers)
                    } else {
                        None
                    };

                    if let Some(cat) = category {
                        if active_categories.contains(&cat) {
                            // Check if this path is already covered by a parent path in results
                            // This is a simple optimization to avoid listing subfolders of a folder we are already deleting.
                            let already_covered = results.iter().any(|item: &RedistItem| path.starts_with(&item.path) && path != item.path);

                            if !already_covered {
                                let size = get_dir_size(path);
                                results.push(RedistItem {
                                    path: path.to_path_buf(),
                                    category: cat,
                                    size,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    results
}

#[cfg(not(windows))]
pub fn scan_redistributables(_libraries: &[PathBuf], _active_categories: &[RedistCategory]) -> Vec<RedistItem> {
    Vec::new()
}

fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|m| m.len())
        .sum()
}

#[cfg(windows)]
pub fn clean_redistributables(items: &[RedistItem], dry_run: bool) -> Vec<String> {
    let mut logs = Vec::new();

    for item in items {
        if dry_run {
            logs.push(format!("[Dry Run] Would delete directory: {} ({})", item.path.display(), format_size(item.size)));
        } else {
            // Check if path exists before trying to delete (it might have been deleted if it was nested in another deleted folder,
            // though our scan logic tries to prevent that)
            if item.path.exists() {
                match fs::remove_dir_all(&item.path) {
                    Ok(_) => logs.push(format!("Deleted: {} ({})", item.path.display(), format_size(item.size))),
                    Err(e) => logs.push(format!("Failed to delete {}: {}", item.path.display(), e)),
                }
            }
        }
    }

    logs
}

#[cfg(not(windows))]
pub fn clean_redistributables(_items: &[RedistItem], _dry_run: bool) -> Vec<String> {
    vec!["Redist cleaning is only supported on Windows.".to_string()]
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
