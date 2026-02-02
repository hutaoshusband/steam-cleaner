#[cfg(windows)]
pub mod file_cleaner;
#[cfg(windows)]
pub mod mac_spoofer;
#[cfg(windows)]
pub mod registry_cleaner;
#[cfg(windows)]
pub mod sid_spoofer;
#[cfg(windows)]
pub mod volumeid_wrapper;
#[cfg(windows)]
pub mod privileges;
#[cfg(windows)]
pub mod inspector;
#[cfg(any(windows, test))]
pub mod backup;

pub mod hardware_profile;
pub mod executor;
pub mod steam;
pub mod redist;

#[cfg(not(windows))]
pub mod file_cleaner {
    pub fn clean_cache(_dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["File cleaning is only supported on Windows.".to_string()])
    }
}

#[cfg(not(windows))]
pub mod mac_spoofer {
    pub fn spoof_mac_all(_dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["MAC spoofing is only supported on Windows.".to_string()])
    }
    pub fn spoof_mac_from_profile(_adapter_macs: &std::collections::HashMap<String, String>, _dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["MAC spoofing is only supported on Windows.".to_string()])
    }
}

#[cfg(not(windows))]
pub mod registry_cleaner {
    pub fn clean_registry(_dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["Registry cleaning is only supported on Windows.".to_string()])
    }
    pub fn clean_aggressive_registry(_dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["Aggressive registry cleaning is only supported on Windows.".to_string()])
    }
}

#[cfg(not(windows))]
pub mod sid_spoofer {
    pub fn spoof_hkcu(_dry_run: bool) -> std::io::Result<Vec<String>> {
        Ok(vec!["HKCU spoofing is only supported on Windows.".to_string()])
    }
}

#[cfg(not(windows))]
pub mod volumeid_wrapper {
    pub fn change_volume_id(_drive_letter: &str, _dry_run: bool) -> std::io::Result<String> {
        Ok("Volume ID changing is only supported on Windows.".to_string())
    }
    pub fn change_volume_id_to_specific(_drive_letter: &str, _volume_id: &str, _dry_run: bool) -> std::io::Result<String> {
        Ok("Volume ID changing is only supported on Windows.".to_string())
    }
}

#[cfg(not(windows))]
pub mod privileges {
    pub fn is_elevated() -> bool {
        true // Assume elevated on non-Windows for compilation purposes
    }
    pub fn show_admin_error_dialog() {}
}

#[cfg(not(windows))]
pub mod inspector {
    #[derive(Debug, Clone, Default)]
    pub struct SystemInfo {
        pub machine_guid: String,
        pub product_id: String,
        pub computer_name: String,
        pub volume_id: String,
        pub network_adapters: Vec<(String, String)>,
        pub steam_login_files: Vec<String>,
    }
    pub async fn gather_system_info() -> SystemInfo {
        SystemInfo {
            machine_guid: "Not applicable".to_string(),
            product_id: "Not applicable".to_string(),
            computer_name: "Not applicable".to_string(),
            volume_id: "Not applicable".to_string(),
            network_adapters: vec![],
            steam_login_files: vec![],
        }
    }
}

#[cfg(all(not(windows), not(test)))]
pub mod backup {
    pub fn create_backup() -> Result<String, String> {
        Err("Backup is only supported on Windows.".to_string())
    }
}
