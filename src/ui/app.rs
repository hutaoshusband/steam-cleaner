// src/ui/app.rs

use iced::widget::{button, column, container, scrollable, text, toggler, Column, Row, Space};
use iced::{executor, window, Command, Element, Length, Sandbox, Settings, Theme};

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
    ToggleRegistry(bool),
    ToggleMac(bool),
    ToggleVolumeId(bool),
    ToggleCache(bool),
    ToggleHkcu(bool),
    Execute,
    CleaningFinished(Vec<String>),
    OpenInspector,
    CloseInspector,
    SystemInfoLoaded(SystemInfo),
}

impl Sandbox for CleanerApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::Idle,
            options: CleaningOptions::default(),
            log_messages: vec!["Wähle die gewünschten Operationen aus.".to_string()],
            inspector_open: false,
            inspector_state: InspectorState::default(),
        }
    }

    fn title(&self) -> String {
        "Modern Cleaner".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleRegistry(value) => self.options.clean_registry = value,
            Message::ToggleMac(value) => self.options.spoof_mac = value,
            Message::ToggleVolumeId(value) => self.options.change_volume_id = value,
            Message::ToggleCache(value) => self.options.clean_cache = value,
            Message::ToggleHkcu(value) => self.options.spoof_hkcu = value,
            Message::Execute => {
                if self.state == State::Idle {
                    self.state = State::Cleaning;
                    self.log_messages = vec!["[*] Starte Bereinigung...".to_string()];

                    let results = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(run_all_selected(self.options));
                    self.log_messages = results;
                    self.state = State::Idle;
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
            Message::CleaningFinished(_) => {}
            Message::SystemInfoLoaded(_) => {}
        }
    }

    fn view(&self) -> Element<Message> {
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

impl Default for CleaningOptions {
    fn default() -> Self {
        Self {
            clean_registry: false,
            spoof_mac: false,
            change_volume_id: false,
            clean_cache: false,
            spoof_hkcu: false,
        }
    }
}

impl CleanerApp {
    fn view_main_window(&self) -> Element<Message> {
        fn make_toggler(label: &str, value: bool, msg: fn(bool) -> Message) -> Element<Message> {
            toggler(Some(label.to_string()), value, msg)
                .style(iced::theme::Toggler::Custom(Box::new(style::CustomTogglerStyle)))
                .width(280)
                .text_size(16)
                .into()
        }

        let inspector_button = button(text("🔍 Inspektor"))
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

        let options = column![
            make_toggler("Registry bereinigen", self.options.clean_registry, Message::ToggleRegistry),
            make_toggler("MAC-Adresse ändern", self.options.spoof_mac, Message::ToggleMac),
            make_toggler("Volume ID ändern", self.options.change_volume_id, Message::ToggleVolumeId),
            make_toggler("Cache-Dateien löschen", self.options.clean_cache, Message::ToggleCache),
            make_toggler("HKCU-Schlüssel bereinigen", self.options.spoof_hkcu, Message::ToggleHkcu),
        ]
        .spacing(15)
        .padding(20);

        let options_box = container(options)
            .style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle)));

        let (button_text, on_press_message) = match self.state {
            State::Idle => ("Ausführen", Some(Message::Execute)),
            State::Cleaning => ("Wird ausgeführt...", None),
        };

        let mut execute_button = button(text(button_text).size(20))
            .padding(15)
            .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));
        
        if let Some(msg) = on_press_message {
            execute_button = execute_button.on_press(msg);
        }

        let log_output = self.log_messages.iter().fold(Column::new().spacing(5), |col, msg| {
            col.push(text(msg))
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
            .push(execute_button)
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

    fn view_inspector_window(&self) -> Element<Message> {
        let content = if self.inspector_state.is_loading {
            Column::new().push(text("Lade Systeminformationen..."))
        } else {
            let info = &self.inspector_state.info;
            let info_col = column![
                text(format!("Machine GUID: {}", info.machine_guid)),
                text(format!("Product ID: {}", info.product_id)),
                text(format!("Computer Name: {}", info.computer_name)),
                text(format!("Volume ID (C:): {}", info.volume_id)),
                Space::with_height(Length::Fixed(10.0)),
                text("Gefundene MAC-Adressen:").size(18),
            ]
            .spacing(10);
            
            let adapters_col = info.network_adapters.iter().fold(Column::new().spacing(5), |col, (_desc, mac)| {
                col.push(text(format!("  - {}", mac)))
            });

            Column::new().push(info_col).push(scrollable(adapters_col))
        };

        let final_layout = Column::new()
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center)
            .push(text("System-Inspektor").size(24).style(style::TITLE_COLOR))
            .push(container(content).padding(15).style(iced::theme::Container::Custom(Box::new(style::OptionsBoxStyle))))
            .push(button("Zurück").on_press(Message::CloseInspector));

        container(final_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle)))
            .into()
    }
}
