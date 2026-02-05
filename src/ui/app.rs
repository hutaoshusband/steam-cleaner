use iced::widget::{button, checkbox, column, container, pick_list, row, scrollable, svg, text, text_input, toggler, Column, Row, Space};
use iced::{Application, Color, Command, Element, Length, Subscription, Theme, time, alignment::Horizontal};
use std::time::Duration;
use tinyfiledialogs as tfd;

use crate::core::backup;
use crate::core::executor::{apply_hardware_profile, run_all_selected, CleaningOptions};
use crate::core::hardware_profile::{HardwareProfile, ProfileManager};
use crate::core::inspector::{gather_system_info, SystemInfo};
use crate::ui::style;
use crate::ui::redist_view;
use crate::i18n::{self, Language, Translations};
use crate::core::redist;
#[cfg(windows)]
use crate::core::steam;

pub struct CleanerApp {
    state: State,
    options: CleaningOptions,
    log_messages: Vec<String>,
    inspector_open: bool,
    inspector_state: InspectorState,
    profile_manager: ProfileManager,
    profile_state: ProfileState,
    redist_open: bool,
    redist_state: redist_view::RedistViewState,
    theme_open: bool,
    current_theme: Theme,
    custom_colors_open: bool,
    custom_colors: style::CustomThemeColors,
    custom_theme_active: bool,
    custom_clean_open: bool,
    custom_clean_options: CleaningOptions,
    rainbow_hue: f32,
    current_language: Language,
    translations: Translations,
    language_selector_open: bool,
}

#[derive(Default)]
struct InspectorState {
    is_loading: bool,
    info: SystemInfo,
}

#[derive(Default, Clone)]
struct ProfileState {
    profile_names: Vec<String>,
    selected_profile: Option<String>,
    new_profile_name: String,
    is_applying: bool,
    status_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Idle,
    Cleaning,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSystemIds(bool),
    ToggleMac(bool),
    ToggleVolumeId(bool),
    ToggleSteam(bool),
    ToggleOrphanedGameFolders(bool),
    ToggleAggressive(bool),
    ToggleDryRun(bool),
    Execute,
    CleaningFinished(Vec<String>),
    Backup,
    OpenInspector,
    InspectorLoaded(SystemInfo),
    CloseInspector,
    ProfileSelected(String),
    NewProfileNameChanged(String),
    SaveCurrentAsProfile,
    ProfileSaved(Result<String, String>),
    ApplySelectedProfile,
    ProfileApplied(Vec<String>),
    DeleteSelectedProfile,
    RefreshProfiles,
    OpenRedist,
    Redist(redist_view::RedistMessage),
    OpenThemes,
    CloseThemes,
    ThemeSelected(Theme),
    OpenCustomColors,
    CloseCustomColors,
    PickBackgroundColor,
    PickSurfaceColor,
    PickTextColor,
    PickPrimaryColor,
    PickDangerColor,
    PickSuccessColor,
    ApplyCustomTheme,
    OpenCustomClean,
    CloseCustomClean,
    CustomCleanToggleSystemIds(bool),
    CustomCleanToggleMac(bool),
    CustomCleanToggleVolumeId(bool),
    CustomCleanToggleSteam(bool),
    CustomCleanToggleAggressive(bool),
    ExecuteCustomClean,
    CustomCleanToggleMachineGuid(bool),
    CustomCleanToggleHwProfileGuid(bool),
    CustomCleanToggleProductId(bool),
    CustomCleanToggleRegisteredOwner(bool),
    CustomCleanToggleInstallDate(bool),
    CustomCleanToggleComputerName(bool),
    CustomCleanToggleSteamRegistryHkcu(bool),
    CustomCleanToggleValveRegistryHklm(bool),
    CustomCleanToggleValveRegistryHku(bool),
    CustomCleanToggleFaceitHkcu(bool),
    CustomCleanToggleRiotHkcu(bool),
    CustomCleanToggleEseaHkcu(bool),
    CustomCleanToggleEacHkcu(bool),
    CustomCleanToggleBattleyeHkcu(bool),
    CustomCleanToggleStartupRun(bool),
    CustomCleanToggleAppCompatCache(bool),
    CustomCleanToggleShimCache(bool),
    CustomCleanToggleAppCompatFlags(bool),
    CustomCleanToggleMacAddresses(bool),
    CustomCleanToggleVolumeCdrive(bool),
    CustomCleanToggleLoginUsersVdf(bool),
    CustomCleanToggleConfigVdf(bool),
    CustomCleanToggleLocalconfigVdf(bool),
    CustomCleanToggleSteamAppdataVdf(bool),
    CustomCleanToggleSsfnFiles(bool),
    CustomCleanToggleLibraryfoldersVdf(bool),
    CustomCleanToggleUserdataDir(bool),
    CustomCleanToggleConfigDir(bool),
    CustomCleanToggleLogsDir(bool),
    CustomCleanToggleAppcacheDir(bool),
    CustomCleanToggleDumpDir(bool),
    CustomCleanToggleShadercacheDir(bool),
    CustomCleanToggleDepotcacheDir(bool),
    CustomCleanToggleOrphanedGameFolders(bool),
    CustomCleanToggleSteamAppdataDir(bool),
    CustomCleanToggleValveLocallowDir(bool),
    CustomCleanToggleD3dCache(bool),
    CustomCleanToggleLocalTemp(bool),
    CustomCleanToggleLocalLowTemp(bool),
    CustomCleanToggleUserTemp(bool),
    CustomCleanToggleWindowsTemp(bool),
    CustomCleanToggleCrashDumps(bool),
    CustomCleanToggleWebCache(bool),
    CustomCleanToggleInetCache(bool),
    CustomCleanToggleWindowsCaches(bool),
    CustomCleanToggleWindowsExplorer(bool),
    CustomCleanToggleRecent(bool),
    CustomCleanToggleAutomaticDestinations(bool),
    CustomCleanToggleCustomDestinations(bool),
    CustomCleanToggleTracingDir(bool),
    CustomCleanToggleNvidiaCache(bool),
    CustomCleanToggleWindowsPrefetch(bool),
    CustomCleanToggleMyGames(bool),
    CustomCleanToggleEasyanticheat(bool),
    CustomCleanToggleBattleye(bool),
    CustomCleanToggleFaceit(bool),
    CustomCleanToggleKillSteam(bool),
    CustomCleanToggleKillExplorer(bool),
    CustomCleanToggleLocalTempContents(bool),
    CustomCleanToggleUserTempContents(bool),
    CustomCleanToggleWindowsTempContents(bool),
    CustomCleanToggleWebCacheContents(bool),
    CustomCleanToggleInetCacheContents(bool),
    CustomCleanToggleWindowsCachesContents(bool),
    CustomCleanToggleWindowsExplorerContents(bool),
    CustomCleanToggleRecentContents(bool),
    CustomCleanToggleAutomaticDestinationsContents(bool),
    CustomCleanToggleCustomDestinationsContents(bool),
    CustomCleanToggleTracingDirContents(bool),
    CustomCleanToggleNvidiaCacheContents(bool),
    CustomCleanToggleD3dCacheContents(bool),
    ChangeLanguage(Language),
    OpenLanguageSelector,
    CloseLanguageSelector,
    RainbowTick(std::time::Instant),
    LogMessage(String),
}

