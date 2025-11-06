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

    if options.spoof_system_ids {
        match registry_cleaner::clean_registry(options.dry_run) {
            Ok(messages) => results.extend(messages),
            Err(e) => results.push(format!("[Error] Error spoofing System IDs: {}", e)),
        }
        match sid_spoofer::spoof_hkcu(options.dry_run) {
            Ok(messages) => results.extend(messages),
            Err(e) => results.push(format!("[Error] Error spoofing HKCU keys: {}", e)),
        }
    }

    if options.spoof_mac {
        match mac_spoofer::spoof_mac_all(options.dry_run) {
            Ok(messages) => results.extend(messages),
            Err(e) => results.push(format!("[Error] Error spoofing MAC addresses: {}", e)),
        }
    }

    if options.spoof_volume_id {
        match volumeid_wrapper::change_volume_id("C", options.dry_run) {
            Ok(message) => results.push(message),
            Err(e) => results.push(format!("[Error] Error changing Volume ID: {}", e)),
        }
    }
    
    if options.clean_steam {
        let dry_run = options.dry_run;
        let result = tokio::task::spawn_blocking(move || file_cleaner::clean_cache(dry_run)).await;
        match result {
            Ok(Ok(messages)) => results.extend(messages),
            Ok(Err(e)) => results.push(format!("[Error] Error cleaning Steam: {}", e)),
            Err(_) => results.push("[Error] Critical error in Steam cleaning task.".to_string()),
        }
    }

    if options.clean_aggressive {
        match registry_cleaner::clean_aggressive_registry(options.dry_run) {
            Ok(messages) => results.extend(messages),
            Err(e) => results.push(format!("[Error] Error with aggressive registry cleaning: {}", e)),
        }
    }

    if results.len() == 1 && options.dry_run {
        results.push("[Info] No operations selected.".to_string());
    } else if results.is_empty() {
        results.push("[Info] No operations selected.".to_string());
    }

    if options.dry_run {
        results.push("--- END OF SIMULATION ---".to_string());
    } else {
        results.push("-----------------------------------".to_string());
        results.push("[Success] All tasks completed. A restart is recommended.".to_string());
    }

    results
}
