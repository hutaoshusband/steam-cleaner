// src/ui/app.rs

use iced::widget::{button, column, container, scrollable, text, toggler, Column, Row, Space};
use iced::{Element, Length, Sandbox, Theme};
use tinyfiledialogs as tfd;

use crate::core::backup;
use crate::core::executor::{run_all_selected, CleaningOptions};
use crate::core::inspector::{gather_system_info, SystemInfo};
use crate::ui::style;

pub struct CleanerApp {
    state: State,
    options: CleaningOptions,
    log_messages: Vec<String>,
    inspector_open: bool,
    inspector_state: InspectorState,
}

#[derive(Default)]
struct InspectorState {
    is_loading: bool,
    info: SystemInfo,
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
    ToggleAggressive(bool),
    ToggleDryRun(bool),
    Execute,
    Backup,
    OpenInspector,
    CloseInspector,
}

impl Sandbox for CleanerApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::Idle,
            options: CleaningOptions::default(),
            log_messages: vec!["Select the desired operations.".to_string()],
            inspector_open: false,
            inspector_state: InspectorState::default(),
        }
    }

    fn title(&self) -> String {
        "Modern Cleaner".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleSystemIds(value) => self.options.spoof_system_ids = value,
            Message::ToggleMac(value) => self.options.spoof_mac = value,
            Message::ToggleVolumeId(value) => self.options.spoof_volume_id = value,
            Message::ToggleSteam(value) => self.options.clean_steam = value,
            Message::ToggleAggressive(value) => self.options.clean_aggressive = value,
            Message::ToggleDryRun(value) => self.options.dry_run = value,
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
                            return;
                        }
                    }

                    self.state = State::Cleaning;
                    self.log_messages = vec!["[*] Starting cleaning...".to_string()];

                    let results = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(run_all_selected(self.options));
                    self.log_messages = results;
                    self.state = State::Idle;
                }
            }
            Message::Backup => {
                match backup::create_backup() {
                    Ok(message) => self.log_messages.push(message),
                    Err(e) => self.log_messages.push(format!("Backup failed: {}", e)),
                }
            }
            Message::OpenInspector => {
                self.inspector_open = true;
                self.inspector_state.is_loading = true;
                let info = tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(gather_system_info());
                self.inspector_state.info = info;
                self.inspector_state.is_loading = false;
            }
            Message::CloseInspector => self.inspector_open = false,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if self.inspector_open {
            self.view_inspector_window()
        } else {
            self.view_main_window()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl CleanerApp {
    fn view_main_window(&self) -> Element<'_, Message> {
        fn make_toggler<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message) -> Element<'a, Message> {
            toggler(Some(label.to_string()), value, msg)
                .style(iced::theme::Toggler::Custom(Box::new(style::CustomTogglerStyle)))
                .width(280)
                .text_size(16)
                .into()
        }

        let inspector_button = button(text("🔍 Inspector"))
            .on_press(Message::OpenInspector)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

        let title_bar = Row::new()
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .push(Space::with_width(Length::Fill))
            .push(text("Modern System Cleaner").size(32).style(style::TITLE_COLOR))
            .push(Space::with_width(Length::Fill))
            .push(inspector_button)
            .push(Space::with_width(Length::Fixed(20.0)));
        
        let system_spoofing_options = column![
            text("System-Spoofing").size(20),
            make_toggler("Spoof System IDs", self.options.spoof_system_ids, Message::ToggleSystemIds),
            make_toggler("Spoof MAC Address", self.options.spoof_mac, Message::ToggleMac),
            make_toggler("Spoof Volume ID", self.options.spoof_volume_id, Message::ToggleVolumeId),
        ]
        .spacing(15)
        .padding(20);

        let steam_cleaning_options = column![
            text("Steam-Reinigung").size(20),
            make_toggler("Clean Steam", self.options.clean_steam, Message::ToggleSteam),
        ]
        .spacing(15)
        .padding(20);

        let aggressive_cleaning_options = column![
            text("Aggressive Reinigung").size(20),
            make_toggler("Aggressive Clean", self.options.clean_aggressive, Message::ToggleAggressive),
        ]
        .spacing(15)
        .padding(20);

        let options_box = container(
            column![]
                .push(system_spoofing_options)
                .push(steam_cleaning_options)
                .push(aggressive_cleaning_options)
        )
        .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle)));
        
        let (button_text, on_press_message) = match self.state {
            State::Idle => ("Execute", Some(Message::Execute)),
            State::Cleaning => ("Running...", None),
        };

        let mut execute_button = button(text(button_text).size(20))
            .padding(15)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));
        
        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let backup_button = button(text("Backup Steam Data").size(20))
            .padding(15)
            .on_press(Message::Backup)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

        let log_output = self.log_messages.iter().fold(Column::new().spacing(5), |col, msg| {
            col.push(text(msg.clone()))
        });
        
        let status_box = container(scrollable(log_output))
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle)))
            .padding(10)
            .width(450)
            .height(150);

        let disclaimer_text = "WARNING: This tool can lead to data loss (game saves, etc.) and will remove accounts from this PC. Use at your own risk. A restart is recommended after cleaning.";
        let disclaimer = container(text(disclaimer_text).size(14))
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle)));

        let content = Column::new()
            .push(title_bar)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(disclaimer)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(options_box)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(make_toggler("Simulation Mode (Dry Run)", self.options.dry_run, Message::ToggleDryRun))
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(Row::new().spacing(20).push(execute_button).push(backup_button))
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(status_box)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle)))
            .into()
    }

    fn view_inspector_window(&self) -> Element<'_, Message> {
        let content = if self.inspector_state.is_loading {
            Column::new().push(text("Loading system information..."))
        } else {
            let info = &self.inspector_state.info;
            let info_col = column![
                text(format!("Machine GUID: {}", info.machine_guid)),
                text(format!("Product ID: {}", info.product_id)),
                text(format!("Computer Name: {}", info.computer_name)),
                text(format!("Volume ID (C:): {}", info.volume_id)),
                Space::with_height(Length::Fixed(10.0)),
                text("Found MAC Addresses:").size(18),
            ]
            .spacing(10);
            
            let adapters_col = info.network_adapters.iter().fold(Column::new().spacing(5), |col, (_desc, mac)| {
                col.push(text(format!("  - {}", mac)))
            });

            let steam_files_col = info.steam_login_files.iter().fold(Column::new().spacing(5), |col, file| {
                col.push(text(format!("  - {}", file)))
            });

            Column::new()
                .push(info_col)
                .push(scrollable(adapters_col))
                .push(Space::with_height(Length::Fixed(10.0)))
                .push(text("Found Steam Login Files:").size(18))
                .push(scrollable(steam_files_col))
        };

        let final_layout = Column::new()
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center)
            .push(text("System Inspector").size(24).style(style::TITLE_COLOR))
            .push(container(content).padding(15).style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle))))
            .push(button("Back").on_press(Message::CloseInspector));

        container(final_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle)))
            .into()
    }
}