impl Application for CleanerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let profile_manager = ProfileManager::load().unwrap_or_default();
        let profile_names = profile_manager.profile_names();
        let current_language = Language::English;
        let translations = i18n::load_translations(current_language);

        (
            Self {
                state: State::Idle,
                options: CleaningOptions::default(),
                log_messages: vec![translations.main_window.ready_message.clone()],
                inspector_open: false,
                inspector_state: InspectorState::default(),
                profile_manager,
                profile_state: ProfileState {
                    profile_names,
                    selected_profile: None,
                    new_profile_name: String::new(),
                    is_applying: false,
                    status_message: None,
                },
                redist_open: false,
                redist_state: redist_view::RedistViewState::default(),
                theme_open: false,
                current_theme: Theme::Dark,
                custom_colors_open: false,
                custom_colors: style::CustomThemeColors::load().unwrap_or_default(),
                custom_theme_active: false,
                custom_clean_open: false,
                custom_clean_options: CleaningOptions {
                    spoof_machine_guid: false,
                    spoof_hw_profile_guid: false,
                    spoof_product_id: false,
                    spoof_registered_owner: false,
                    spoof_install_date: false,
                    spoof_computer_name: false,
                    delete_steam_registry_hkcu: false,
                    delete_valve_registry_hklm: false,
                    delete_valve_registry_hku: false,
                    delete_faceit_hkcu: false,
                    delete_riot_hkcu: false,
                    delete_esea_hkcu: false,
                    delete_eac_hkcu: false,
                    delete_battleye_hkcu: false,
                    delete_startup_run: false,
                    clean_app_compat_cache: false,
                    clean_shim_cache: false,
                    clean_app_compat_flags: false,
                    spoof_mac_addresses: false,
                    spoof_volume_c_drive: false,
                    delete_login_users_vdf: false,
                    delete_config_vdf: false,
                    delete_localconfig_vdf: false,
                    delete_steam_appdata_vdf: false,
                    delete_ssfn_files: false,
                    delete_libraryfolders_vdf: false,
                    delete_userdata_dir: false,
                    delete_config_dir: false,
                    delete_logs_dir: false,
                    delete_appcache_dir: false,
                    delete_dump_dir: false,
                    delete_shadercache_dir: false,
                    delete_depotcache_dir: false,
                    delete_orphaned_game_folders: false,
                    delete_steam_appdata_dir: false,
                    delete_valve_locallow_dir: false,
                    delete_d3d_cache: false,
                    delete_d3d_cache_contents: false,
                    delete_local_temp: false,
                    delete_local_low_temp: false,
                    delete_local_temp_contents: false,
                    delete_user_temp: false,
                    delete_user_temp_contents: false,
                    delete_windows_temp: false,
                    delete_windows_temp_contents: false,
                    delete_crash_dumps: false,
                    delete_web_cache: false,
                    delete_web_cache_contents: false,
                    delete_inet_cache: false,
                    delete_inet_cache_contents: false,
                    delete_windows_caches: false,
                    delete_windows_caches_contents: false,
                    delete_windows_explorer: false,
                    delete_windows_explorer_contents: false,
                    delete_recent: false,
                    delete_recent_contents: false,
                    delete_automatic_destinations: false,
                    delete_automatic_destinations_contents: false,
                    delete_custom_destinations: false,
                    delete_custom_destinations_contents: false,
                    delete_tracing_dir: false,
                    delete_tracing_dir_contents: false,
                    delete_nvidia_cache: false,
                    delete_nvidia_cache_contents: false,
                    delete_windows_prefetch: false,
                    delete_my_games: false,
                    delete_easyanticheat: false,
                    delete_battleye: false,
                    delete_faceit: false,
                    kill_steam_processes: false,
                    kill_explorer: false,
                    ..Default::default()
                },
                rainbow_hue: 0.0,
                current_language,
                translations,
                language_selector_open: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.translations.app_title.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleSystemIds(value) => {
                self.options.spoof_system_ids = value;
                Command::none()
            }
            Message::ToggleMac(value) => {
                self.options.spoof_mac = value;
                Command::none()
            }
            Message::ToggleVolumeId(value) => {
                self.options.spoof_volume_id = value;
                Command::none()
            }
            Message::ToggleSteam(value) => {
                self.options.clean_steam = value;
                Command::none()
            }
            Message::ToggleOrphanedGameFolders(value) => {
                self.options.delete_orphaned_game_folders = value;
                Command::none()
            }
            Message::ToggleAggressive(value) => {
                self.options.clean_aggressive = value;
                Command::none()
            }
            Message::ToggleDryRun(value) => {
                self.options.dry_run = value;
                Command::none()
            }
            Message::Execute => {
                if self.state == State::Idle {
                    if self.options.clean_aggressive {
                        let confirmation = tfd::message_box_yes_no(
                            &self.translations.common.aggressive_cleaning_warning_title,
                            &self.translations.common.aggressive_cleaning_warning_message,
                            tfd::MessageBoxIcon::Warning,
                            tfd::YesNo::No,
                        );
                        if confirmation == tfd::YesNo::No {
                            self.log_messages.push(self.translations.common.aggressive_cleaning_cancelled.clone());
                            return Command::none();
                        }
                    }

                    self.state = State::Cleaning;
                    self.log_messages = vec![self.translations.common.starting_cleaning.clone()];
                    let options = self.options;
                    
                    let (mut tx, rx) = iced::futures::channel::mpsc::unbounded();
                    tokio::spawn(async move {
                        let tx_log = tx.clone();
                        let results = run_all_selected(options, move |log| {
                            let _ = tx_log.unbounded_send(Message::LogMessage(log));
                        }).await;
                        let _ = tx.unbounded_send(Message::CleaningFinished(results));
                    });


                    return Command::run(rx, |msg| msg);
                } else {
                    Command::none()
                }
            }


            Message::CleaningFinished(results) => {
                self.state = State::Idle;
                self.log_messages = results;
                Command::none()
            }
            Message::Backup => {
                match backup::create_backup() {
                    Ok(message) => self.log_messages.push(message),
                    Err(e) => self.log_messages.push(format!("Backup failed: {}", e)),
                }
                Command::none()
            }
            Message::OpenInspector => {
                self.inspector_open = true;
                self.inspector_state.is_loading = true;
                self.profile_state.profile_names = self.profile_manager.profile_names();
                Command::perform(gather_system_info(), Message::InspectorLoaded)
            }
            Message::InspectorLoaded(info) => {
                self.inspector_state.info = info;
                self.inspector_state.is_loading = false;
                Command::none()
            }
            Message::CloseInspector => {
                self.inspector_open = false;
                Command::none()
            }
            Message::ProfileSelected(name) => {
                self.profile_state.selected_profile = Some(name);
                self.profile_state.status_message = None;
                Command::none()
            }
            Message::NewProfileNameChanged(name) => {
                self.profile_state.new_profile_name = name;
                Command::none()
            }
            Message::SaveCurrentAsProfile => {
                let name = self.profile_state.new_profile_name.trim().to_string();
                if name.is_empty() {
                    self.profile_state.status_message = Some(self.translations.common.please_enter_profile_name.clone());
                    return Command::none();
                }

                Command::perform(
                    async move {
                        match HardwareProfile::snapshot_current(name.clone()) {
                            Ok(profile) => {
                                let mut manager = ProfileManager::load().unwrap_or_default();
                                manager.add_or_update_profile(profile);
                                match manager.save() {
                                    Ok(_) => Ok(name),
                                    Err(e) => Err(format!("{}", e)),
                                }
                            }
                            Err(e) => Err(format!("{}", e)),
                        }
                    },
                    Message::ProfileSaved,
                )
            }
            Message::ProfileSaved(result) => {
                match result {
                    Ok(name) => {
                        self.profile_state.status_message = Some(i18n::format_string(&self.translations.common.profile_saved, &[&name]));
                        self.profile_state.new_profile_name.clear();
                        self.profile_manager = ProfileManager::load().unwrap_or_default();
                        self.profile_state.profile_names = self.profile_manager.profile_names();
                        self.profile_state.selected_profile = Some(name);
                    }
                    Err(e) => {
                        self.profile_state.status_message = Some(i18n::format_string(&self.translations.common.failed_to_save, &[&e]));
                    }
                }
                Command::none()
            }
            Message::ApplySelectedProfile => {
                if let Some(name) = &self.profile_state.selected_profile {
                    if let Some(profile) = self.profile_manager.get_profile(name) {
                        self.profile_state.is_applying = true;
                        let profile_clone = profile.clone();
                        let dry_run = self.options.dry_run;

                        let (mut tx, rx) = iced::futures::channel::mpsc::unbounded();
                        tokio::spawn(async move {
                            let tx_log = tx.clone();
                            let results = apply_hardware_profile(profile_clone, dry_run, move |log| {
                                let _ = tx_log.unbounded_send(Message::LogMessage(log));
                            }).await;
                            let _ = tx.unbounded_send(Message::ProfileApplied(results));
                        });
                        return Command::run(rx, |msg| msg);
                    } else {
                        self.profile_state.status_message = Some(self.translations.common.profile_not_found.clone());
                    }
                } else {
                    self.profile_state.status_message = Some(self.translations.common.please_select_profile.clone());
                }
                Command::none()
            }

            Message::ProfileApplied(results) => {
                self.profile_state.is_applying = false;
                self.log_messages = results;
                self.profile_state.status_message = Some(self.translations.common.profile_applied.clone());
                Command::none()
            }
            Message::DeleteSelectedProfile => {
                if let Some(name) = &self.profile_state.selected_profile.clone() {
                    let confirmation = tfd::message_box_yes_no(
                        &self.translations.common.delete_profile_title,
                        &i18n::format_string(&self.translations.common.delete_profile_confirmation, &[name]),
                        tfd::MessageBoxIcon::Question,
                        tfd::YesNo::No,
                    );
                    if confirmation == tfd::YesNo::Yes {
                        self.profile_manager.remove_profile(name);
                        if let Err(e) = self.profile_manager.save() {
                            self.profile_state.status_message = Some(i18n::format_string(&self.translations.common.failed_to_save, &[&e.to_string()]));
                        } else {
                            self.profile_state.status_message = Some(i18n::format_string(&self.translations.common.profile_deleted, &[name]));
                            self.profile_state.profile_names = self.profile_manager.profile_names();
                            self.profile_state.selected_profile = None;
                        }
                    }
                } else {
                    self.profile_state.status_message = Some(self.translations.common.please_select_profile.clone());
                }
                Command::none()
            }
            Message::RefreshProfiles => {
                self.profile_manager = ProfileManager::load().unwrap_or_default();
                self.profile_state.profile_names = self.profile_manager.profile_names();
                self.profile_state.status_message = Some(self.translations.common.profiles_refreshed.clone());
                Command::none()
            }
            Message::OpenRedist => {
                self.redist_open = true;
                Command::none()
            }
            Message::Redist(redist_msg) => {
                match redist_msg {
                    redist_view::RedistMessage::ToggleCommon(v) => { self.redist_state.category_common = v; Command::none() }
                    redist_view::RedistMessage::ToggleDirectX(v) => { self.redist_state.category_directx = v; Command::none() }
                    redist_view::RedistMessage::ToggleDotNet(v) => { self.redist_state.category_dotnet = v; Command::none() }
                    redist_view::RedistMessage::ToggleVCRedist(v) => { self.redist_state.category_vcredist = v; Command::none() }
                    redist_view::RedistMessage::ToggleInstallers(v) => { self.redist_state.category_installers = v; Command::none() }
                    redist_view::RedistMessage::StartScan => {
                        self.redist_state.is_scanning = true;
                        self.redist_state.scan_results = None;
                        let categories = self.redist_state.get_active_categories();
                        Command::perform(async move {
                            #[cfg(windows)]
                            {
                                if let Some(root) = steam::get_steam_root() {
                                    let libs = steam::get_library_folders(&root);
                                    redist::scan_redistributables(&libs, &categories)
                                } else {
                                    Vec::new()
                                }
                            }
                            #[cfg(not(windows))]
                            {
                                let _ = categories;
                                Vec::new()
                            }
                        }, |items| Message::Redist(redist_view::RedistMessage::ScanFinished(items)))
                    }
                    redist_view::RedistMessage::ScanFinished(items) => {
                        self.redist_state.is_scanning = false;
                        self.redist_state.scan_results = Some(items);
                        Command::none()
                    }
                    redist_view::RedistMessage::CleanFoundItems => {
                        if let Some(items) = &self.redist_state.scan_results {
                            let items_clone = items.clone();
                            let dry_run = self.options.dry_run;
                            Command::perform(async move {
                                redist::clean_redistributables(&items_clone, dry_run)
                            }, |logs| Message::Redist(redist_view::RedistMessage::CleanFinished(logs)))
                        } else {
                            Command::none()
                        }
                    }
                    redist_view::RedistMessage::CleanFinished(logs) => {
                        self.redist_state.last_clean_log = Some(logs);
                        self.redist_state.scan_results = Some(Vec::new());
                        Command::none()
                    }
                    redist_view::RedistMessage::Close => {
                        self.redist_open = false;
                        Command::none()
                    }
                }
            }
            Message::OpenThemes => {
                self.theme_open = true;
                Command::none()
            }
            Message::CloseThemes => {
                self.theme_open = false;
                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.current_theme = theme;
                self.custom_theme_active = false;
                Command::none()
            }
            Message::OpenCustomColors => {
                self.custom_colors_open = true;
                Command::none()
            }
            Message::CloseCustomColors => {
                self.custom_colors_open = false;
                Command::none()
            }
            Message::PickBackgroundColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_background_color, tfd::DefaultColorValue::Hex("#1a1b26")) {
                    self.custom_colors.background = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickSurfaceColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_surface_color, tfd::DefaultColorValue::Hex("#24283b")) {
                    self.custom_colors.surface = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickTextColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_text_color, tfd::DefaultColorValue::Hex("#c0caf5")) {
                    self.custom_colors.text = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickPrimaryColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_primary_color, tfd::DefaultColorValue::Hex("#7aa2f7")) {
                    self.custom_colors.primary = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickDangerColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_danger_color, tfd::DefaultColorValue::Hex("#f7768e")) {
                    self.custom_colors.danger = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickSuccessColor => {
                if let Some(rgb) = tfd::color_chooser_dialog(&self.translations.common.pick_success_color, tfd::DefaultColorValue::Hex("#9ece6a")) {
                    self.custom_colors.success = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::ApplyCustomTheme => {
                self.custom_colors_open = false;
                self.custom_theme_active = true;
                let _ = self.custom_colors.save();
                Command::none()
            }
            Message::OpenCustomClean => {
                self.custom_clean_open = true;
                Command::none()
            }
            Message::CloseCustomClean => {
                self.custom_clean_open = false;
                Command::none()
            }
            Message::CustomCleanToggleSystemIds(value) => {
                self.custom_clean_options.spoof_system_ids = value;
                Command::none()
            }
            Message::CustomCleanToggleMac(value) => {
                self.custom_clean_options.spoof_mac = value;
                Command::none()
            }
            Message::CustomCleanToggleVolumeId(value) => {
                self.custom_clean_options.spoof_volume_id = value;
                Command::none()
            }
            Message::CustomCleanToggleSteam(value) => {
                self.custom_clean_options.clean_steam = value;
                Command::none()
            }
            Message::CustomCleanToggleAggressive(value) => {
                self.custom_clean_options.clean_aggressive = value;
                Command::none()
            }
            Message::CustomCleanToggleMachineGuid(value) => { self.custom_clean_options.spoof_machine_guid = value; Command::none() }
            Message::CustomCleanToggleHwProfileGuid(value) => { self.custom_clean_options.spoof_hw_profile_guid = value; Command::none() }
            Message::CustomCleanToggleProductId(value) => { self.custom_clean_options.spoof_product_id = value; Command::none() }
            Message::CustomCleanToggleRegisteredOwner(value) => { self.custom_clean_options.spoof_registered_owner = value; Command::none() }
            Message::CustomCleanToggleInstallDate(value) => { self.custom_clean_options.spoof_install_date = value; Command::none() }
            Message::CustomCleanToggleComputerName(value) => { self.custom_clean_options.spoof_computer_name = value; Command::none() }
            Message::CustomCleanToggleSteamRegistryHkcu(value) => { self.custom_clean_options.delete_steam_registry_hkcu = value; Command::none() }
            Message::CustomCleanToggleValveRegistryHklm(value) => { self.custom_clean_options.delete_valve_registry_hklm = value; Command::none() }
            Message::CustomCleanToggleValveRegistryHku(value) => { self.custom_clean_options.delete_valve_registry_hku = value; Command::none() }
            Message::CustomCleanToggleFaceitHkcu(value) => { self.custom_clean_options.delete_faceit_hkcu = value; Command::none() }
            Message::CustomCleanToggleRiotHkcu(value) => { self.custom_clean_options.delete_riot_hkcu = value; Command::none() }
            Message::CustomCleanToggleEseaHkcu(value) => { self.custom_clean_options.delete_esea_hkcu = value; Command::none() }
            Message::CustomCleanToggleEacHkcu(value) => { self.custom_clean_options.delete_eac_hkcu = value; Command::none() }
            Message::CustomCleanToggleBattleyeHkcu(value) => { self.custom_clean_options.delete_battleye_hkcu = value; Command::none() }
            Message::CustomCleanToggleStartupRun(value) => { self.custom_clean_options.delete_startup_run = value; Command::none() }
            Message::CustomCleanToggleAppCompatCache(value) => { self.custom_clean_options.clean_app_compat_cache = value; Command::none() }
            Message::CustomCleanToggleShimCache(value) => { self.custom_clean_options.clean_shim_cache = value; Command::none() }
            Message::CustomCleanToggleAppCompatFlags(value) => { self.custom_clean_options.clean_app_compat_flags = value; Command::none() }
            Message::CustomCleanToggleMacAddresses(value) => { self.custom_clean_options.spoof_mac_addresses = value; Command::none() }
            Message::CustomCleanToggleVolumeCdrive(value) => { self.custom_clean_options.spoof_volume_c_drive = value; Command::none() }
            Message::CustomCleanToggleLoginUsersVdf(value) => { self.custom_clean_options.delete_login_users_vdf = value; Command::none() }
            Message::CustomCleanToggleConfigVdf(value) => { self.custom_clean_options.delete_config_vdf = value; Command::none() }
            Message::CustomCleanToggleLocalconfigVdf(value) => { self.custom_clean_options.delete_localconfig_vdf = value; Command::none() }
            Message::CustomCleanToggleSteamAppdataVdf(value) => { self.custom_clean_options.delete_steam_appdata_vdf = value; Command::none() }

            Message::CustomCleanToggleSsfnFiles(value) => { self.custom_clean_options.delete_ssfn_files = value; Command::none() }
            Message::CustomCleanToggleLibraryfoldersVdf(value) => { self.custom_clean_options.delete_libraryfolders_vdf = value; Command::none() }
            Message::CustomCleanToggleUserdataDir(value) => { self.custom_clean_options.delete_userdata_dir = value; Command::none() }
            Message::CustomCleanToggleConfigDir(value) => { self.custom_clean_options.delete_config_dir = value; Command::none() }
            Message::CustomCleanToggleLogsDir(value) => { self.custom_clean_options.delete_logs_dir = value; Command::none() }
            Message::CustomCleanToggleAppcacheDir(value) => { self.custom_clean_options.delete_appcache_dir = value; Command::none() }
            Message::CustomCleanToggleDumpDir(value) => { self.custom_clean_options.delete_dump_dir = value; Command::none() }
            Message::CustomCleanToggleShadercacheDir(value) => { self.custom_clean_options.delete_shadercache_dir = value; Command::none() }
            Message::CustomCleanToggleDepotcacheDir(value) => { self.custom_clean_options.delete_depotcache_dir = value; Command::none() }
            Message::CustomCleanToggleOrphanedGameFolders(value) => { self.custom_clean_options.delete_orphaned_game_folders = value; Command::none() }
            Message::CustomCleanToggleSteamAppdataDir(value) => { self.custom_clean_options.delete_steam_appdata_dir = value; Command::none() }
            Message::CustomCleanToggleValveLocallowDir(value) => { self.custom_clean_options.delete_valve_locallow_dir = value; Command::none() }
            Message::CustomCleanToggleD3dCache(value) => { self.custom_clean_options.delete_d3d_cache = value; Command::none() }
            Message::CustomCleanToggleLocalTemp(value) => { self.custom_clean_options.delete_local_temp = value; Command::none() }
            Message::CustomCleanToggleLocalLowTemp(value) => { self.custom_clean_options.delete_local_low_temp = value; Command::none() }
            Message::CustomCleanToggleUserTemp(value) => { self.custom_clean_options.delete_user_temp = value; Command::none() }
            Message::CustomCleanToggleWindowsTemp(value) => { self.custom_clean_options.delete_windows_temp = value; Command::none() }
            Message::CustomCleanToggleCrashDumps(value) => { self.custom_clean_options.delete_crash_dumps = value; Command::none() }
            Message::CustomCleanToggleWebCache(value) => { self.custom_clean_options.delete_web_cache = value; Command::none() }
            Message::CustomCleanToggleInetCache(value) => { self.custom_clean_options.delete_inet_cache = value; Command::none() }
            Message::CustomCleanToggleWindowsCaches(value) => { self.custom_clean_options.delete_windows_caches = value; Command::none() }
            Message::CustomCleanToggleWindowsExplorer(value) => { self.custom_clean_options.delete_windows_explorer = value; Command::none() }
            Message::CustomCleanToggleRecent(value) => { self.custom_clean_options.delete_recent = value; Command::none() }
            Message::CustomCleanToggleAutomaticDestinations(value) => { self.custom_clean_options.delete_automatic_destinations = value; Command::none() }
            Message::CustomCleanToggleCustomDestinations(value) => { self.custom_clean_options.delete_custom_destinations = value; Command::none() }
            Message::CustomCleanToggleTracingDir(value) => { self.custom_clean_options.delete_tracing_dir = value; Command::none() }
            Message::CustomCleanToggleNvidiaCache(value) => { self.custom_clean_options.delete_nvidia_cache = value; Command::none() }
            Message::CustomCleanToggleWindowsPrefetch(value) => { self.custom_clean_options.delete_windows_prefetch = value; Command::none() }
            Message::CustomCleanToggleMyGames(value) => { self.custom_clean_options.delete_my_games = value; Command::none() }
            Message::CustomCleanToggleEasyanticheat(value) => { self.custom_clean_options.delete_easyanticheat = value; Command::none() }
            Message::CustomCleanToggleBattleye(value) => { self.custom_clean_options.delete_battleye = value; Command::none() }
            Message::CustomCleanToggleFaceit(value) => { self.custom_clean_options.delete_faceit = value; Command::none() }
            Message::CustomCleanToggleKillSteam(value) => { self.custom_clean_options.kill_steam_processes = value; Command::none() }
            Message::CustomCleanToggleKillExplorer(value) => { self.custom_clean_options.kill_explorer = value; Command::none() }
            Message::CustomCleanToggleLocalTempContents(value) => { self.custom_clean_options.delete_local_temp_contents = value; Command::none() }
            Message::CustomCleanToggleUserTempContents(value) => { self.custom_clean_options.delete_user_temp_contents = value; Command::none() }
            Message::CustomCleanToggleWindowsTempContents(value) => { self.custom_clean_options.delete_windows_temp_contents = value; Command::none() }
            Message::CustomCleanToggleWebCacheContents(value) => { self.custom_clean_options.delete_web_cache_contents = value; Command::none() }
            Message::CustomCleanToggleInetCacheContents(value) => { self.custom_clean_options.delete_inet_cache_contents = value; Command::none() }
            Message::CustomCleanToggleWindowsCachesContents(value) => { self.custom_clean_options.delete_windows_caches_contents = value; Command::none() }
            Message::CustomCleanToggleWindowsExplorerContents(value) => { self.custom_clean_options.delete_windows_explorer_contents = value; Command::none() }
            Message::CustomCleanToggleRecentContents(value) => { self.custom_clean_options.delete_recent_contents = value; Command::none() }
            Message::CustomCleanToggleAutomaticDestinationsContents(value) => { self.custom_clean_options.delete_automatic_destinations_contents = value; Command::none() }
            Message::CustomCleanToggleCustomDestinationsContents(value) => { self.custom_clean_options.delete_custom_destinations_contents = value; Command::none() }
            Message::CustomCleanToggleTracingDirContents(value) => { self.custom_clean_options.delete_tracing_dir_contents = value; Command::none() }
            Message::CustomCleanToggleNvidiaCacheContents(value) => { self.custom_clean_options.delete_nvidia_cache_contents = value; Command::none() }
            Message::CustomCleanToggleD3dCacheContents(value) => { self.custom_clean_options.delete_d3d_cache_contents = value; Command::none() }
            Message::ExecuteCustomClean => {
                if self.state == State::Idle {
                    if self.custom_clean_options.clean_aggressive {
                        let confirmation = tfd::message_box_yes_no(
                            &self.translations.common.aggressive_cleaning_warning_title,
                            &self.translations.common.aggressive_cleaning_warning_message,
                            tfd::MessageBoxIcon::Warning,
                            tfd::YesNo::No,
                        );
                        if confirmation == tfd::YesNo::No {
                            self.log_messages.push(self.translations.common.aggressive_cleaning_cancelled.clone());
                            return Command::none();
                        }
                    }

                    self.state = State::Cleaning;
                    self.log_messages = vec![self.translations.common.starting_custom_cleaning.clone()];
                    let options = self.custom_clean_options;

                    let (mut tx, rx) = iced::futures::channel::mpsc::unbounded();
                    tokio::spawn(async move {
                        let tx_log = tx.clone();
                        let results = run_all_selected(options, move |log| {
                            let _ = tx_log.unbounded_send(Message::LogMessage(log));
                        }).await;
                        let _ = tx.unbounded_send(Message::CleaningFinished(results));
                    });
                    return Command::run(rx, |msg| msg);
                } else {
                    Command::none()
                }
            }

            Message::ChangeLanguage(lang) => {
                self.current_language = lang;
                self.translations = i18n::load_translations(lang);
                self.language_selector_open = false;
                Command::none()
            }
            Message::OpenLanguageSelector => {
                self.language_selector_open = true;
                Command::none()
            }
            Message::CloseLanguageSelector => {
                self.language_selector_open = false;
                Command::none()
            }
            Message::RainbowTick(_) => {
                self.rainbow_hue += 0.005;
                if self.rainbow_hue > 1.0 {
                    self.rainbow_hue = 0.0;
                }
                Command::none()
            }
            Message::LogMessage(log) => {
                self.log_messages.push(log);
                Command::none()
            }
        }
    }


    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(Message::RainbowTick)
    }

    fn view(&self) -> Element<'_, Message> {
        if self.inspector_open {
            self.view_inspector_window()
        } else if self.redist_open {
             let active_colors = if self.custom_theme_active {
                Some(self.custom_colors)
            } else {
                None
            };
            redist_view::view(&self.redist_state, active_colors, &self.translations, self.current_language).map(Message::Redist)
        } else if self.custom_clean_open {
            self.view_custom_clean_window()
        } else if self.custom_colors_open {
            self.view_custom_colors()
        } else if self.theme_open {
            self.view_theme_selection()
        } else {
            self.view_main_window()
        }
    }

    fn theme(&self) -> Theme {
        self.current_theme.clone()
    }
}

