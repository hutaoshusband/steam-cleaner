// src/core/executor.rs

use crate::core::{file_cleaner, mac_spoofer, registry_cleaner, sid_spoofer, volumeid_wrapper};

#[derive(Debug, Clone, Copy, Default)]
pub struct CleaningOptions {
    // Legacy options (for main window)
    pub spoof_system_ids: bool,
    pub spoof_mac: bool,
    pub spoof_volume_id: bool,
    pub clean_steam: bool,
    pub clean_aggressive: bool,
    pub dry_run: bool,

    // Granular - Registry System ID Spoofing
    pub spoof_machine_guid: bool,
    pub spoof_hw_profile_guid: bool,
    pub spoof_product_id: bool,
    pub spoof_registered_owner: bool,
    pub spoof_install_date: bool,
    pub spoof_computer_name: bool,

    // Granular - Registry Game Tracking
    pub delete_steam_registry_hkcu: bool,
    pub delete_valve_registry_hklm: bool,
    pub delete_valve_registry_hku: bool,
    pub delete_faceit_hkcu: bool,
    pub delete_riot_hkcu: bool,
    pub delete_esea_hkcu: bool,
    pub delete_eac_hkcu: bool,
    pub delete_battleye_hkcu: bool,
    pub delete_startup_run: bool,

    // Granular - Registry System Caches
    pub clean_app_compat_cache: bool,
    pub clean_shim_cache: bool,
    pub clean_app_compat_flags: bool,

    // Granular - MAC Address Spoofing
    pub spoof_mac_addresses: bool,

    // Granular - Volume ID Spoofing
    pub spoof_volume_c_drive: bool,

    // Granular - Steam Login Files
    pub delete_login_users_vdf: bool,
    pub delete_config_vdf: bool,
    pub delete_localconfig_vdf: bool,
    pub delete_steam_appdata_vdf: bool,
    pub delete_ssfn_files: bool,
    pub delete_libraryfolders_vdf: bool,

    // Granular - Steam Directories
    pub delete_userdata_dir: bool,
    pub delete_config_dir: bool,
    pub delete_logs_dir: bool,
    pub delete_appcache_dir: bool,
    pub delete_dump_dir: bool,
    pub delete_shadercache_dir: bool,
    pub delete_depotcache_dir: bool,
    pub delete_orphaned_game_folders: bool,

    // Granular - System Cache Directories
    pub delete_steam_appdata_dir: bool,
    pub delete_valve_locallow_dir: bool,
    pub delete_d3d_cache: bool,
    pub delete_d3d_cache_contents: bool,
    pub delete_local_temp: bool,
    pub delete_local_low_temp: bool,
    pub delete_local_temp_contents: bool,
    pub delete_user_temp: bool,
    pub delete_user_temp_contents: bool,
    pub delete_windows_temp: bool,
    pub delete_windows_temp_contents: bool,
    pub delete_crash_dumps: bool,

    // Granular - Windows Explorer Caches
    pub delete_web_cache: bool,
    pub delete_web_cache_contents: bool,
    pub delete_inet_cache: bool,
    pub delete_inet_cache_contents: bool,
    pub delete_windows_caches: bool,
    pub delete_windows_caches_contents: bool,
    pub delete_windows_explorer: bool,
    pub delete_windows_explorer_contents: bool,

    // Granular - Recent Files
    pub delete_recent: bool,
    pub delete_recent_contents: bool,
    pub delete_automatic_destinations: bool,
    pub delete_automatic_destinations_contents: bool,
    pub delete_custom_destinations: bool,
    pub delete_custom_destinations_contents: bool,
    pub delete_tracing_dir: bool,
    pub delete_tracing_dir_contents: bool,

    // Granular - GPU Caches
    pub delete_nvidia_cache: bool,
    pub delete_nvidia_cache_contents: bool,

    // Granular - Deep Cleaning
    pub delete_windows_prefetch: bool,
    pub delete_my_games: bool,
    pub delete_easyanticheat: bool,
    pub delete_battleye: bool,
    pub delete_faceit: bool,

    // Granular - Steam Processes
    pub kill_steam_processes: bool,
    pub kill_explorer: bool,
}

