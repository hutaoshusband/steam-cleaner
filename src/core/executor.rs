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

pub async fn run_all_selected(options: CleaningOptions) -> Vec<String> {
    let mut results = Vec::new();
    let dry_run = options.dry_run;

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
        options.delete_depotcache_dir || options.delete_steam_appdata_dir ||
        options.delete_valve_locallow_dir || options.delete_d3d_cache ||
        options.delete_local_temp || options.delete_local_low_temp ||
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
        results.push("--- SIMULATION MODE (DRY RUN) ---".to_string());
    }
    println!("Starting asynchronous cleaning with options: {:?}", options);

    // Run all blocking operations in parallel using spawn_blocking
    let mut tasks = Vec::new();

    // Legacy support - if legacy options are set, enable all granular options in that category
    if options.spoof_system_ids {
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
    } else if has_granular_options() {
        // Granular registry spoofing
        let dry_run = dry_run;
        let spoof_machine_guid = options.spoof_machine_guid;
        let spoof_hw_profile_guid = options.spoof_hw_profile_guid;
        let spoof_product_id = options.spoof_product_id;
        let spoof_registered_owner = options.spoof_registered_owner;
        let spoof_install_date = options.spoof_install_date;
        let spoof_computer_name = options.spoof_computer_name;
        let delete_steam_registry_hkcu = options.delete_steam_registry_hkcu;
        let spoof_machine_guid_bool = spoof_machine_guid;
        let spoof_hw_profile_guid_bool = spoof_hw_profile_guid;
        let spoof_product_id_bool = spoof_product_id;
        let spoof_registered_owner_bool = spoof_registered_owner;
        let spoof_install_date_bool = spoof_install_date;
        let spoof_computer_name_bool = spoof_computer_name;
        let delete_steam_registry_hkcu_bool = delete_steam_registry_hkcu;

        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            if spoof_machine_guid_bool {
                match registry_cleaner::spoof_machine_guid(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing MachineGuid: {}", e)),
                }
            }
            if spoof_hw_profile_guid_bool {
                match registry_cleaner::spoof_hw_profile_guid(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing HwProfileGuid: {}", e)),
                }
            }
            if spoof_product_id_bool {
                match registry_cleaner::spoof_windows_nt_info(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing Windows NT info: {}", e)),
                }
            }
            if spoof_registered_owner_bool {
                // Part of spoof_windows_nt_info
                match registry_cleaner::spoof_registered_owner(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing registered owner: {}", e)),
                }
            }
            if spoof_install_date_bool {
                // Part of spoof_windows_nt_info
                match registry_cleaner::spoof_install_date(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing install date: {}", e)),
                }
            }
            if spoof_computer_name_bool {
                match registry_cleaner::spoof_computer_name(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error spoofing computer name: {}", e)),
                }
            }
            if delete_steam_registry_hkcu_bool {
                match registry_cleaner::delete_steam_registry(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error deleting Steam registry: {}", e)),
                }
            }
            logs
        }));

        // Granular game registry deletion
        let dry_run = dry_run;
        let delete_valve_registry_hklm = options.delete_valve_registry_hklm;
        let delete_valve_registry_hku = options.delete_valve_registry_hku;
        let delete_faceit_hkcu = options.delete_faceit_hkcu;
        let delete_riot_hkcu = options.delete_riot_hkcu;
        let delete_esea_hkcu = options.delete_esea_hkcu;
        let delete_eac_hkcu = options.delete_eac_hkcu;
        let delete_battleye_hkcu = options.delete_battleye_hkcu;
        let delete_startup_run = options.delete_startup_run;
        let clean_app_compat_cache = options.clean_app_compat_cache;
        let clean_shim_cache = options.clean_shim_cache;
        let clean_app_compat_flags = options.clean_app_compat_flags;

        tasks.push(tokio::task::spawn_blocking(move || {
            let mut logs = Vec::new();
            if delete_valve_registry_hklm || delete_valve_registry_hku {
                match registry_cleaner::delete_more_valve_keys(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error deleting Valve keys: {}", e)),
                }
            }
            if clean_app_compat_cache || clean_shim_cache || clean_app_compat_flags {
                match registry_cleaner::clean_system_caches(dry_run, &mut logs) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error cleaning system caches: {}", e)),
                }
            }
            if delete_faceit_hkcu || delete_riot_hkcu || delete_esea_hkcu ||
               delete_eac_hkcu || delete_battleye_hkcu || delete_startup_run {
                match sid_spoofer::spoof_hkcu_detailed(
                    dry_run, delete_faceit_hkcu, delete_riot_hkcu,
                    delete_esea_hkcu, delete_eac_hkcu, delete_battleye_hkcu,
                    delete_startup_run, &mut logs
                ) {
                    Ok(_) => {},
                    Err(e) => logs.push(format!("❌ Error deleting HKCU keys: {}", e)),
                }
            }
            logs
        }));
    }

    if options.spoof_mac || options.spoof_mac_addresses {
        let dry_run = dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match mac_spoofer::spoof_mac_all(dry_run) {
                Ok(messages) => messages,
                Err(e) => vec![format!("❌ Error spoofing MAC addresses: {}", e)],
            }
        }));
    }

    if options.spoof_volume_id || options.spoof_volume_c_drive {
        let dry_run = dry_run;
        tasks.push(tokio::task::spawn_blocking(move || {
            match volumeid_wrapper::change_volume_id("C", dry_run) {
                Ok(message) => vec![message],
                Err(e) => vec![format!("❌ Error changing Volume ID: {}", e)],
            }
        }));
    }

    if options.clean_steam || options.clean_aggressive || has_granular_options() {
        let dry_run = dry_run;
        let kill_steam_processes = options.kill_steam_processes;
        let kill_explorer = options.kill_explorer;
        let delete_login_users_vdf = options.delete_login_users_vdf;
        let delete_config_vdf = options.delete_config_vdf;
        let delete_localconfig_vdf = options.delete_localconfig_vdf;
        let delete_steam_appdata_vdf = options.delete_steam_appdata_vdf;
        let delete_ssfn_files = options.delete_ssfn_files;
        let delete_libraryfolders_vdf = options.delete_libraryfolders_vdf;
        let delete_userdata_dir = options.delete_userdata_dir;
        let delete_config_dir = options.delete_config_dir;
        let delete_logs_dir = options.delete_logs_dir;
        let delete_appcache_dir = options.delete_appcache_dir;
        let delete_dump_dir = options.delete_dump_dir;
        let delete_shadercache_dir = options.delete_shadercache_dir;
        let delete_depotcache_dir = options.delete_depotcache_dir;
        let delete_steam_appdata_dir = options.delete_steam_appdata_dir;
        let delete_valve_locallow_dir = options.delete_valve_locallow_dir;
        let delete_d3d_cache = options.delete_d3d_cache;
        let delete_d3d_cache_contents = options.delete_d3d_cache_contents;
        let delete_local_temp = options.delete_local_temp;
        let delete_local_low_temp = options.delete_local_low_temp;
        let delete_local_temp_contents = options.delete_local_temp_contents;
        let delete_user_temp = options.delete_user_temp;
        let delete_user_temp_contents = options.delete_user_temp_contents;
        let delete_windows_temp = options.delete_windows_temp;
        let delete_windows_temp_contents = options.delete_windows_temp_contents;
        let delete_crash_dumps = options.delete_crash_dumps;
        let delete_web_cache = options.delete_web_cache;
        let delete_web_cache_contents = options.delete_web_cache_contents;
        let delete_inet_cache = options.delete_inet_cache;
        let delete_inet_cache_contents = options.delete_inet_cache_contents;
        let delete_windows_caches = options.delete_windows_caches;
        let delete_windows_caches_contents = options.delete_windows_caches_contents;
        let delete_windows_explorer = options.delete_windows_explorer;
        let delete_windows_explorer_contents = options.delete_windows_explorer_contents;
        let delete_recent = options.delete_recent;
        let delete_recent_contents = options.delete_recent_contents;
        let delete_automatic_destinations = options.delete_automatic_destinations;
        let delete_automatic_destinations_contents = options.delete_automatic_destinations_contents;
        let delete_custom_destinations = options.delete_custom_destinations;
        let delete_custom_destinations_contents = options.delete_custom_destinations_contents;
        let delete_tracing_dir = options.delete_tracing_dir;
        let delete_tracing_dir_contents = options.delete_tracing_dir_contents;
        let delete_nvidia_cache = options.delete_nvidia_cache;
        let delete_nvidia_cache_contents = options.delete_nvidia_cache_contents;
        let delete_windows_prefetch = options.delete_windows_prefetch;
        let delete_my_games = options.delete_my_games;
        let delete_easyanticheat = options.delete_easyanticheat;
        let delete_battleye = options.delete_battleye;
        let delete_faceit = options.delete_faceit;

        tasks.push(tokio::task::spawn_blocking(move || {
            if options.clean_steam || options.clean_aggressive {
                match file_cleaner::clean_cache(dry_run) {
                    Ok(messages) => messages,
                    Err(e) => vec![format!("❌ Error cleaning Steam: {}", e)],
                }
            } else {
                // Granular file cleaning
                file_cleaner::clean_granular(
                    dry_run,
                    kill_steam_processes,
                    kill_explorer,
                    delete_login_users_vdf,
                    delete_config_vdf,
                    delete_localconfig_vdf,
                    delete_steam_appdata_vdf,
                    delete_ssfn_files,
                    delete_libraryfolders_vdf,
                    delete_userdata_dir,
                    delete_config_dir,
                    delete_logs_dir,
                    delete_appcache_dir,
                    delete_dump_dir,
                    delete_shadercache_dir,
                    delete_depotcache_dir,
                    delete_steam_appdata_dir,
                    delete_valve_locallow_dir,
                    delete_d3d_cache,
                    delete_d3d_cache_contents,
                    delete_local_temp,
                    delete_local_low_temp,
                    delete_local_temp_contents,
                    delete_user_temp,
                    delete_user_temp_contents,
                    delete_windows_temp,
                    delete_windows_temp_contents,
                    delete_crash_dumps,
                    delete_web_cache,
                    delete_web_cache_contents,
                    delete_inet_cache,
                    delete_inet_cache_contents,
                    delete_windows_caches,
                    delete_windows_caches_contents,
                    delete_windows_explorer,
                    delete_windows_explorer_contents,
                    delete_recent,
                    delete_recent_contents,
                    delete_automatic_destinations,
                    delete_automatic_destinations_contents,
                    delete_custom_destinations,
                    delete_custom_destinations_contents,
                    delete_tracing_dir,
                    delete_tracing_dir_contents,
                    delete_nvidia_cache,
                    delete_nvidia_cache_contents,
                    delete_windows_prefetch,
                    delete_my_games,
                    delete_easyanticheat,
                    delete_battleye,
                    delete_faceit
                )
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

    if results.len() == 1 && dry_run {
        results.push("ℹ️ No operations selected.".to_string());
    } else if results.is_empty() {
        results.push("ℹ️ No operations selected.".to_string());
    }

    if dry_run {
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

