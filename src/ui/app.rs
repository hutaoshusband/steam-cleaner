use iced::widget::{button, column, container, scrollable, text, toggler, Column, Row, Space};
use iced::{window, Application, Command, Element, Length, Theme};
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
    CleaningFinished(Vec<String>),
    Backup,
    OpenInspector,
    InspectorLoaded(SystemInfo),
    CloseInspector,
    CloseWindow,
    WindowDragged,
}

impl Application for CleanerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                state: State::Idle,
                options: CleaningOptions::default(),
                log_messages: vec!["[*] Ready. Select options and click Execute.".to_string()],
                inspector_open: false,
                inspector_state: InspectorState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Modern Cleaner".to_string()
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
            Message::CloseWindow => {
                std::process::exit(0);
            }
            Message::WindowDragged => {
                window::drag(window::Id::MAIN)
            }
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
        Theme::Light
    }
}

impl CleanerApp {
    fn view_main_window(&self) -> Element<'_, Message> {
        fn make_toggler<'a>(label: &'a str, value: bool, msg: fn(bool) -> Message) -> Element<'a, Message> {
            toggler(Some(label.to_string()), value, msg)
                .style(iced::theme::Toggler::Custom(Box::new(style::CustomTogglerStyle)))
                .width(Length::Fill)
                .text_size(16)
                .into()
        }

        // --- Title Bar ---
        let inspector_button = button(text("Inspector").size(14))
            .on_press(Message::OpenInspector)
            .padding(8)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

        let close_button = button(text("X").size(18).horizontal_alignment(iced::alignment::Horizontal::Center))
            .on_press(Message::CloseWindow)
            .padding(8)
            .width(Length::Fixed(40.0))
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

        // Make the title text draggable by wrapping it in a transparent button
        let title_text = button(text("Modern System Cleaner").size(24).style(style::TITLE_COLOR))
            .on_press(Message::WindowDragged)
            .style(iced::theme::Button::Custom(Box::new(style::TransparentButtonStyle)));

        let title_bar = Row::new()
            .spacing(10)
            .align_items(iced::Alignment::Center)
            .push(Space::with_width(Length::Fixed(15.0)))
            .push(title_text)
            .push(Space::with_width(Length::Fill))
            .push(inspector_button)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(close_button)
            .push(Space::with_width(Length::Fixed(15.0)));

        // --- Left Panel: Options ---
        let system_spoofing_options = column![
            text("System-Spoofing").size(18).style(style::TITLE_COLOR),
            make_toggler("Spoof System IDs", self.options.spoof_system_ids, Message::ToggleSystemIds),
            make_toggler("Spoof MAC Address", self.options.spoof_mac, Message::ToggleMac),
            make_toggler("Spoof Volume ID", self.options.spoof_volume_id, Message::ToggleVolumeId),
        ]
        .spacing(12)
        .padding(15);

        let steam_cleaning_options = column![
            text("Steam-Reinigung").size(18).style(style::TITLE_COLOR),
            make_toggler("Clean Steam", self.options.clean_steam, Message::ToggleSteam),
        ]
        .spacing(12)
        .padding(15);

        let aggressive_cleaning_options = column![
            text("Aggressive Reinigung").size(18).style(style::TITLE_COLOR),
            make_toggler("Aggressive Clean", self.options.clean_aggressive, Message::ToggleAggressive),
        ]
        .spacing(12)
        .padding(15);

        let options_content = column![
            system_spoofing_options,
            steam_cleaning_options,
            aggressive_cleaning_options
        ]
        .spacing(10);

        let options_box = container(options_content)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle)))
            .padding(10)
            .width(Length::Fill);

        // Action Buttons
        let (button_text_str, on_press_message) = match self.state {
            State::Idle => ("Execute Cleaning", Some(Message::Execute)),
            State::Cleaning => ("Cleaning in Progress...", None),
        };

        let mut execute_button = button(text(button_text_str).size(18).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(15)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));
        
        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let backup_button = button(text("Backup Steam Data").size(18).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(15)
            .width(Length::Fill)
            .on_press(Message::Backup)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

        let dry_run_toggle = make_toggler("Simulation Mode (Dry Run)", self.options.dry_run, Message::ToggleDryRun);

        let left_panel = Column::new()
            .push(options_box)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(container(dry_run_toggle).padding(10).style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle))))
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(execute_button)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(backup_button)
            .spacing(10)
            .width(Length::FillPortion(1)) // 1/3 width
            .padding(20);

        // --- Right Panel: Console Output ---
        let log_output = self.log_messages.iter().fold(Column::new().spacing(5), |col, msg| {
            col.push(text(msg.clone()).font(iced::Font::MONOSPACE).size(14))
        });
        
        let console_box = container(scrollable(log_output))
            .style(iced::theme::Container::Custom(Box::new(style::ConsoleContainerStyle)))
            .padding(15)
            .width(Length::Fill)
            .height(Length::Fill);

        let right_panel = Column::new()
            .push(text("Verbose Log Output").size(18).style(style::TITLE_COLOR))
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(console_box)
            .width(Length::FillPortion(2)) // 2/3 width
            .padding(20);

        // --- Main Layout ---
        let main_content = Row::new()
            .push(left_panel)
            .push(right_panel)
            .height(Length::Fill);

        let content = Column::new()
            .push(title_bar)
            .push(main_content);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
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