pub async fn run_all_selected(options: CleaningOptions, on_log: impl Fn(String) + Send + Sync + 'static) -> Vec<String> {
    let mut results = Vec::new();
    let dry_run = options.dry_run;
    let on_log = std::sync::Arc::new(on_log);

    // Helper to check if any granular option is set
    let has_granular_options = || {
        options.spoof_machine_guid || options.spoof_hw_profile_guid ||
        options.spoof_product_id || options.spoof_registered_owner ||
        options.spoof_install_date || options.spoof_computer_name ||
        options.delete_steam_registry_hkcu || options.delete_valve_registry_hklm ||
        options.delete_valve_registry_hku || options.delete_faceit_hkcu ||
        options.delete_riot_hkcu || options.delete_esea_hkcu ||
        options.delete_eac_hkcu || options.delete_battleye_hkcu ||
        options.delete_startup_run || options.clean_app_compat_cache ||
        options.clean_shim_cache || options.clean_app_compat_flags ||
        options.spoof_mac_addresses || options.spoof_volume_c_drive ||
        options.delete_login_users_vdf || options.delete_config_vdf ||
        options.delete_localconfig_vdf || options.delete_steam_appdata_vdf ||
        options.delete_ssfn_files || options.delete_libraryfolders_vdf ||
        options.delete_userdata_dir || options.delete_config_dir ||
        options.delete_logs_dir || options.delete_appcache_dir ||
        options.delete_dump_dir || options.delete_shadercache_dir ||
        options.delete_depotcache_dir || options.delete_orphaned_game_folders ||
        options.delete_steam_appdata_dir || options.delete_valve_locallow_dir ||
        options.delete_d3d_cache || options.delete_local_temp ||
        options.delete_local_low_temp ||
        options.delete_local_temp_contents || options.delete_user_temp ||
        options.delete_user_temp_contents || options.delete_windows_temp ||
        options.delete_windows_temp_contents || options.delete_crash_dumps ||
        options.delete_web_cache || options.delete_web_cache_contents ||
        options.delete_inet_cache || options.delete_inet_cache_contents ||
        options.delete_windows_caches || options.delete_windows_caches_contents ||
        options.delete_windows_explorer || options.delete_windows_explorer_contents ||
        options.delete_recent || options.delete_recent_contents ||
        options.delete_automatic_destinations || options.delete_automatic_destinations_contents ||
        options.delete_custom_destinations || options.delete_custom_destinations_contents ||
        options.delete_tracing_dir || options.delete_tracing_dir_contents ||
        options.delete_nvidia_cache || options.delete_nvidia_cache_contents ||
        options.delete_windows_prefetch || options.delete_my_games ||
        options.delete_easyanticheat || options.delete_battleye ||
        options.delete_faceit || options.kill_steam_processes ||
        options.kill_explorer
    };

    if dry_run {
        let msg = "--- SIMULATION MODE (DRY RUN) ---".to_string();
        on_log(msg.clone());
        results.push(msg);
    }
    let start_msg = format!("[*] Starting asynchronous cleaning with options: {:?}", options);
    on_log(start_msg.clone());
    results.push(start_msg);

    // Run all blocking operations in parallel using spawn_blocking
    let mut tasks = Vec::new();

    // Legacy support - if legacy options are set, enable all granular options in that category
    if options.spoof_system_ids {
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            match registry_cleaner::clean_registry(dry_run) {
                Ok(messages) => {
                    for m in messages {
                        on_log_inner(m.clone());
                        logs.push(m);
                    }
                }
                Err(e) => {
                    let m = format!("❌ Error spoofing System IDs: {}", e);
                    on_log_inner(m.clone());
                    logs.push(m);
                }
            }
            match sid_spoofer::spoof_hkcu(dry_run) {
                Ok(messages) => {
                    for m in messages {
                        on_log_inner(m.clone());
                        logs.push(m);
                    }
                }
                Err(e) => {
                    let m = format!("❌ Error spoofing HKCU keys: {}", e);
                    on_log_inner(m.clone());
                    logs.push(m);
                }
            }
            logs
        }));
    } else if has_granular_options() {
        // Granular registry spoofing
        let dry_run = dry_run;
        let options = options;
        let on_log_inner = on_log.clone();

        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            let mut run_action = |name: &str, action: fn(bool, &mut Vec<String>) -> std::io::Result<()>| {
                let mut action_logs = Vec::new();
                match action(dry_run, &mut action_logs) {
                    Ok(_) => {
                        for m in action_logs {
                            on_log_inner(m.clone());
                            logs.push(m);
                        }
                    }
                    Err(e) => {
                        let m = format!("❌ Error spoofing {}: {}", name, e);
                        on_log_inner(m.clone());
                        logs.push(m);
                    }
                }
            };

            if options.spoof_machine_guid { run_action("MachineGuid", registry_cleaner::spoof_machine_guid); }
            if options.spoof_hw_profile_guid { run_action("HwProfileGuid", registry_cleaner::spoof_hw_profile_guid); }
            if options.spoof_product_id { run_action("Windows NT info", registry_cleaner::spoof_windows_nt_info); }
            if options.spoof_registered_owner { run_action("registered owner", registry_cleaner::spoof_registered_owner); }
            if options.spoof_install_date { run_action("install date", registry_cleaner::spoof_install_date); }
            if options.spoof_computer_name { run_action("computer name", registry_cleaner::spoof_computer_name); }
            if options.delete_steam_registry_hkcu { run_action("Steam registry", registry_cleaner::delete_steam_registry); }
            logs
        }));

        // Granular game registry deletion
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            let mut action_logs = Vec::new();
            if options.delete_valve_registry_hklm || options.delete_valve_registry_hku {
                match registry_cleaner::delete_more_valve_keys(dry_run, &mut action_logs) {
                    Ok(_) => {}
                    Err(e) => action_logs.push(format!("❌ Error deleting Valve keys: {}", e)),
                }
            }
            if options.clean_app_compat_cache || options.clean_shim_cache || options.clean_app_compat_flags {
                match registry_cleaner::clean_system_caches(dry_run, &mut action_logs) {
                    Ok(_) => {}
                    Err(e) => action_logs.push(format!("❌ Error cleaning system caches: {}", e)),
                }
            }
            if options.delete_faceit_hkcu || options.delete_riot_hkcu || options.delete_esea_hkcu ||
               options.delete_eac_hkcu || options.delete_battleye_hkcu || options.delete_startup_run {
                match sid_spoofer::spoof_hkcu_detailed(
                    dry_run, options.delete_faceit_hkcu, options.delete_riot_hkcu,
                    options.delete_esea_hkcu, options.delete_eac_hkcu, options.delete_battleye_hkcu,
                    options.delete_startup_run, &mut action_logs
                ) {
                    Ok(_) => {}
                    Err(e) => action_logs.push(format!("❌ Error deleting HKCU keys: {}", e)),
                }
            }
            for m in action_logs {
                on_log_inner(m.clone());
                logs.push(m);
            }
            logs
        }));
    }

    if options.spoof_mac || options.spoof_mac_addresses {
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            match mac_spoofer::spoof_mac_all(dry_run) {
                Ok(messages) => {
                    for m in &messages { on_log_inner(m.clone()); }
                    messages
                }
                Err(e) => {
                    let m = format!("❌ Error spoofing MAC addresses: {}", e);
                    on_log_inner(m.clone());
                    vec![m]
                }
            }
        }));
    }

    if options.spoof_volume_id || options.spoof_volume_c_drive {
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            match volumeid_wrapper::change_volume_id("C", dry_run) {
                Ok(message) => {
                    on_log_inner(message.clone());
                    vec![message]
                }
                Err(e) => {
                    let m = format!("❌ Error changing Volume ID: {}", e);
                    on_log_inner(m.clone());
                    vec![m]
                }
            }
        }));
    }

    if options.clean_steam || options.clean_aggressive || has_granular_options() {
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            let messages = if options.clean_steam || options.clean_aggressive {
                match file_cleaner::clean_cache(dry_run, options.delete_orphaned_game_folders) {
                    Ok(messages) => messages,
                    Err(e) => vec![format!("❌ Error cleaning Steam: {}", e)],
                }
            } else {
                file_cleaner::clean_granular(
                    dry_run,
                    options.kill_steam_processes,
                    options.kill_explorer,
                    options.delete_login_users_vdf,
                    options.delete_config_vdf,
                    options.delete_localconfig_vdf,
                    options.delete_steam_appdata_vdf,
                    options.delete_ssfn_files,
                    options.delete_libraryfolders_vdf,
                    options.delete_userdata_dir,
                    options.delete_config_dir,
                    options.delete_logs_dir,
                    options.delete_appcache_dir,
                    options.delete_dump_dir,
                    options.delete_shadercache_dir,
                    options.delete_depotcache_dir,
                    options.delete_orphaned_game_folders,
                    options.delete_steam_appdata_dir,
                    options.delete_valve_locallow_dir,
                    options.delete_d3d_cache,
                    options.delete_d3d_cache_contents,
                    options.delete_local_temp,
                    options.delete_local_low_temp,
                    options.delete_local_temp_contents,
                    options.delete_user_temp,
                    options.delete_user_temp_contents,
                    options.delete_windows_temp,
                    options.delete_windows_temp_contents,
                    options.delete_crash_dumps,
                    options.delete_web_cache,
                    options.delete_web_cache_contents,
                    options.delete_inet_cache,
                    options.delete_inet_cache_contents,
                    options.delete_windows_caches,
                    options.delete_windows_caches_contents,
                    options.delete_windows_explorer,
                    options.delete_windows_explorer_contents,
                    options.delete_recent,
                    options.delete_recent_contents,
                    options.delete_automatic_destinations,
                    options.delete_automatic_destinations_contents,
                    options.delete_custom_destinations,
                    options.delete_custom_destinations_contents,
                    options.delete_tracing_dir,
                    options.delete_tracing_dir_contents,
                    options.delete_nvidia_cache,
                    options.delete_nvidia_cache_contents,
                    options.delete_windows_prefetch,
                    options.delete_my_games,
                    options.delete_easyanticheat,
                    options.delete_battleye,
                    options.delete_faceit
                )
            };
            for m in &messages { on_log_inner(m.clone()); }
            messages
        }));
    }

    // Wait for all tasks to complete and collect results
    for task in tasks {
        match task.await {
            Ok(logs) => results.extend(logs),
            Err(e) => {
                let m = format!("❌ Task failed: {}", e);
                on_log(m.clone());
                results.push(m);
            }
        }
    }

    if results.is_empty() || (results.len() == 1 && dry_run) {
        let m = "ℹ️ No operations selected.".to_string();
        on_log(m.clone());
        results.push(m);
    }

    if dry_run {
        let m = "--- END OF SIMULATION ---".to_string();
        on_log(m.clone());
        results.push(m);
    } else {
        on_log("-----------------------------------".to_string());
        let m = "✅ All tasks completed. A restart is recommended.".to_string();
        on_log(m.clone());
        results.push(m);
    }

    results
}