impl CleanerApp {
    fn view_main_window(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        let lang_font = self.current_language.font();

        fn make_toggler<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message, colors: Option<style::CustomThemeColors>, font: Option<iced::Font>) -> Element<'a, Message> {
            let toggler = toggler(Some(label.to_string()), value, msg)
                .style(iced::theme::Toggler::Custom(Box::new(style::CustomTogglerStyle { custom_colors: colors })))
                .width(Length::Fill)
                .text_size(15);
            if let Some(f) = font {
                toggler.font(f).into()
            } else {
                toggler.into()
            }
        }

        fn make_checkbox<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message, colors: Option<style::CustomThemeColors>, font: Option<iced::Font>) -> Element<'a, Message> {
            checkbox(label, value)
                .on_toggle(msg)
                .style(iced::theme::Checkbox::Custom(Box::new(style::CustomCheckboxStyle { custom_colors: colors })))
                .width(Length::Fill)
                .text_size(14)
                .font(font.unwrap_or(iced::Font::DEFAULT))
                .spacing(8)
                .into()
        }

        let system_spoofing_options = column![
            text(&self.translations.main_window.system_spoofing).size(15).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            make_toggler(&self.translations.main_window.spoof_system_ids, self.options.spoof_system_ids, Message::ToggleSystemIds, active_colors, lang_font),
            make_toggler(&self.translations.main_window.spoof_mac_address, self.options.spoof_mac, Message::ToggleMac, active_colors, lang_font),
            make_toggler(&self.translations.main_window.spoof_volume_id, self.options.spoof_volume_id, Message::ToggleVolumeId, active_colors, lang_font),
        ]
        .spacing(6);

        let steam_cleaning_options = column![
            text(&self.translations.main_window.steam_cleaning).size(15).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            make_toggler(&self.translations.main_window.clean_steam, self.options.clean_steam, Message::ToggleSteam, active_colors, lang_font),
            make_toggler(&self.translations.main_window.delete_orphaned_game_folders, self.options.delete_orphaned_game_folders, Message::ToggleOrphanedGameFolders, active_colors, lang_font),
        ]
        .spacing(6);

        let aggressive_cleaning_options = column![
            text(&self.translations.main_window.aggressive_cleaning).size(15).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            make_toggler(&self.translations.main_window.aggressive_clean, self.options.clean_aggressive, Message::ToggleAggressive, active_colors, lang_font),
        ]
        .spacing(6);

        let options_content = column![
            system_spoofing_options,
            steam_cleaning_options,
            aggressive_cleaning_options
        ]
        .spacing(10);

        let options_box = container(options_content)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })))
            .padding(12)
            .width(Length::Fill);

        let inspector_button = button(text(&self.translations.main_window.inspector_and_profiles).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::OpenInspector)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let redist_button = button(text(&self.translations.main_window.steam_redist_cleaner_beta).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::OpenRedist)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let themes_button = button(text(&self.translations.main_window.themes_and_appearance).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::OpenThemes)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let (button_text_str, on_press_message) = match self.state {
            State::Idle => (&self.translations.main_window.execute_cleaning, Some(Message::Execute)),
            State::Cleaning => (&self.translations.main_window.cleaning, None),
        };

        let mut execute_button = button(text(button_text_str).size(15).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(16)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(style::SuccessButtonStyle { custom_colors: active_colors })));

        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let backup_button = button(text(&self.translations.main_window.backup_steam_data).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::Backup)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let dry_run_toggle = make_toggler(&self.translations.main_window.simulation_mode_dry_run, self.options.dry_run, Message::ToggleDryRun, active_colors, lang_font);

        let simulation_mode_box = container(
            column![
                text("Simulation Mode").size(16).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                Space::with_height(Length::Fixed(8.0)),
                dry_run_toggle
            ]
            .spacing(4)
        )
        .padding(15)
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })));

        let lang_code = self.current_language.name();
        let lang_svg = r#"<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12" cy="12" r="9" fill="white" stroke="black" stroke-width="1.5"/>
            <ellipse cx="12" cy="12" rx="9" ry="2.5" fill="none" stroke="black" stroke-width="1.2"/>
            <path d="M3 12 L21 12" stroke="black" stroke-width="1.2"/>
            <path d="M12 3 L12 21" stroke="black" stroke-width="1.2"/>
        </svg>"#;
        
        let lang_icon = svg::Handle::from_memory(lang_svg.as_bytes());

        let language_button = button(
            row![
                svg(lang_icon)
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0)),
                text(lang_code).size(13).font(lang_font.unwrap_or(iced::Font::DEFAULT))
            ].spacing(6).align_items(iced::Alignment::Center)
        )
        .on_press(Message::OpenLanguageSelector)
        .padding(8)
        .width(Length::Shrink)
        .style(iced::theme::Button::Custom(Box::new(style::IconButtonStyle { custom_colors: active_colors })));

        let custom_clean_button = button(text(&self.translations.main_window.custom_clean).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::OpenCustomClean)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let left_panel_content = column![
            Space::with_height(Length::Fixed(48.0)),
            options_box,
            Space::with_height(Length::Fixed(15.0)),
            simulation_mode_box,
            Space::with_height(Length::Fixed(5.0)),
            execute_button,
            Space::with_height(Length::Fixed(4.0)),
            custom_clean_button,
            Space::with_height(Length::Fixed(4.0)),
            backup_button,
            Space::with_height(Length::Fixed(4.0)),
            inspector_button,
            Space::with_height(Length::Fixed(4.0)),
            redist_button,
            Space::with_height(Length::Fixed(4.0)),
            themes_button,
            Space::with_height(Length::Fixed(5.0)),
        ]
        .spacing(4)
        .width(Length::Fill);

        let left_panel = container(left_panel_content)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(12);

        let log_output = self.log_messages.iter().fold(Column::new().spacing(4), |col, msg| {
            col.push(text(msg.clone()).font(iced::Font::MONOSPACE).size(13))
        });

        let console_box = container(scrollable(log_output))
            .style(iced::theme::Container::Custom(Box::new(style::ConsoleContainerStyle { custom_colors: active_colors })))
            .padding(15)
            .width(Length::Fill)
            .height(Length::Fill);

        let log_header = row![
            text(&self.translations.main_window.verbose_log_output).size(16).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            Space::with_width(Length::Fill),
            language_button,
        ].align_items(iced::Alignment::Center);

        let right_panel = Column::new()
            .push(log_header)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(console_box)
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .padding(15);

        let main_content = Row::new()
            .push(left_panel)
            .push(right_panel)
            .height(Length::Fill);

        let main_container = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle { custom_colors: active_colors })));

        if self.language_selector_open {
            let language_buttons: Column<'_, Message> = Language::all().iter().fold(
                Column::new().spacing(8),
                |col, &lang| {
                    let font = lang.font();
                    let is_selected = self.current_language == lang;
                    let mut label = text(lang.name())
                        .size(14)
                        .horizontal_alignment(Horizontal::Center)
                        .style(Color::WHITE);
                    
                    if let Some(f) = font {
                        label = label.font(f);
                    }
                    let btn = button(label)
                        .on_press(Message::ChangeLanguage(lang))
                        .padding(5)
                        .width(Length::Fill)
                        .height(Length::Shrink)
                        .style(iced::theme::Button::Custom(Box::new(style::LanguageButtonStyle { 
                            custom_colors: active_colors,
                            is_selected 
                        })));
                    col.push(btn)
                }
            );

            let close_button_text_color = if let Some(colors) = active_colors {
                colors.background
            } else {
                match self.current_theme {
                    iced::Theme::Light | iced::Theme::SolarizedLight => Color::BLACK,
                    _ => Color::WHITE,
                }
            };
            
            let overlay_content = column![
                text("Select Language").size(18).horizontal_alignment(Horizontal::Center).style(style::title_color(&self.current_theme)),
                Space::with_height(Length::Fixed(12.0)),
                language_buttons,
                Space::with_height(Length::Fixed(12.0)),
                button(text("Close").size(14).horizontal_alignment(Horizontal::Center).style(close_button_text_color))
                    .on_press(Message::CloseLanguageSelector)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fixed(40.0))
                    .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })))
            ]
            .align_items(iced::Alignment::Center)
            .spacing(0);

            let overlay = container(overlay_content)
                .width(Length::Fixed(350.0))
                .padding(20)
                .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })));

            let centered = row![
                Space::with_width(Length::Fill),
                column![
                    Space::with_height(Length::Fill),
                    overlay,
                    Space::with_height(Length::Fill),
                ],
                Space::with_width(Length::Fill),
            ];

            container(centered)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(style::ModalOverlayStyle { custom_colors: active_colors })))
                .into()
        } else {
            main_container.into()
        }
    }

    fn view_inspector_window(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        let lang_font = self.current_language.font();

        let header = container(
            text(&self.translations.inspector.title).size(24).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        let system_info_section = if self.inspector_state.is_loading {
            Column::new().push(text(&self.translations.inspector.loading_system_info).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
        } else {
            let info = &self.inspector_state.info;
            let info_col = column![
                text(&self.translations.inspector.current_system_hardware_ids).size(18).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                Space::with_height(Length::Fixed(10.0)),
                text(i18n::format_string(&self.translations.inspector.machine_guid, &[&info.machine_guid])).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                text(i18n::format_string(&self.translations.inspector.product_id, &[&info.product_id])).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                text(i18n::format_string(&self.translations.inspector.computer_name, &[&info.computer_name])).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                text(i18n::format_string(&self.translations.inspector.volume_id_c, &[&info.volume_id])).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                Space::with_height(Length::Fixed(10.0)),
                text(&self.translations.inspector.mac_addresses).size(16).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            ]
            .spacing(8);

            let adapters_col = info.network_adapters.iter().fold(Column::new().spacing(3), |col, (_desc, mac)| {
                col.push(text(format!("  - {}", mac)).size(14))
            });

            Column::new()
                .push(info_col)
                .push(adapters_col)
        };

        let system_info_box = container(system_info_section)
            .padding(15)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })));

        let profile_header = text(&self.translations.inspector.hardware_id_profile_manager).size(18).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT));

        let profile_dropdown: Element<'_, Message> = pick_list(
            self.profile_state.profile_names.clone(),
            self.profile_state.selected_profile.clone(),
            Message::ProfileSelected,
        )
        .placeholder(&self.translations.inspector.select_a_profile)
        .width(Length::Fill)
        .font(lang_font.unwrap_or(iced::Font::DEFAULT))
        .into();

        let dropdown_row = Row::new()
            .push(text(&self.translations.inspector.load_profile).size(14).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .push(profile_dropdown)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        let apply_button = button(
            text(&self.translations.inspector.apply).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(8)
            .width(Length::FillPortion(1))
            .on_press(Message::ApplySelectedProfile)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let delete_button = button(
            text(&self.translations.inspector.delete).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(8)
            .width(Length::FillPortion(1))
            .on_press(Message::DeleteSelectedProfile)
            .style(iced::theme::Button::Custom(Box::new(style::DangerButtonStyle { custom_colors: active_colors })));

        let profile_actions_row = Row::new()
            .push(apply_button)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(delete_button)
            .spacing(5);

        let new_profile_input = text_input(
            &self.translations.inspector.enter_profile_name,
            &self.profile_state.new_profile_name,
        )
        .on_input(Message::NewProfileNameChanged)
        .padding(10)
        .font(lang_font.unwrap_or(iced::Font::DEFAULT))
        .width(Length::Fill);

        let save_button = button(
            text(&self.translations.inspector.save_current_as_profile).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::SaveCurrentAsProfile)
            .style(iced::theme::Button::Custom(Box::new(style::SuccessButtonStyle { custom_colors: active_colors })));

        let status_text: Element<'_, Message> = if let Some(msg) = &self.profile_state.status_message {
            text(msg).size(13).font(lang_font.unwrap_or(iced::Font::DEFAULT)).into()
        } else {
            Space::with_height(Length::Fixed(13.0)).into()
        };

        let profile_details: Element<'_, Message> = if let Some(name) = &self.profile_state.selected_profile {
            if let Some(profile) = self.profile_manager.get_profile(name) {
                let mac_count = profile.mac_addresses.len();
                let vol_count = profile.volume_ids.len();
                column![
                    text(i18n::format_string(&self.translations.inspector.profile, &[&profile.name])).size(14).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                    text(i18n::format_string(&self.translations.inspector.created, &[&profile.created_at])).size(12).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                    text(i18n::format_string(&self.translations.inspector.mac_addresses_count, &[&mac_count.to_string(), &vol_count.to_string()])).size(12).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                ]
                .spacing(3)
                .into()
            } else {
                Space::with_height(Length::Fixed(1.0)).into()
            }
        } else {
            text(&self.translations.inspector.select_profile_details)
                .size(13)
                .font(lang_font.unwrap_or(iced::Font::DEFAULT))
                .into()
        };

        let profile_section = column![
            profile_header,
            Space::with_height(Length::Fixed(15.0)),
            dropdown_row,
            Space::with_height(Length::Fixed(10.0)),
            profile_details,
            Space::with_height(Length::Fixed(10.0)),
            profile_actions_row,
            Space::with_height(Length::Fixed(20.0)),
            text(&self.translations.inspector.create_new_profile).size(14).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            Space::with_height(Length::Fixed(5.0)),
            new_profile_input,
            Space::with_height(Length::Fixed(10.0)),
            save_button,
            Space::with_height(Length::Fixed(10.0)),
            status_text,
        ]
        .spacing(2);

        let profile_box = container(profile_section)
            .padding(15)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })));

        let scrollable_content = column![
            system_info_box,
            Space::with_height(Length::Fixed(20.0)),
            profile_box,
        ]
        .spacing(10)
        .width(Length::Fill);

        let back_button = button(
            text(&self.translations.inspector.back_to_main).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(10)
            .width(Length::Fixed(180.0))
            .on_press(Message::CloseInspector)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let footer = container(back_button)
            .padding(20)
            .width(Length::Fill)
            .center_x()
            .align_y(iced::alignment::Vertical::Center);

        let main_layout = column![
            header,
            container(scrollable(scrollable_content))
                .width(Length::Fill)
                .height(Length::Fill),
            footer,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(main_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle { custom_colors: active_colors })))
            .into()
    }

    fn view_theme_selection(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        let lang_font = self.current_language.font();

        let header = container(
            text(&self.translations.themes.title).size(24).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        fn theme_btn<'a>(label: &'a str, theme: Theme, current: &Theme, colors: Option<style::CustomThemeColors>, font: Option<iced::Font>) -> Element<'a, Message> {
            let is_selected = theme == *current;

            let btn_style = if is_selected {
                style::ThemedButtonStyle::Success(colors)
            } else {
                style::ThemedButtonStyle::Primary(colors)
            };

            button(text(label).size(16).horizontal_alignment(iced::alignment::Horizontal::Center).font(font.unwrap_or(iced::Font::DEFAULT)))
                .padding(15)
                .width(Length::Fill)
                .on_press(Message::ThemeSelected(theme))
                .style(iced::theme::Button::Custom(Box::new(btn_style)))
                .into()
        }

        let theme_buttons = column![
            text(&self.translations.themes.select_application_theme).size(18).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            Space::with_height(Length::Fixed(15.0)),
            theme_btn(&self.translations.themes.red_retro_default, Theme::Dark, &self.current_theme, active_colors, lang_font),
            theme_btn(&self.translations.themes.light_mode, Theme::Light, &self.current_theme, active_colors, lang_font),
            theme_btn(&self.translations.themes.neutral_dark, Theme::Dracula, &self.current_theme, active_colors, lang_font),
            theme_btn(&self.translations.themes.ultra_dark, Theme::Nord, &self.current_theme, active_colors, lang_font),
            theme_btn(&self.translations.themes.cream, Theme::SolarizedLight, &self.current_theme, active_colors, lang_font),
            Space::with_height(Length::Fixed(20.0)),
            button(text(&self.translations.themes.custom_colors).size(16).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
                .padding(15)
                .width(Length::Fill)
                .on_press(Message::OpenCustomColors)
                .style(iced::theme::Button::Custom(Box::new(style::ThemedButtonStyle::Primary(active_colors)))),
        ]
        .spacing(15)
        .width(Length::Fixed(400.0));

        let content = container(theme_buttons)
            .padding(30)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })))
            .center_x();

        let back_button = button(
            text(&self.translations.themes.back_to_main).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(10)
            .width(Length::Fixed(180.0))
            .on_press(Message::CloseThemes)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let footer = container(back_button)
            .padding(20)
            .width(Length::Fill)
            .center_x()
            .align_y(iced::alignment::Vertical::Center);

        let layout = column![
            header,
            container(scrollable(content))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
            footer
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle { custom_colors: active_colors })))
            .into()
    }

    fn view_custom_colors(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        let lang_font = self.current_language.font();

        let header = container(
            text(&self.translations.theme_customizer.title).size(24).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        let color_btn = |label: &str, color: Color, msg: Message, font: Option<iced::Font>| -> Element<Message> {
            column![
                text(label).size(14).style(active_colors.map_or_else(|| style::TITLE_COLOR, |c| c.text)).font(font.unwrap_or(iced::Font::DEFAULT)),
                button(
                    container(Space::with_width(Length::Fill))
                        .width(Length::Fill)
                        .height(Length::Fixed(40.0))
                        .style(iced::theme::Container::Custom(Box::new(style::ColorPreviewStyle { color })))
                )
                .padding(0)
                .width(Length::Fill)
                .on_press(msg)
                .style(iced::theme::Button::Custom(Box::new(style::ThemedButtonStyle::Primary(None))))
            ]
            .spacing(5)
            .into()
        };

        let active_colors = Some(self.custom_colors);

        let core_colors = container(
            column![
                text(&self.translations.theme_customizer.core_layout).size(18).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                color_btn(&self.translations.theme_customizer.background, self.custom_colors.background, Message::PickBackgroundColor, lang_font),
                color_btn(&self.translations.theme_customizer.surface_panels, self.custom_colors.surface, Message::PickSurfaceColor, lang_font),
                color_btn(&self.translations.theme_customizer.text_color, self.custom_colors.text, Message::PickTextColor, lang_font),
            ]
            .spacing(15)
            .width(Length::Fill)
        )
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })))
        .width(Length::FillPortion(1));

        let accent_colors = container(
            column![
                text(&self.translations.theme_customizer.accents_and_status).size(18).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
                color_btn(&self.translations.theme_customizer.primary_accent, self.custom_colors.primary, Message::PickPrimaryColor, lang_font),
                color_btn(&self.translations.theme_customizer.danger_error, self.custom_colors.danger, Message::PickDangerColor, lang_font),
                color_btn(&self.translations.theme_customizer.success_go, self.custom_colors.success, Message::PickSuccessColor, lang_font),
            ]
            .spacing(15)
            .width(Length::Fill)
        )
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })))
        .width(Length::FillPortion(1));

        let color_columns = row![core_colors, accent_colors].spacing(20);

        let preview_header = container(text(&self.translations.theme_customizer.live_preview).size(16).style(active_colors.map_or_else(|| style::TITLE_COLOR, |c| c.text)).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(style::PreviewBoxStyle {
                bg: self.custom_colors.surface,
                text: self.custom_colors.text
            })))
            .width(Length::Fill);

        let preview_content = column![
            text(&self.translations.theme_customizer.sample_window_content).size(18).style(self.custom_colors.text).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            text(&self.translations.theme_customizer.sample_description).size(14).style(self.custom_colors.text).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            row![
                button(text(&self.translations.theme_customizer.primary_action).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::Button::Custom(Box::new(style::ColorPreviewStyle { color: self.custom_colors.primary }))),
                button(text(&self.translations.theme_customizer.danger_zone).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::Button::Custom(Box::new(style::ColorPreviewStyle { color: self.custom_colors.danger }))),
            ].spacing(10),
            container(text(&self.translations.theme_customizer.success_message).style(Color::WHITE).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
                .padding(10)
                .width(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(style::ColorPreviewStyle { color: self.custom_colors.success })))
        ]
        .spacing(15)
        .padding(20);

        let preview_container = container(
            column![
                preview_header,
                preview_content
            ]
        )
        .style(iced::theme::Container::Custom(Box::new(style::PreviewBoxStyle {
            bg: self.custom_colors.background,
            text: self.custom_colors.text
        })))
        .width(Length::Fill)
        .padding(10);

        let main_col = column![
            color_columns,
            Space::with_height(Length::Fixed(20.0)),
            text(&self.translations.theme_customizer.preview).size(18).font(lang_font.unwrap_or(iced::Font::DEFAULT)),
            preview_container,
            Space::with_height(Length::Fixed(20.0)),
            button(text(&self.translations.theme_customizer.apply_changes).size(16).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
                    .padding(15)
                    .width(Length::Fill)
                    .on_press(Message::ApplyCustomTheme)
                    .style(iced::theme::Button::Custom(Box::new(style::ThemedButtonStyle::Success(None))))
        ]
        .spacing(10)
        .width(Length::Fill);

        let back_button = button(text(&self.translations.theme_customizer.back_to_themes).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(10)
            .width(Length::Fixed(180.0))
            .on_press(Message::CloseCustomColors)
            .style(iced::theme::Button::Custom(Box::new(style::ThemedButtonStyle::Primary(None))));

        let footer = container(back_button)
            .padding(20)
            .width(Length::Fill)
            .center_x()
            .align_y(iced::alignment::Vertical::Center);

        let layout = column![
            header,
            container(scrollable(main_col)).width(Length::Fill).height(Length::Fill).padding(30),
            footer
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle::default())))
            .into()
    }

    fn view_custom_clean_window(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        let lang_font = self.current_language.font();

        fn make_checkbox<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message, colors: Option<style::CustomThemeColors>, font: Option<iced::Font>) -> Element<'a, Message> {
            checkbox(label, value)
                .on_toggle(msg)
                .style(iced::theme::Checkbox::Custom(Box::new(style::CustomCheckboxStyle { custom_colors: colors })))
                .width(Length::Fill)
                .text_size(14)
                .font(font.unwrap_or(iced::Font::DEFAULT))
                .spacing(8)
                .into()
        }

        fn make_section<'a>(title: &'a str, checkboxes: Column<'a, Message>) -> Element<'a, Message> {
            container(
                column![
                    text(title).size(16).style(iced::Color::from_rgb(0.5, 0.7, 1.0)),
                    Space::with_height(Length::Fixed(8.0)),
                    checkboxes
                ]
                .spacing(4)
            )
            .padding(15)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: None })))
            .into()
        }

        let header = container(
            text(&self.translations.custom_clean.title).size(24).style(style::title_color(&self.current_theme)).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        let processes_section = make_section(
            &self.translations.custom_clean.processes,
            column![
                make_checkbox(&self.translations.custom_clean.kill_steam_processes, self.custom_clean_options.kill_steam_processes, Message::CustomCleanToggleKillSteam, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.kill_explorer, self.custom_clean_options.kill_explorer, Message::CustomCleanToggleKillExplorer, active_colors, lang_font),
            ].spacing(10)
        );

        let system_id_section = make_section(
            &self.translations.custom_clean.system_id_spoofing,
            column![
                make_checkbox(&self.translations.custom_clean.spoof_machine_guid, self.custom_clean_options.spoof_machine_guid, Message::CustomCleanToggleMachineGuid, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_hw_profile_guid, self.custom_clean_options.spoof_hw_profile_guid, Message::CustomCleanToggleHwProfileGuid, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_product_id, self.custom_clean_options.spoof_product_id, Message::CustomCleanToggleProductId, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_registered_owner, self.custom_clean_options.spoof_registered_owner, Message::CustomCleanToggleRegisteredOwner, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_install_date, self.custom_clean_options.spoof_install_date, Message::CustomCleanToggleInstallDate, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_computer_name, self.custom_clean_options.spoof_computer_name, Message::CustomCleanToggleComputerName, active_colors, lang_font),
            ].spacing(10)
        );

        let game_tracking_section = make_section(
            &self.translations.custom_clean.registry_game_tracking,
            column![
                make_checkbox(&self.translations.custom_clean.delete_steam_registry_hkcu, self.custom_clean_options.delete_steam_registry_hkcu, Message::CustomCleanToggleSteamRegistryHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_valve_registry_hklm, self.custom_clean_options.delete_valve_registry_hklm, Message::CustomCleanToggleValveRegistryHklm, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_valve_registry_hku, self.custom_clean_options.delete_valve_registry_hku, Message::CustomCleanToggleValveRegistryHku, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_faceit_hkcu, self.custom_clean_options.delete_faceit_hkcu, Message::CustomCleanToggleFaceitHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_riot_hkcu, self.custom_clean_options.delete_riot_hkcu, Message::CustomCleanToggleRiotHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_esea_hkcu, self.custom_clean_options.delete_esea_hkcu, Message::CustomCleanToggleEseaHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_eac_hkcu, self.custom_clean_options.delete_eac_hkcu, Message::CustomCleanToggleEacHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_battleye_hkcu, self.custom_clean_options.delete_battleye_hkcu, Message::CustomCleanToggleBattleyeHkcu, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_startup_run, self.custom_clean_options.delete_startup_run, Message::CustomCleanToggleStartupRun, active_colors, lang_font),
            ].spacing(10)
        );

        let registry_caches_section = make_section(
            &self.translations.custom_clean.registry_system_caches,
            column![
                make_checkbox(&self.translations.custom_clean.clean_app_compat_cache, self.custom_clean_options.clean_app_compat_cache, Message::CustomCleanToggleAppCompatCache, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.clean_shim_cache, self.custom_clean_options.clean_shim_cache, Message::CustomCleanToggleShimCache, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.clean_app_compat_flags, self.custom_clean_options.clean_app_compat_flags, Message::CustomCleanToggleAppCompatFlags, active_colors, lang_font),
            ].spacing(10)
        );

        let mac_volume_section = make_section(
            &self.translations.custom_clean.mac_volume_spoofing,
            column![
                make_checkbox(&self.translations.custom_clean.spoof_mac_addresses, self.custom_clean_options.spoof_mac_addresses, Message::CustomCleanToggleMacAddresses, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.spoof_volume_c_drive, self.custom_clean_options.spoof_volume_c_drive, Message::CustomCleanToggleVolumeCdrive, active_colors, lang_font),
            ].spacing(10)
        );

        let steam_login_section = make_section(
            &self.translations.custom_clean.steam_login_files,
            column![
                make_checkbox(&self.translations.custom_clean.delete_login_users_vdf, self.custom_clean_options.delete_login_users_vdf, Message::CustomCleanToggleLoginUsersVdf, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_config_vdf, self.custom_clean_options.delete_config_vdf, Message::CustomCleanToggleConfigVdf, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_localconfig_vdf, self.custom_clean_options.delete_localconfig_vdf, Message::CustomCleanToggleLocalconfigVdf, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_steam_appdata_vdf, self.custom_clean_options.delete_steam_appdata_vdf, Message::CustomCleanToggleSteamAppdataVdf, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_ssfn_files, self.custom_clean_options.delete_ssfn_files, Message::CustomCleanToggleSsfnFiles, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_libraryfolders_vdf, self.custom_clean_options.delete_libraryfolders_vdf, Message::CustomCleanToggleLibraryfoldersVdf, active_colors, lang_font),
            ].spacing(10)
        );

        let steam_dirs_section = make_section(
            &self.translations.custom_clean.steam_directories,
            column![
                make_checkbox(&self.translations.custom_clean.delete_userdata_dir, self.custom_clean_options.delete_userdata_dir, Message::CustomCleanToggleUserdataDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_config_dir, self.custom_clean_options.delete_config_dir, Message::CustomCleanToggleConfigDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_logs_dir, self.custom_clean_options.delete_logs_dir, Message::CustomCleanToggleLogsDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_appcache_dir, self.custom_clean_options.delete_appcache_dir, Message::CustomCleanToggleAppcacheDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_dump_dir, self.custom_clean_options.delete_dump_dir, Message::CustomCleanToggleDumpDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_shadercache_dir, self.custom_clean_options.delete_shadercache_dir, Message::CustomCleanToggleShadercacheDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_depotcache_dir, self.custom_clean_options.delete_depotcache_dir, Message::CustomCleanToggleDepotcacheDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_orphaned_game_folders, self.custom_clean_options.delete_orphaned_game_folders, Message::CustomCleanToggleOrphanedGameFolders, active_colors, lang_font),
            ].spacing(10)
        );

        let system_cache_section = make_section(
            &self.translations.custom_clean.system_cache_directories,
            column![
                make_checkbox(&self.translations.custom_clean.delete_steam_appdata_dir, self.custom_clean_options.delete_steam_appdata_dir, Message::CustomCleanToggleSteamAppdataDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_valve_locallow_dir, self.custom_clean_options.delete_valve_locallow_dir, Message::CustomCleanToggleValveLocallowDir, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_d3d_cache, self.custom_clean_options.delete_d3d_cache, Message::CustomCleanToggleD3dCache, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_local_temp, self.custom_clean_options.delete_local_temp, Message::CustomCleanToggleLocalTemp, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_local_low_temp, self.custom_clean_options.delete_local_low_temp, Message::CustomCleanToggleLocalLowTemp, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_user_temp, self.custom_clean_options.delete_user_temp, Message::CustomCleanToggleUserTemp, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_windows_temp, self.custom_clean_options.delete_windows_temp, Message::CustomCleanToggleWindowsTemp, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_crash_dumps, self.custom_clean_options.delete_crash_dumps, Message::CustomCleanToggleCrashDumps, active_colors, lang_font),
            ].spacing(10)
        );

        let explorer_caches_section = make_section(
            &self.translations.custom_clean.windows_explorer_caches,
            column![
                make_checkbox(&self.translations.custom_clean.delete_web_cache, self.custom_clean_options.delete_web_cache, Message::CustomCleanToggleWebCache, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_inet_cache, self.custom_clean_options.delete_inet_cache, Message::CustomCleanToggleInetCache, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_windows_caches, self.custom_clean_options.delete_windows_caches, Message::CustomCleanToggleWindowsCaches, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_windows_explorer, self.custom_clean_options.delete_windows_explorer, Message::CustomCleanToggleWindowsExplorer, active_colors, lang_font),
            ].spacing(10)
        );

        let recent_files_section = make_section(
            &self.translations.custom_clean.recent_files,
            column![
                make_checkbox(&self.translations.custom_clean.delete_recent, self.custom_clean_options.delete_recent, Message::CustomCleanToggleRecent, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_automatic_destinations, self.custom_clean_options.delete_automatic_destinations, Message::CustomCleanToggleAutomaticDestinations, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_custom_destinations, self.custom_clean_options.delete_custom_destinations, Message::CustomCleanToggleCustomDestinations, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_tracing_dir, self.custom_clean_options.delete_tracing_dir, Message::CustomCleanToggleTracingDir, active_colors, lang_font),
            ].spacing(10)
        );

        let gpu_caches_section = make_section(
            &self.translations.custom_clean.gpu_caches,
            column![
                make_checkbox(&self.translations.custom_clean.delete_nvidia_cache, self.custom_clean_options.delete_nvidia_cache, Message::CustomCleanToggleNvidiaCache, active_colors, lang_font),
            ].spacing(10)
        );

        let deep_cleaning_section = make_section(
            &self.translations.custom_clean.deep_cleaning,
            column![
                make_checkbox(&self.translations.custom_clean.delete_windows_prefetch, self.custom_clean_options.delete_windows_prefetch, Message::CustomCleanToggleWindowsPrefetch, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_my_games, self.custom_clean_options.delete_my_games, Message::CustomCleanToggleMyGames, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_easyanticheat, self.custom_clean_options.delete_easyanticheat, Message::CustomCleanToggleEasyanticheat, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_battleye, self.custom_clean_options.delete_battleye, Message::CustomCleanToggleBattleye, active_colors, lang_font),
                make_checkbox(&self.translations.custom_clean.delete_faceit, self.custom_clean_options.delete_faceit, Message::CustomCleanToggleFaceit, active_colors, lang_font),
            ].spacing(10)
        );

        let left_column = column![
            processes_section,
            Space::with_height(Length::Fixed(15.0)),
            system_id_section,
            Space::with_height(Length::Fixed(15.0)),
            game_tracking_section,
            Space::with_height(Length::Fixed(15.0)),
            registry_caches_section,
            Space::with_height(Length::Fixed(15.0)),
            mac_volume_section,
        ].spacing(10).width(Length::FillPortion(1));

        let right_column = column![
            steam_login_section,
            Space::with_height(Length::Fixed(15.0)),
            steam_dirs_section,
            Space::with_height(Length::Fixed(15.0)),
            system_cache_section,
            Space::with_height(Length::Fixed(15.0)),
            explorer_caches_section,
            Space::with_height(Length::Fixed(15.0)),
            recent_files_section,
            Space::with_height(Length::Fixed(15.0)),
            gpu_caches_section,
            Space::with_height(Length::Fixed(15.0)),
            deep_cleaning_section,
        ].spacing(10).width(Length::FillPortion(1));

        let (button_text_str, on_press_message) = match self.state {
            State::Idle => (&self.translations.custom_clean.execute_custom_clean, Some(Message::ExecuteCustomClean)),
            State::Cleaning => (&self.translations.custom_clean.cleaning_in_progress, None),
        };

        let mut execute_button = button(text(button_text_str).size(16).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT)))
            .padding(15)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(style::SuccessButtonStyle { custom_colors: active_colors })));

        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let back_button = button(
            text(&self.translations.custom_clean.back_to_main).size(14).horizontal_alignment(iced::alignment::Horizontal::Center).font(lang_font.unwrap_or(iced::Font::DEFAULT))
        )
            .padding(10)
            .width(Length::Fixed(180.0))
            .on_press(Message::CloseCustomClean)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let main_content = column![
            header,
            Space::with_height(Length::Fixed(15.0)),
            container(
                scrollable(
                    column![
                        row![left_column, right_column].spacing(20).width(Length::Fill),
                        Space::with_height(Length::Fixed(20.0)),
                        execute_button,
                        Space::with_height(Length::Fixed(15.0)),
                        container(back_button).center_x().width(Length::Fill),
                    ]
                    .width(Length::Fill)
                )
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle { custom_colors: active_colors })))
            .into()
    }
}
