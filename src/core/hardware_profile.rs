// src/core/hardware_profile.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    pub name: String,
    pub created_at: String,
    pub machine_guid: Option<String>,
    /// Product ID
    pub product_id: Option<String>,
    /// Computer Name
    pub computer_name: Option<String>,
    /// Volume IDs for different drives (e.g. "C" -> "1234-5678")
    pub volume_ids: HashMap<String, String>,
    /// MAC addresses for different adapters (Adapter-Key -> MAC)
    pub mac_addresses: HashMap<String, String>,
}

/// Container for all saved profiles
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileManager {
    /// List of all profiles
    pub profiles: Vec<HardwareProfile>,
    /// Name of the currently active profile (if any)
    pub active_profile: Option<String>,
}

impl ProfileManager {
    /// Default path for the profile file
    pub fn default_path() -> PathBuf {
        let app_data = std::env::var("APPDATA")
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(app_data)
            .join("SteamCleaner")
            .join("hardware_profiles.json")
    }

    /// Loads the ProfileManager from the default file
    pub fn load() -> io::Result<Self> {
        let path = Self::default_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Saves the ProfileManager to the default file
    pub fn save(&self) -> io::Result<()> {
        let path = Self::default_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&path, content)
    }

    /// Adds a new profile or updates an existing one
    pub fn add_or_update_profile(&mut self, profile: HardwareProfile) {
        // Remove existing profile with same name
        self.profiles.retain(|p| p.name != profile.name);
        self.profiles.push(profile);
    }

    /// Removes a profile by name
    pub fn remove_profile(&mut self, name: &str) {
        self.profiles.retain(|p| p.name != name);
        if self.active_profile.as_deref() == Some(name) {
            self.active_profile = None;
        }
    }

    /// Searches for a profile by name
    pub fn get_profile(&self, name: &str) -> Option<&HardwareProfile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    /// Returns all profile names
    pub fn profile_names(&self) -> Vec<String> {
        self.profiles.iter().map(|p| p.name.clone()).collect()
    }
}

impl HardwareProfile {
    /// Creates a new profile with the specified name
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_at: chrono_lite_now(),
            ..Default::default()
        }
    }

    /// Creates a snapshot of current system hardware IDs
    #[cfg(windows)]
    pub fn snapshot_current(name: String) -> io::Result<Self> {
        use std::process::Command;
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;

        let mut profile = Self::new(name);

        // Machine GUID
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
        {
            profile.machine_guid = hklm.get_value("MachineGuid").ok();
        }

        // Product ID
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        {
            profile.product_id = hklm.get_value("ProductId").ok();
        }

        // Computer Name
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters")
        {
            profile.computer_name = hklm.get_value("Hostname").ok();
        }

        // Volume ID for C:
        if let Ok(output) = Command::new("cmd").args(["/C", "vol C:"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = output_str.lines().find(|l| l.contains("Volume Serial Number")) {
                if line.len() > 23 {
                    let vol_id = line.split_at(23).1.trim().to_string();
                    profile.volume_ids.insert("C".to_string(), vol_id);
                }
            }
        }

        // MAC addresses
        let adapters_key_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";
        if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(adapters_key_path) {
            for res in hklm.enum_keys() {
                if let Ok(sub_key_name) = res {
                    if sub_key_name == "Properties" {
                        continue;
                    }
                    let current_adapter_path = format!(r"{}\{}", adapters_key_path, sub_key_name);
                    if let Ok(current_adapter_key) = RegKey::predef(HKEY_LOCAL_MACHINE)
                        .open_subkey(&current_adapter_path)
                    {
                        // Check if it's a physical adapter
                        let driver_desc: Result<String, _> = current_adapter_key.get_value("DriverDesc");
                        if let Ok(desc) = driver_desc {
                            let lc_desc = desc.to_lowercase();
                            let blacklist = ["wan miniport", "tunnel", "ppoe", "loopback", "ras async", "virtual", "teredo", "pseudo"];
                            if blacklist.iter().any(|b| lc_desc.contains(b)) {
                                continue;
                            }
                            
                            // Read current MAC (NetworkAddress) or original MAC
                            let mac: Option<String> = current_adapter_key
                                .get_value("NetworkAddress")
                                .ok()
                                .or_else(|| current_adapter_key.get_value("OriginalNetworkAddress").ok());
                            
                            if let Some(mac_addr) = mac {
                                profile.mac_addresses.insert(sub_key_name.clone(), mac_addr);
                            }
                        }
                    }
                }
            }
        }

        Ok(profile)
    }

    #[cfg(not(windows))]
    pub fn snapshot_current(name: String) -> io::Result<Self> {
        Ok(Self::new(name))
    }
}

/// Simple timestamp function without external dependencies
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    let secs = duration.as_secs();
    // Simple ISO-8601 like format
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;
    
    // Calculate approximate date (since 1970-01-01)
    let years = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        years, month.min(12), day.min(31), hours, minutes, seconds
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_manager_serialization() {
        let mut manager = ProfileManager::default();
        
        let mut profile = HardwareProfile::new("Test Profile".to_string());
        profile.machine_guid = Some("test-guid-123".to_string());
        profile.volume_ids.insert("C".to_string(), "1234-5678".to_string());
        profile.mac_addresses.insert("0001".to_string(), "AABBCCDDEEFF".to_string());
        
        manager.add_or_update_profile(profile);
        
        let json = serde_json::to_string_pretty(&manager).unwrap();
        let loaded: ProfileManager = serde_json::from_str(&json).unwrap();
        
        assert_eq!(loaded.profiles.len(), 1);
        assert_eq!(loaded.profiles[0].name, "Test Profile");
        assert_eq!(loaded.profiles[0].machine_guid, Some("test-guid-123".to_string()));
    }

    #[test]
    fn test_profile_names() {
        let mut manager = ProfileManager::default();
        manager.add_or_update_profile(HardwareProfile::new("Profile A".to_string()));
        manager.add_or_update_profile(HardwareProfile::new("Profile B".to_string()));
        
        let names = manager.profile_names();
        assert!(names.contains(&"Profile A".to_string()));
        assert!(names.contains(&"Profile B".to_string()));
    }
}
