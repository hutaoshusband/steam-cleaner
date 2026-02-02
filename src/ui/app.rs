use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, toggler, Column, Row, Space};
use iced::{border, Application, Color, Command, Element, Length, Theme};
use tinyfiledialogs as tfd;

use crate::core::backup;
use crate::core::executor::{apply_hardware_profile, run_all_selected, CleaningOptions};
use crate::core::hardware_profile::{HardwareProfile, ProfileManager};
use crate::core::inspector::{gather_system_info, SystemInfo};
use crate::ui::style;
use crate::ui::redist_view;
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

// CustomThemeColors moved to style.rs

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
}

impl Application for CleanerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let profile_manager = ProfileManager::load().unwrap_or_default();
        let profile_names = profile_manager.profile_names();

        (
            Self {
                state: State::Idle,
                options: CleaningOptions::default(),
                log_messages: vec!["[*] Ready. Select options and click Execute.".to_string()],
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
                custom_colors: style::CustomThemeColors::load(),
                custom_theme_active: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Steam Cleaner 0.1.8".to_string()
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
                            "Aggressive Cleaning Warning",
                            "Aggressive cleaning can have unintended side effects. Are you sure you want to continue?",
                            tfd::MessageBoxIcon::Warning,
                            tfd::YesNo::No,
                        );
                        if confirmation == tfd::YesNo::No {
                            self.log_messages.push("Aggressive cleaning cancelled.".to_string());
                            return Command::none();
                        }
                    }

                    self.state = State::Cleaning;
                    self.log_messages = vec!["[*] Starting cleaning...".to_string()];
                    Command::perform(run_all_selected(self.options), Message::CleaningFinished)
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
                    self.profile_state.status_message = Some("[!] Please enter a profile name.".to_string());
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
                                    Err(e) => Err(format!("Failed to save: {}", e)),
                                }
                            }
                            Err(e) => Err(format!("Failed to snapshot: {}", e)),
                        }
                    },
                    Message::ProfileSaved,
                )
            }
            Message::ProfileSaved(result) => {
                match result {
                    Ok(name) => {
                        self.profile_state.status_message = Some(format!("[+] Profile '{}' saved!", name));
                        self.profile_state.new_profile_name.clear();
                        self.profile_manager = ProfileManager::load().unwrap_or_default();
                        self.profile_state.profile_names = self.profile_manager.profile_names();
                        self.profile_state.selected_profile = Some(name);
                    }
                    Err(e) => {
                        self.profile_state.status_message = Some(format!("[-] {}", e));
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

                        return Command::perform(
                            apply_hardware_profile(profile_clone, dry_run),
                            Message::ProfileApplied,
                        );
                    } else {
                        self.profile_state.status_message = Some("[!] Profile not found.".to_string());
                    }
                } else {
                    self.profile_state.status_message = Some("[!] Please select a profile first.".to_string());
                }
                Command::none()
            }
            Message::ProfileApplied(results) => {
                self.profile_state.is_applying = false;
                self.log_messages = results;
                self.profile_state.status_message = Some("[+] Profile applied! Check log for details.".to_string());
                Command::none()
            }
            Message::DeleteSelectedProfile => {
                if let Some(name) = &self.profile_state.selected_profile.clone() {
                    let confirmation = tfd::message_box_yes_no(
                        "Delete Profile",
                        &format!("Are you sure you want to delete profile '{}'?", name),
                        tfd::MessageBoxIcon::Question,
                        tfd::YesNo::No,
                    );
                    if confirmation == tfd::YesNo::Yes {
                        self.profile_manager.remove_profile(name);
                        if let Err(e) = self.profile_manager.save() {
                            self.profile_state.status_message = Some(format!("[-] Failed to save: {}", e));
                        } else {
                            self.profile_state.status_message = Some(format!("[+] Profile '{}' deleted.", name));
                            self.profile_state.profile_names = self.profile_manager.profile_names();
                            self.profile_state.selected_profile = None;
                        }
                    }
                } else {
                    self.profile_state.status_message = Some("[!] Please select a profile first.".to_string());
                }
                Command::none()
            }
            Message::RefreshProfiles => {
                self.profile_manager = ProfileManager::load().unwrap_or_default();
                self.profile_state.profile_names = self.profile_manager.profile_names();
                self.profile_state.status_message = Some("[*] Profiles refreshed.".to_string());
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
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Background Color", tfd::DefaultColorValue::Hex("#1a1b26")) {
                    self.custom_colors.background = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickSurfaceColor => {
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Surface Color", tfd::DefaultColorValue::Hex("#24283b")) {
                    self.custom_colors.surface = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickTextColor => {
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Text Color", tfd::DefaultColorValue::Hex("#c0caf5")) {
                    self.custom_colors.text = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickPrimaryColor => {
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Primary Color", tfd::DefaultColorValue::Hex("#7aa2f7")) {
                    self.custom_colors.primary = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickDangerColor => {
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Danger Color", tfd::DefaultColorValue::Hex("#f7768e")) {
                    self.custom_colors.danger = Color::from_rgb8(rgb.1[0], rgb.1[1], rgb.1[2]);
                }
                Command::none()
            }
            Message::PickSuccessColor => {
                if let Some(rgb) = tfd::color_chooser_dialog("Pick Success Color", tfd::DefaultColorValue::Hex("#9ece6a")) {
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
        }
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
            redist_view::view(&self.redist_state, active_colors).map(Message::Redist)
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

        fn make_toggler<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message, colors: Option<style::CustomThemeColors>) -> Element<'a, Message> {
            toggler(Some(label.to_string()), value, msg)
                .style(iced::theme::Toggler::Custom(Box::new(style::CustomTogglerStyle { custom_colors: colors })))
                .width(Length::Fill)
                .text_size(15)
                .into()
        }

        let system_spoofing_options = column![
            text("System-Spoofing").size(16).style(style::title_color(&self.current_theme)),
            make_toggler("Spoof System IDs", self.options.spoof_system_ids, Message::ToggleSystemIds, active_colors),
            make_toggler("Spoof MAC Address", self.options.spoof_mac, Message::ToggleMac, active_colors),
            make_toggler("Spoof Volume ID", self.options.spoof_volume_id, Message::ToggleVolumeId, active_colors),
        ]
        .spacing(10)
        .padding(12);

        let steam_cleaning_options = column![
            text("Steam-Reinigung").size(16).style(style::title_color(&self.current_theme)),
            make_toggler("Clean Steam", self.options.clean_steam, Message::ToggleSteam, active_colors),
        ]
        .spacing(10)
        .padding(12);

        let aggressive_cleaning_options = column![
            text("Aggressive Reinigung").size(16).style(style::title_color(&self.current_theme)),
            make_toggler("Aggressive Clean", self.options.clean_aggressive, Message::ToggleAggressive, active_colors),
        ]
        .spacing(10)
        .padding(12);

        let options_content = column![
            system_spoofing_options,
            steam_cleaning_options,
            aggressive_cleaning_options
        ]
        .spacing(8);

        let options_box = container(options_content)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors })))
            .padding(10)
            .width(Length::Fill);

        let inspector_button = button(text("Inspector & Profiles").size(15).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fill)
            .on_press(Message::OpenInspector)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let redist_button = button(text("Steam Redist Cleaner (Beta)").size(15).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fill)
            .on_press(Message::OpenRedist)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let themes_button = button(text("Themes & Appearance").size(15).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fill)
            .on_press(Message::OpenThemes)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let (button_text_str, on_press_message) = match self.state {
            State::Idle => ("Execute Cleaning", Some(Message::Execute)),
            State::Cleaning => ("Cleaning in Progress...", None),
        };

        let mut execute_button = button(text(button_text_str).size(15).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(style::SuccessButtonStyle { custom_colors: active_colors })));

        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let backup_button = button(text("Backup Steam Data").size(15).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fill)
            .on_press(Message::Backup)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let dry_run_toggle = make_toggler("Simulation Mode (Dry Run)", self.options.dry_run, Message::ToggleDryRun, active_colors);

        let left_panel_content = column![
            options_box,
            Space::with_height(Length::Fixed(15.0)),
            container(dry_run_toggle).padding(10).style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle { custom_colors: active_colors }))),
            Space::with_height(Length::Fixed(15.0)),
            execute_button,
            Space::with_height(Length::Fixed(8.0)),
            backup_button,
            Space::with_height(Length::Fixed(8.0)),
            inspector_button,
            Space::with_height(Length::Fixed(8.0)),
            redist_button,
            Space::with_height(Length::Fixed(8.0)),
            themes_button,
            Space::with_height(Length::Fixed(20.0)),
        ]
        .spacing(8)
        .width(Length::Fill);

        let left_panel = container(scrollable(left_panel_content))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .padding(15);

        let log_output = self.log_messages.iter().fold(Column::new().spacing(4), |col, msg| {
            col.push(text(msg.clone()).font(iced::Font::MONOSPACE).size(13))
        });

        let console_box = container(scrollable(log_output))
            .style(iced::theme::Container::Custom(Box::new(style::ConsoleContainerStyle { custom_colors: active_colors })))
            .padding(15)
            .width(Length::Fill)
            .height(Length::Fill);

        let right_panel = Column::new()
            .push(text("Verbose Log Output").size(16).style(style::title_color(&self.current_theme)))
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(console_box)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(15);

        let main_content = Row::new()
            .push(left_panel)
            .push(right_panel)
            .height(Length::Fill);

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle { custom_colors: active_colors })))
            .into()
    }

    fn view_inspector_window(&self) -> Element<'_, Message> {
        let active_colors = if self.custom_theme_active { Some(self.custom_colors) } else { None };
        
        let header = container(
            text("System Inspector & Profile Manager").size(24).style(style::title_color(&self.current_theme))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        let system_info_section = if self.inspector_state.is_loading {
            Column::new().push(text("Loading system information..."))
        } else {
            let info = &self.inspector_state.info;
            let info_col = column![
                text("Current System Hardware IDs").size(18).style(style::title_color(&self.current_theme)),
                Space::with_height(Length::Fixed(10.0)),
                text(format!("Machine GUID: {}", info.machine_guid)),
                text(format!("Product ID: {}", info.product_id)),
                text(format!("Computer Name: {}", info.computer_name)),
                text(format!("Volume ID (C:): {}", info.volume_id)),
                Space::with_height(Length::Fixed(10.0)),
                text("MAC Addresses:").size(16),
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

        let profile_header = text("Hardware-ID Profile Manager").size(18).style(style::title_color(&self.current_theme));

        let profile_dropdown: Element<'_, Message> = pick_list(
            self.profile_state.profile_names.clone(),
            self.profile_state.selected_profile.clone(),
            Message::ProfileSelected,
        )
        .placeholder("Select a profile...")
        .width(Length::Fill)
        .into();

        let dropdown_row = Row::new()
            .push(text("Load Profile: ").size(14))
            .push(profile_dropdown)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        let apply_button = button(
            text("Apply").size(14).horizontal_alignment(iced::alignment::Horizontal::Center)
        )
            .padding(8)
            .width(Length::FillPortion(1))
            .on_press(Message::ApplySelectedProfile)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle { custom_colors: active_colors })));

        let delete_button = button(
            text("Delete").size(14).horizontal_alignment(iced::alignment::Horizontal::Center)
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
            "Enter profile name...",
            &self.profile_state.new_profile_name,
        )
        .on_input(Message::NewProfileNameChanged)
        .padding(10)
        .width(Length::Fill);

        let save_button = button(
            text("Save Current as Profile").size(14).horizontal_alignment(iced::alignment::Horizontal::Center)
        )
            .padding(10)
            .width(Length::Fill)
            .on_press(Message::SaveCurrentAsProfile)
            .style(iced::theme::Button::Custom(Box::new(style::SuccessButtonStyle { custom_colors: active_colors })));

        let status_text: Element<'_, Message> = if let Some(msg) = &self.profile_state.status_message {
            text(msg).size(13).into()
        } else {
            Space::with_height(Length::Fixed(13.0)).into()
        };

        let profile_details: Element<'_, Message> = if let Some(name) = &self.profile_state.selected_profile {
            if let Some(profile) = self.profile_manager.get_profile(name) {
                let mac_count = profile.mac_addresses.len();
                let vol_count = profile.volume_ids.len();
                column![
                    text(format!("Profile: {}", profile.name)).size(14),
                    text(format!("  Created: {}", profile.created_at)).size(12),
                    text(format!("  {} MAC address(es), {} Volume ID(s)", mac_count, vol_count)).size(12),
                ]
                .spacing(3)
                .into()
            } else {
                Space::with_height(Length::Fixed(1.0)).into()
            }
        } else {
            text("Select a profile to see details, or save current hardware IDs as a new profile.")
                .size(13)
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
            text("Create New Profile from Current Hardware:").size(14),
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
            text("<- Back to Main").size(14).horizontal_alignment(iced::alignment::Horizontal::Center)
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

        let header = container(
            text("Appearance Settings").size(24).style(style::title_color(&self.current_theme))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        fn theme_btn<'a>(label: &'static str, theme: Theme, current: &Theme, colors: Option<style::CustomThemeColors>) -> Element<'a, Message> {
            let is_selected = theme == *current;

            let btn_style = if is_selected {
                style::ThemedButtonStyle::Success(colors)
            } else {
                style::ThemedButtonStyle::Primary(colors)
            };

            button(text(label).size(16).horizontal_alignment(iced::alignment::Horizontal::Center))
                .padding(15)
                .width(Length::Fill)
                .on_press(Message::ThemeSelected(theme))
                .style(iced::theme::Button::Custom(Box::new(btn_style)))
                .into()
        }

        let theme_buttons = column![
            text("Select Application Theme:").size(18).style(style::title_color(&self.current_theme)),
            Space::with_height(Length::Fixed(15.0)),
            theme_btn("Red Retro (Default)", Theme::Dark, &self.current_theme, active_colors),
            theme_btn("White Mode (Light)", Theme::Light, &self.current_theme, active_colors),
            theme_btn("Pure Dark (Neutral)", Theme::Dracula, &self.current_theme, active_colors),
            theme_btn("Ultra Dark", Theme::Nord, &self.current_theme, active_colors),
            theme_btn("Cream", Theme::SolarizedLight, &self.current_theme, active_colors),
            Space::with_height(Length::Fixed(20.0)),
            button(text("Custom Colors...").size(16).horizontal_alignment(iced::alignment::Horizontal::Center))
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
            text("<- Back to Main").size(14).horizontal_alignment(iced::alignment::Horizontal::Center)
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
        let header = container(
            text("Theme Customizer").size(24).style(style::title_color(&self.current_theme))
        )
        .padding(20)
        .width(Length::Fill)
        .align_y(iced::alignment::Vertical::Center);

        let color_btn = |label: &str, color: Color, msg: Message| -> Element<Message> {
            column![
                text(label).size(14).style(style::title_color(&self.current_theme)),
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

        let core_colors = container(
            column![
                text("Core Layout").size(18),
                color_btn("Background", self.custom_colors.background, Message::PickBackgroundColor),
                color_btn("Surface / Panels", self.custom_colors.surface, Message::PickSurfaceColor),
                color_btn("Text Color", self.custom_colors.text, Message::PickTextColor),
            ]
            .spacing(15)
            .width(Length::Fill)
        )
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle::default())))
        .width(Length::FillPortion(1));

        let accent_colors = container(
            column![
                text("Accents & Status").size(18),
                color_btn("Primary Accent", self.custom_colors.primary, Message::PickPrimaryColor),
                color_btn("Danger / Error", self.custom_colors.danger, Message::PickDangerColor),
                color_btn("Success / Go", self.custom_colors.success, Message::PickSuccessColor),
            ]
            .spacing(15)
            .width(Length::Fill)
        )
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle::default())))
        .width(Length::FillPortion(1));

        let color_columns = row![core_colors, accent_colors].spacing(20);

        // --- LIVE PREVIEW BOX ---
        let preview_header = container(text("Live Preview").size(16).style(style::title_color(&self.current_theme)))
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(style::PreviewBoxStyle {
                bg: self.custom_colors.surface,
                text: self.custom_colors.text
            })))
            .width(Length::Fill);

        let preview_content = column![
            text("Sample Window Content").size(18).style(self.custom_colors.text),
            text("This shows how your color choices look together.").size(14).style(self.custom_colors.text),
            row![
                button(text("Primary Action").horizontal_alignment(iced::alignment::Horizontal::Center))
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::Button::Custom(Box::new(style::ColorPreviewStyle { color: self.custom_colors.primary }))),
                button(text("Danger Zone").horizontal_alignment(iced::alignment::Horizontal::Center))
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::theme::Button::Custom(Box::new(style::ColorPreviewStyle { color: self.custom_colors.danger }))),
            ].spacing(10),
            container(text("Success Message Received").style(Color::WHITE))
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
            text("Preview").size(18),
            preview_container,
            Space::with_height(Length::Fixed(20.0)),
            button(text("Apply Changes").size(16).horizontal_alignment(iced::alignment::Horizontal::Center))
                    .padding(15)
                    .width(Length::Fill)
                    .on_press(Message::ApplyCustomTheme)
                    .style(iced::theme::Button::Custom(Box::new(style::ThemedButtonStyle::Success(None))))
        ]
        .spacing(10)
        .width(Length::Fill);

        let back_button = button(text("<- Back to Themes").size(14).horizontal_alignment(iced::alignment::Horizontal::Center))
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
}
