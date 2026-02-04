// src/core/executor.rs

use crate::core::{file_cleaner, mac_spoofer, registry_cleaner, sid_spoofer, volumeid_wrapper};

#[derive(Debug, Clone, Copy, Default)]
pub struct CleaningOptions {
    pub spoof_system_ids: bool,
    pub spoof_mac: bool,
    pub spoof_volume_id: bool,
    pub clean_steam: bool,
    pub clean_aggressive: bool,
    pub dry_run: bool,
}

pub async fn run_all_selected(options: CleaningOptions) -> Vec<String> {
    let mut results = Vec::new();
    if options.dry_run {
        results.push("--- SIMULATION MODE (DRY RUN) ---".to_string());
    }
    println!("Starting asynchronous cleaning with options: {:?}", options);

    // Run all blocking operations in parallel using spawn_blocking
    let mut tasks = Vec::new();

    if options.spoof_system_ids {
        let dry_run = options.dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            match registry_cleaner::clean_registry(dry_run) {
                Ok(messages) => logs.extend(messages),
                Err(e) => logs.push(format!("❌ Error spoofing System IDs: {}", e)),
            }
            match sid_spoofer::spoof_hkcu(dry_run) {
                Ok(messages) => logs.extend(messages),
                Err(e) => logs.push(format!("❌ Error spoofing HKCU keys: {}", e)),
            }
            logs
        }));
    }

    if options.spoof_mac {
        let dry_run = options.dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match mac_spoofer::spoof_mac_all(dry_run) {
                Ok(messages) => messages,
                Err(e) => vec![format!("❌ Error spoofing MAC addresses: {}", e)],
            }
        }));
    }

    if options.spoof_volume_id {
        let dry_run = options.dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match volumeid_wrapper::change_volume_id("C", dry_run) {
                Ok(message) => vec![message],
                Err(e) => vec![format!("❌ Error changing Volume ID: {}", e)],
            }
        }));
    }

    if options.clean_steam {
        let dry_run = options.dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match file_cleaner::clean_cache(dry_run) {
                Ok(messages) => messages,
                Err(e) => vec![format!("❌ Error cleaning Steam: {}", e)],
            }
        }));
    }

    if options.clean_aggressive {
        let dry_run = options.dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match registry_cleaner::clean_aggressive_registry(dry_run) {
                Ok(messages) => messages,
                Err(e) => vec![format!("❌ Error with aggressive registry cleaning: {}", e)],
            }
        }));
    }

    // Wait for all tasks to complete and collect results
    for task in tasks {
        match task.await {
            Ok(logs) => results.extend(logs),
            Err(e) => results.push(format!("❌ Task failed: {}", e)),
        }
    }

    if results.len() == 1 && options.dry_run {
        results.push("ℹ️ No operations selected.".to_string());
    } else if results.is_empty() {
        results.push("ℹ️ No operations selected.".to_string());
    }

    if options.dry_run {
        results.push("--- END OF SIMULATION ---".to_string());
    } else {
        results.push("-----------------------------------".to_string());
        results.push("✅ All tasks completed. A restart is recommended.".to_string());
    }

    results
}

/// Applies a hardware profile (MAC addresses, Volume IDs)
pub async fn apply_hardware_profile(
    profile: crate::core::hardware_profile::HardwareProfile,
    dry_run: bool,
) -> Vec<String> {
    let mut results = Vec::new();

    results.push(format!("━━━ Applying Profile: '{}' ━━━", profile.name));
    results.push(format!("Created: {}", profile.created_at));

    if dry_run {
        results.push("--- SIMULATION MODE (DRY RUN) ---".to_string());
    }

    // Run blocking operations in parallel
    let mut tasks = Vec::new();

    // Apply MAC addresses
    if !profile.mac_addresses.is_empty() {
        results.push(format!("[*] Applying {} MAC address(es)...", profile.mac_addresses.len()));
        let mac_addresses = profile.mac_addresses.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            match mac_spoofer::spoof_mac_from_profile(&mac_addresses, dry_run) {
                Ok(messages) => messages,
                Err(e) => vec![format!("❌ Error applying MAC addresses: {}", e)],
            }
        }));
    } else {
        results.push("[!] No MAC addresses in profile.".to_string());
    }

    // Apply Volume IDs
    if !profile.volume_ids.is_empty() {
        results.push(format!("[*] Applying {} Volume ID(s)...", profile.volume_ids.len()));
        let volume_ids = profile.volume_ids.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            for (drive, vol_id) in &volume_ids {
                match volumeid_wrapper::change_volume_id_to_specific(drive, vol_id, dry_run) {
                    Ok(message) => logs.push(message),
                    Err(e) => logs.push(format!("❌ Error setting Volume ID for {}: {}", drive, e)),
                }
            }
            logs
        }));
    } else {
        results.push("[!] No Volume IDs in profile.".to_string());
    }

    // Wait for all tasks to complete and collect results
    for task in tasks {
        match task.await {
            Ok(logs) => results.extend(logs),
            Err(e) => results.push(format!("❌ Task failed: {}", e)),
        }
    }

    if dry_run {
        results.push("--- END OF SIMULATION ---".to_string());
    } else {
        results.push("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());
        results.push("✅ Profile applied. A restart is recommended.".to_string());
    }

    results
}

