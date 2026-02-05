// src/core/backup.rs

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

fn find_steam_root() -> Option<PathBuf> {
    let possible_paths = ["C:\\Program Files (x86)\\Steam", "C:\\Program Files\\Steam"];

    for path in possible_paths.iter() {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(windows)]
pub fn create_backup() -> Result<String, String> {
    let steam_root = find_steam_root().ok_or("Steam installation not found.")?;
    let config_path = steam_root.join("config");
    let userdata_path = steam_root.join("userdata");

    let save_path = rfd::FileDialog::new()
        .add_filter("zip", &["zip"])
        .set_file_name("steam_backup.zip")
        .save_file();

    if let Some(save_path) = save_path {
        let file = File::create(&save_path).map_err(|e| e.to_string())?;
        let mut zip = ZipWriter::new(file);

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);

        if config_path.exists() {
            add_dir_to_zip(&mut zip, &config_path, "config", options)?;
        }

        if userdata_path.exists() {
            add_dir_to_zip(&mut zip, &userdata_path, "userdata", options)?;
        }

        zip.finish().map_err(|e| e.to_string())?;
        Ok(format!("Backup created successfully at: {:?}", save_path))
    } else {
        Err("Backup operation was canceled.".to_string())
    }
}

#[cfg(not(windows))]
pub fn create_backup() -> Result<String, String> {
    Err("Backup is only supported on Windows.".to_string())
}

fn add_dir_to_zip<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    dir: &Path,
    base_in_zip: &str,
    options: FileOptions,
) -> Result<(), String> {
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let relative_path = path.strip_prefix(dir).map_err(|e| e.to_string())?;
        let name_path = Path::new(base_in_zip).join(relative_path);
        let name = name_path.to_str().unwrap();

        if path.is_file() {
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            let mut f = File::open(path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        } else if !name.is_empty() {
            zip.add_directory(name, options)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Cursor;
    use zip::ZipArchive;

    #[test]
    fn test_add_dir_to_zip_ignores_base_in_zip() {
        // Setup
        let temp_dir = std::env::temp_dir().join("test_steam_cleaner_backup_repro");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let source_dir = temp_dir.join("source_data");
        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("test_file.txt"), "content").unwrap();

        // Use Cursor<Vec<u8>> to own the buffer
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);

        // Execute
        let result = add_dir_to_zip(&mut zip, &source_dir, "target_data", options);

        assert!(result.is_ok());
        let cursor = zip.finish().unwrap();
        let buffer = cursor.into_inner();

        // Verify
        let mut archive = ZipArchive::new(Cursor::new(buffer)).unwrap();

        let file_names: Vec<_> = archive.file_names().map(|s| s.to_string()).collect();

        let found_target = file_names.iter().any(|n| n.starts_with("target_data"));
        let found_source = file_names.iter().any(|n| n.starts_with("source_data"));

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();

        assert!(
            found_target,
            "Bug reproduced: 'target_data' prefix not found in zip. Found: {:?}",
            file_names
        );
        assert!(
            !found_source,
            "Bug reproduced: 'source_data' prefix found in zip, should have been renamed."
        );
    }
}
