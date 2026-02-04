use iced::Font;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    German,
    Spanish,
    French,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::Italian => "Italiano",
            Language::Portuguese => "Português",
            Language::Russian => "Русский",
            Language::Chinese => "中文",
            Language::Japanese => "日本語",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::German,
            Language::Spanish,
            Language::French,
            Language::Italian,
            Language::Portuguese,
            Language::Russian,
            Language::Chinese,
            Language::Japanese,
        ]
    }

    pub fn font(&self) -> Option<Font> {
        match self {
            Language::Chinese => Some(Font::with_name("Microsoft YaHei")),
            Language::Japanese => Some(Font::with_name("MS Gothic")),
            Language::English
            | Language::German
            | Language::Spanish
            | Language::French
            | Language::Italian
            | Language::Portuguese
            | Language::Russian => None,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub app_title: String,
    pub main_window: MainWindowTranslations,
    pub inspector: InspectorTranslations,
    pub themes: ThemesTranslations,
    pub theme_customizer: ThemeCustomizerTranslations,
    pub custom_clean: CustomCleanTranslations,
    pub redist: RedistTranslations,
    pub common: CommonTranslations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWindowTranslations {
    pub ready_message: String,
    pub system_spoofing: String,
    pub spoof_system_ids: String,
    pub spoof_mac_address: String,
    pub spoof_volume_id: String,
    pub steam_cleaning: String,
    pub clean_steam: String,
    pub aggressive_cleaning: String,
    pub aggressive_clean: String,
    pub inspector_and_profiles: String,
    pub steam_redist_cleaner_beta: String,
    pub themes_and_appearance: String,
    pub execute_cleaning: String,
    pub cleaning: String,
    pub backup_steam_data: String,
    pub simulation_mode_dry_run: String,
    pub custom_clean: String,
    pub verbose_log_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorTranslations {
    pub title: String,
    pub loading_system_info: String,
    pub current_system_hardware_ids: String,
    pub machine_guid: String,
    pub product_id: String,
    pub computer_name: String,
    pub volume_id_c: String,
    pub mac_addresses: String,
    pub hardware_id_profile_manager: String,
    pub select_a_profile: String,
    pub load_profile: String,
    pub apply: String,
    pub delete: String,
    pub enter_profile_name: String,
    pub save_current_as_profile: String,
    pub profile: String,
    pub created: String,
    pub mac_addresses_count: String,
    pub volume_ids_count: String,
    pub select_profile_details: String,
    pub create_new_profile: String,
    pub back_to_main: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemesTranslations {
    pub title: String,
    pub select_application_theme: String,
    pub red_retro_default: String,
    pub light_mode: String,
    pub neutral_dark: String,
    pub ultra_dark: String,
    pub cream: String,
    pub custom_colors: String,
    pub back_to_main: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeCustomizerTranslations {
    pub title: String,
    pub core_layout: String,
    pub background: String,
    pub surface_panels: String,
    pub text_color: String,
    pub accents_and_status: String,
    pub primary_accent: String,
    pub danger_error: String,
    pub success_go: String,
    pub preview: String,
    pub live_preview: String,
    pub sample_window_content: String,
    pub sample_description: String,
    pub primary_action: String,
    pub danger_zone: String,
    pub success_message: String,
    pub apply_changes: String,
    pub back_to_themes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCleanTranslations {
    pub title: String,
    pub execute_custom_clean: String,
    pub cleaning_in_progress: String,
    pub back_to_main: String,

    // Section headers
    pub system_id_spoofing: String,
    pub registry_game_tracking: String,
    pub registry_system_caches: String,
    pub mac_volume_spoofing: String,
    pub mac_address_spoofing: String,
    pub volume_id_spoofing: String,
    pub steam_login_files: String,
    pub steam_directories: String,
    pub system_cache_directories: String,
    pub windows_explorer_caches: String,
    pub recent_files: String,
    pub gpu_caches: String,
    pub deep_cleaning: String,
    pub processes: String,

    // Granular options - System ID Spoofing
    pub spoof_machine_guid: String,
    pub spoof_hw_profile_guid: String,
    pub spoof_product_id: String,
    pub spoof_registered_owner: String,
    pub spoof_install_date: String,
    pub spoof_computer_name: String,

    // Granular options - Registry Game Tracking
    pub delete_steam_registry_hkcu: String,
    pub delete_valve_registry_hklm: String,
    pub delete_valve_registry_hku: String,
    pub delete_faceit_hkcu: String,
    pub delete_riot_hkcu: String,
    pub delete_esea_hkcu: String,
    pub delete_eac_hkcu: String,
    pub delete_battleye_hkcu: String,
    pub delete_startup_run: String,

    // Granular options - Registry System Caches
    pub clean_app_compat_cache: String,
    pub clean_shim_cache: String,
    pub clean_app_compat_flags: String,

    // Granular options - MAC & Volume ID
    pub spoof_mac_addresses: String,
    pub spoof_volume_c_drive: String,

    // Granular options - Steam Login Files
    pub delete_login_users_vdf: String,
    pub delete_config_vdf: String,
    pub delete_localconfig_vdf: String,
    pub delete_steam_appdata_vdf: String,
    pub delete_ssfn_files: String,
    pub delete_libraryfolders_vdf: String,

    // Granular options - Steam Directories
    pub delete_userdata_dir: String,
    pub delete_config_dir: String,
    pub delete_logs_dir: String,
    pub delete_appcache_dir: String,
    pub delete_dump_dir: String,
    pub delete_shadercache_dir: String,
    pub delete_depotcache_dir: String,

    // Granular options - System Cache Directories
    pub delete_steam_appdata_dir: String,
    pub delete_valve_locallow_dir: String,
    pub delete_d3d_cache: String,
    pub delete_local_temp: String,
    pub delete_local_low_temp: String,
    pub delete_user_temp: String,
    pub delete_windows_temp: String,
    pub delete_crash_dumps: String,

    // Granular options - Windows Explorer Caches
    pub delete_web_cache: String,
    pub delete_inet_cache: String,
    pub delete_windows_caches: String,
    pub delete_windows_explorer: String,

    // Granular options - Recent Files
    pub delete_recent: String,
    pub delete_automatic_destinations: String,
    pub delete_custom_destinations: String,
    pub delete_tracing_dir: String,

    // Granular options - GPU Caches
    pub delete_nvidia_cache: String,

    // Granular options - Deep Cleaning
    pub delete_windows_prefetch: String,
    pub delete_my_games: String,
    pub delete_easyanticheat: String,
    pub delete_battleye: String,
    pub delete_faceit: String,

    // Granular options - Processes
    pub kill_steam_processes: String,
    pub kill_explorer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedistTranslations {
    pub title: String,
    pub select_categories: String,
    pub common_redistributables: String,
    pub directx_installers: String,
    pub dotnet_framework: String,
    pub visual_c_redistributables: String,
    pub other_installers_aggressive: String,
    pub scan_steam_libraries: String,
    pub scanning: String,
    pub found_folders: String,
    pub found_count: String,
    pub clean_selected_items: String,
    pub clean_results: String,
    pub and_more: String,
    pub back_to_main: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTranslations {
    pub aggressive_cleaning_warning_title: String,
    pub aggressive_cleaning_warning_message: String,
    pub aggressive_cleaning_cancelled: String,
    pub starting_cleaning: String,
    pub starting_custom_cleaning: String,
    pub backup_failed: String,
    pub please_enter_profile_name: String,
    pub profile_saved: String,
    pub failed_to_save: String,
    pub failed_to_snapshot: String,
    pub profile_not_found: String,
    pub please_select_profile: String,
    pub profile_applied: String,
    pub delete_profile_title: String,
    pub delete_profile_confirmation: String,
    pub profile_deleted: String,
    pub profiles_refreshed: String,
    pub pick_background_color: String,
    pub pick_surface_color: String,
    pub pick_text_color: String,
    pub pick_primary_color: String,
    pub pick_danger_color: String,
    pub pick_success_color: String,
}

pub fn load_translations(lang: Language) -> Translations {
    let json_bytes: &[u8] = match lang {
        Language::English => include_bytes!("locales/en.json"),
        Language::German => include_bytes!("locales/de.json"),
        Language::Spanish => include_bytes!("locales/es.json"),
        Language::French => include_bytes!("locales/fr.json"),
        Language::Italian => include_bytes!("locales/it.json"),
        Language::Portuguese => include_bytes!("locales/pt.json"),
        Language::Russian => include_bytes!("locales/ru.json"),
        Language::Chinese => include_bytes!("locales/zh.json"),
        Language::Japanese => include_bytes!("locales/ja.json"),
    };

    let json_str = std::str::from_utf8(json_bytes).expect("Invalid UTF-8 in translation file");
    serde_json::from_str(json_str).unwrap_or_else(|e| {
        eprintln!(
            "Failed to parse translation for language '{}': {:?}",
            lang.name(),
            e
        );
        default_translations()
    })
}

fn default_translations() -> Translations {
    Translations {
        app_title: "Steam Cleaner 0.1.9".to_string(),
        main_window: MainWindowTranslations {
            ready_message: "[*] Ready. Select options and click Execute.".to_string(),
            system_spoofing: "System Spoofing".to_string(),
            spoof_system_ids: "Spoof System IDs".to_string(),
            spoof_mac_address: "Spoof MAC Address".to_string(),
            spoof_volume_id: "Spoof Volume ID".to_string(),
            steam_cleaning: "Steam Cleaning".to_string(),
            clean_steam: "Clean Steam".to_string(),
            aggressive_cleaning: "Aggressive Cleaning".to_string(),
            aggressive_clean: "Aggressive Clean".to_string(),
            inspector_and_profiles: "Inspector & Profiles".to_string(),
            steam_redist_cleaner_beta: "Steam Redist Cleaner (Beta)".to_string(),
            themes_and_appearance: "Themes & Appearance".to_string(),
            execute_cleaning: "Execute Cleaning".to_string(),
            cleaning: "Cleaning...".to_string(),
            backup_steam_data: "Backup Steam Data".to_string(),
            simulation_mode_dry_run: "Simulation Mode (Dry Run)".to_string(),
            custom_clean: "Custom Clean".to_string(),
            verbose_log_output: "Verbose Log Output".to_string(),
        },
        inspector: InspectorTranslations {
            title: "System Inspector & Profile Manager".to_string(),
            loading_system_info: "Loading system information...".to_string(),
            current_system_hardware_ids: "Current System Hardware IDs".to_string(),
            machine_guid: "Machine GUID: {}".to_string(),
            product_id: "Product ID: {}".to_string(),
            computer_name: "Computer Name: {}".to_string(),
            volume_id_c: "Volume ID (C:): {}".to_string(),
            mac_addresses: "MAC Addresses:".to_string(),
            hardware_id_profile_manager: "Hardware ID Profile Manager".to_string(),
            select_a_profile: "Select a profile...".to_string(),
            load_profile: "Load Profile: ".to_string(),
            apply: "Apply".to_string(),
            delete: "Delete".to_string(),
            enter_profile_name: "Enter profile name...".to_string(),
            save_current_as_profile: "Save Current as Profile".to_string(),
            profile: "Profile: {}".to_string(),
            created: "  Created: {}".to_string(),
            mac_addresses_count: "  {} MAC address(es), {} Volume ID(s)".to_string(),
            volume_ids_count: "  {} MAC address(es), {} Volume ID(s)".to_string(),
            select_profile_details: "Select a profile to see details, or save current hardware IDs as a new profile.".to_string(),
            create_new_profile: "Create New Profile from Current Hardware:".to_string(),
            back_to_main: "<- Back to Main".to_string(),
        },
        themes: ThemesTranslations {
            title: "Appearance Settings".to_string(),
            select_application_theme: "Select Application Theme:".to_string(),
            red_retro_default: "Red Retro (Default)".to_string(),
            light_mode: "Light Mode".to_string(),
            neutral_dark: "Neutral Dark".to_string(),
            ultra_dark: "Ultra Dark".to_string(),
            cream: "Cream".to_string(),
            custom_colors: "Custom Colors...".to_string(),
            back_to_main: "<- Back to Main".to_string(),
        },
        theme_customizer: ThemeCustomizerTranslations {
            title: "Theme Customizer".to_string(),
            core_layout: "Core Layout".to_string(),
            background: "Background".to_string(),
            surface_panels: "Surface / Panels".to_string(),
            text_color: "Text Color".to_string(),
            accents_and_status: "Accents & Status".to_string(),
            primary_accent: "Primary Accent".to_string(),
            danger_error: "Danger / Error".to_string(),
            success_go: "Success / Go".to_string(),
            preview: "Preview".to_string(),
            live_preview: "Live Preview".to_string(),
            sample_window_content: "Sample Window Content".to_string(),
            sample_description: "This shows how your color choices look together.".to_string(),
            primary_action: "Primary Action".to_string(),
            danger_zone: "Danger Zone".to_string(),
            success_message: "Success Message Received".to_string(),
            apply_changes: "Apply Changes".to_string(),
            back_to_themes: "<- Back to Themes".to_string(),
        },
        custom_clean: CustomCleanTranslations {
            title: "Custom Cleaning Options".to_string(),
            execute_custom_clean: "Execute Custom Clean".to_string(),
            cleaning_in_progress: "Cleaning in Progress...".to_string(),
            back_to_main: "<- Back to Main".to_string(),

            // Section headers
            system_id_spoofing: "System ID Spoofing".to_string(),
            registry_game_tracking: "Registry - Game Tracking".to_string(),
            registry_system_caches: "Registry - System Caches".to_string(),
            mac_volume_spoofing: "MAC & Volume ID Spoofing".to_string(),
            mac_address_spoofing: "MAC Address Spoofing".to_string(),
            volume_id_spoofing: "Volume ID Spoofing".to_string(),
            steam_login_files: "Steam Login Files".to_string(),
            steam_directories: "Steam Directories".to_string(),
            system_cache_directories: "System Cache Directories".to_string(),
            windows_explorer_caches: "Windows Explorer Caches".to_string(),
            recent_files: "Recent Files".to_string(),
            gpu_caches: "GPU Caches".to_string(),
            deep_cleaning: "Deep Cleaning".to_string(),
            processes: "Processes".to_string(),

            // Granular options - System ID Spoofing
            spoof_machine_guid: "Spoof Machine GUID".to_string(),
            spoof_hw_profile_guid: "Spoof HwProfileGuid".to_string(),
            spoof_product_id: "Spoof Windows Product ID".to_string(),
            spoof_registered_owner: "Spoof Registered Owner/Org".to_string(),
            spoof_install_date: "Spoof Windows Install Date".to_string(),
            spoof_computer_name: "Spoof Computer Name".to_string(),

            // Granular options - Registry Game Tracking
            delete_steam_registry_hkcu: "Delete HKCU\\Software\\Valve\\Steam".to_string(),
            delete_valve_registry_hklm: "Delete HKLM\\Software\\Valve".to_string(),
            delete_valve_registry_hku: "Delete HKU users\\Software\\Valve".to_string(),
            delete_faceit_hkcu: "Delete HKCU\\Software\\FaceIt".to_string(),
            delete_riot_hkcu: "Delete HKCU\\Software\\Riot Games".to_string(),
            delete_esea_hkcu: "Delete HKCU\\Software\\ESEA".to_string(),
            delete_eac_hkcu: "Delete HKCU\\Software\\EasyAntiCheat".to_string(),
            delete_battleye_hkcu: "Delete HKCU\\Software\\Battleye".to_string(),
            delete_startup_run: "Delete HKCU\\...\\Run entries".to_string(),

            // Granular options - Registry System Caches
            clean_app_compat_cache: "Delete AppCompatCache".to_string(),
            clean_shim_cache: "Delete ShimCache".to_string(),
            clean_app_compat_flags: "Delete AppCompatFlags".to_string(),

            // Granular options - MAC & Volume ID
            spoof_mac_addresses: "Spoof All Network MAC Addresses".to_string(),
            spoof_volume_c_drive: "Spoof C: Drive Volume ID".to_string(),

            // Granular options - Steam Login Files
            delete_login_users_vdf: "Delete loginusers.vdf".to_string(),
            delete_config_vdf: "Delete config.vdf".to_string(),
            delete_localconfig_vdf: "Delete localconfig.vdf".to_string(),
            delete_steam_appdata_vdf: "Delete SteamAppData.vdf".to_string(),
            delete_ssfn_files: "Delete all SSFN files".to_string(),
            delete_libraryfolders_vdf: "Delete libraryfolders.vdf".to_string(),

            // Granular options - Steam Directories
            delete_userdata_dir: "Delete userdata directory".to_string(),
            delete_config_dir: "Delete config directory".to_string(),
            delete_logs_dir: "Delete logs directory".to_string(),
            delete_appcache_dir: "Delete appcache directory".to_string(),
            delete_dump_dir: "Delete dump directory".to_string(),
            delete_shadercache_dir: "Delete shadercache directory".to_string(),
            delete_depotcache_dir: "Delete depotcache directory".to_string(),

            // Granular options - System Cache Directories
            delete_steam_appdata_dir: "Delete Steam AppData folder".to_string(),
            delete_valve_locallow_dir: "Delete Valve LocalLow folder".to_string(),
            delete_d3d_cache: "Delete D3D shader cache".to_string(),
            delete_local_temp: "Delete Local\\Temp".to_string(),
            delete_local_low_temp: "Delete LocalLow\\Temp".to_string(),
            delete_user_temp: "Delete user Temp".to_string(),
            delete_windows_temp: "Delete Windows\\Temp".to_string(),
            delete_crash_dumps: "Delete CrashDumps folder".to_string(),

            // Granular options - Windows Explorer Caches
            delete_web_cache: "Delete WebCache".to_string(),
            delete_inet_cache: "Delete INetCache".to_string(),
            delete_windows_caches: "Delete Windows Caches".to_string(),
            delete_windows_explorer: "Delete Windows Explorer folder".to_string(),

            // Granular options - Recent Files
            delete_recent: "Delete Recent folder".to_string(),
            delete_automatic_destinations: "Delete AutomaticDestinations".to_string(),
            delete_custom_destinations: "Delete CustomDestinations".to_string(),
            delete_tracing_dir: "Delete Tracing directory".to_string(),

            // Granular options - GPU Caches
            delete_nvidia_cache: "Delete NVIDIA NV_Cache".to_string(),

            // Granular options - Deep Cleaning
            delete_windows_prefetch: "Delete Windows Prefetch".to_string(),
            delete_my_games: "Delete My Games folder".to_string(),
            delete_easyanticheat: "Delete EasyAntiCheat data".to_string(),
            delete_battleye: "Delete BattlEye data".to_string(),
            delete_faceit: "Delete FACEIT data".to_string(),

            // Granular options - Processes
            kill_steam_processes: "Terminate Steam processes".to_string(),
            kill_explorer: "Restart explorer.exe".to_string(),
        },
        redist: RedistTranslations {
            title: "Steam Redistributable Cleaner".to_string(),
            select_categories: "Select categories to remove from game folders:".to_string(),
            common_redistributables: "Common Redistributables (_CommonRedist)".to_string(),
            directx_installers: "DirectX Installers".to_string(),
            dotnet_framework: ".NET Framework".to_string(),
            visual_c_redistributables: "Visual C++ Redistributables".to_string(),
            other_installers_aggressive: "Other Installers/Support (Aggressive)".to_string(),
            scan_steam_libraries: "Scan Steam Libraries".to_string(),
            scanning: "Scanning... please wait.".to_string(),
            found_folders: "Found {} folders. Total size: {}".to_string(),
            found_count: "Found {} folders. Total size: {}".to_string(),
            clean_selected_items: "Clean Selected Items".to_string(),
            clean_results: "Clean Results:".to_string(),
            and_more: "...and {} more.".to_string(),
            back_to_main: "<- Back to Main".to_string(),
        },
        common: CommonTranslations {
            aggressive_cleaning_warning_title: "Aggressive Cleaning Warning".to_string(),
            aggressive_cleaning_warning_message: "Aggressive cleaning can have unintended side effects. Are you sure you want to continue?".to_string(),
            aggressive_cleaning_cancelled: "Aggressive cleaning cancelled.".to_string(),
            starting_cleaning: "[*] Starting cleaning...".to_string(),
            starting_custom_cleaning: "[*] Starting custom cleaning...".to_string(),
            backup_failed: "Backup failed: {}".to_string(),
            please_enter_profile_name: "[!] Please enter a profile name.".to_string(),
            profile_saved: "[+] Profile '{}' saved!".to_string(),
            failed_to_save: "[-] Failed to save: {}".to_string(),
            failed_to_snapshot: "Failed to snapshot: {}".to_string(),
            profile_not_found: "[!] Profile not found.".to_string(),
            please_select_profile: "[!] Please select a profile first.".to_string(),
            profile_applied: "[+] Profile applied! Check log for details.".to_string(),
            delete_profile_title: "Delete Profile".to_string(),
            delete_profile_confirmation: "Are you sure you want to delete profile '{}'?".to_string(),
            profile_deleted: "[+] Profile '{}' deleted.".to_string(),
            profiles_refreshed: "[*] Profiles refreshed.".to_string(),
            pick_background_color: "Pick Background Color".to_string(),
            pick_surface_color: "Pick Surface Color".to_string(),
            pick_text_color: "Pick Text Color".to_string(),
            pick_primary_color: "Pick Primary Color".to_string(),
            pick_danger_color: "Pick Danger Color".to_string(),
            pick_success_color: "Pick Success Color".to_string(),
        },
    }
}

pub fn format_string(template: &str, args: &[&str]) -> String {
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("{{{}}}", i), arg);
    }
    result
}