/// Applies a hardware profile (MAC addresses, Volume IDs)
pub async fn apply_hardware_profile(
    profile: crate::core::hardware_profile::HardwareProfile,
    dry_run: bool,
    on_log: impl Fn(String) + Send + Sync + 'static,
) -> Vec<String> {
    let mut results = Vec::new();
    let on_log = std::sync::Arc::new(on_log);

    let m1 = format!("━━━ Applying Profile: '{}' ━━━", profile.name);
    let m2 = format!("Created: {}", profile.created_at);
    on_log(m1.clone()); results.push(m1);
    on_log(m2.clone()); results.push(m2);

    if dry_run {
        let m = "--- SIMULATION MODE (DRY RUN) ---".to_string();
        on_log(m.clone()); results.push(m);
    }

    // Run blocking operations in parallel
    let mut tasks = Vec::new();

    // Apply MAC addresses
    if !profile.mac_addresses.is_empty() {
        let m = format!("[*] Applying {} MAC address(es)...", profile.mac_addresses.len());
        on_log(m.clone()); results.push(m);
        let mac_addresses = profile.mac_addresses.clone();
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            match mac_spoofer::spoof_mac_from_profile(&mac_addresses, dry_run) {
                Ok(messages) => {
                    for m in &messages { on_log_inner(m.clone()); }
                    messages
                }
                Err(e) => {
                    let m = format!("❌ Error applying MAC addresses: {}", e);
                    on_log_inner(m.clone());
                    vec![m]
                }
            }
        }));
    } else {
        let m = "[!] No MAC addresses in profile.".to_string();
        on_log(m.clone()); results.push(m);
    }

    // Apply Volume IDs
    if !profile.volume_ids.is_empty() {
        let m = format!("[*] Applying {} Volume ID(s)...", profile.volume_ids.len());
        on_log(m.clone()); results.push(m);
        let volume_ids = profile.volume_ids.clone();
        let on_log_inner = on_log.clone();
        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            for (drive, vol_id) in &volume_ids {
                match volumeid_wrapper::change_volume_id_to_specific(drive, vol_id, dry_run) {
                    Ok(message) => {
                        on_log_inner(message.clone());
                        logs.push(message);
                    }
                    Err(e) => {
                        let m = format!("❌ Error setting Volume ID for {}: {}", drive, e);
                        on_log_inner(m.clone());
                        logs.push(m);
                    }
                }
            }
            logs
        }));
    } else {
        let m = "[!] No Volume IDs in profile.".to_string();
        on_log(m.clone()); results.push(m);
    }

    // Wait for all tasks to complete and collect results
    for task in tasks {
        match task.await {
            Ok(logs) => results.extend(logs),
            Err(e) => {
                let m = format!("❌ Task failed: {}", e);
                on_log(m.clone());
                results.push(m);
            }
        }
    }

    if dry_run {
        let m = "--- END OF SIMULATION ---".to_string();
        on_log(m.clone()); results.push(m);
    } else {
        on_log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());
        let m = "✅ Profile applied. A restart is recommended.".to_string();
        on_log(m.clone()); results.push(m);
    }

    results
}
