use iced::widget::{button, checkbox, scrollable, text, Column, Space, container};
use iced::{Element, Length};
use crate::core::redist::{RedistCategory, RedistItem, format_size};
use crate::ui::style;

#[derive(Debug, Clone)]
pub struct RedistViewState {
    pub category_common: bool,
    pub category_directx: bool,
    pub category_dotnet: bool,
    pub category_vcredist: bool,
    pub category_installers: bool,

    pub is_scanning: bool,
    pub scan_results: Option<Vec<RedistItem>>,
    pub last_clean_log: Option<Vec<String>>,
}

impl Default for RedistViewState {
    fn default() -> Self {
        Self {
            category_common: true,
            category_directx: true,
            category_dotnet: true,
            category_vcredist: true,
            category_installers: false,
            is_scanning: false,
            scan_results: None,
            last_clean_log: None,
        }
    }
}

impl RedistViewState {
    pub fn get_active_categories(&self) -> Vec<RedistCategory> {
        let mut cats = Vec::new();
        if self.category_common { cats.push(RedistCategory::CommonRedist); }
        if self.category_directx { cats.push(RedistCategory::DirectX); }
        if self.category_dotnet { cats.push(RedistCategory::DotNet); }
        if self.category_vcredist { cats.push(RedistCategory::VCRedist); }
        if self.category_installers { cats.push(RedistCategory::Installers); }
        cats
    }
}

#[derive(Debug, Clone)]
pub enum RedistMessage {
    ToggleCommon(bool),
    ToggleDirectX(bool),
    ToggleDotNet(bool),
    ToggleVCRedist(bool),
    ToggleInstallers(bool),
    StartScan,
    ScanFinished(Vec<RedistItem>),
    CleanFoundItems,
    CleanFinished(Vec<String>),
    Close,
}

pub fn view<'a>(state: &'a RedistViewState) -> Element<'a, RedistMessage> {
    // Toggles
    let toggles = Column::new()
        .push(checkbox("Common Redistributables (_CommonRedist)", state.category_common).on_toggle(RedistMessage::ToggleCommon))
        .push(checkbox("DirectX Installers", state.category_directx).on_toggle(RedistMessage::ToggleDirectX))
        .push(checkbox(".NET Framework", state.category_dotnet).on_toggle(RedistMessage::ToggleDotNet))
        .push(checkbox("Visual C++ Redistributables", state.category_vcredist).on_toggle(RedistMessage::ToggleVCRedist))
        .push(checkbox("Other Installers/Support (Aggressive)", state.category_installers).on_toggle(RedistMessage::ToggleInstallers))
        .spacing(10);

    let scan_btn = button(text("Scan Steam Libraries").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(RedistMessage::StartScan)
        .padding(10)
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

    let mut content = Column::new()
        .push(text("Steam Redistributable Cleaner").size(24).style(style::TITLE_COLOR))
        .push(Space::with_height(Length::Fixed(10.0)))
        .push(text("Select categories to remove from game folders:").size(16))
        .push(Space::with_height(Length::Fixed(10.0)))
        .push(toggles)
        .push(Space::with_height(Length::Fixed(20.0)))
        .push(scan_btn);

    if state.is_scanning {
        content = content.push(Space::with_height(Length::Fixed(20.0))).push(text("Scanning... please wait.").size(18));
    } else if let Some(results) = &state.scan_results {
        let total_size: u64 = results.iter().map(|i| i.size).sum();
        let count = results.len();

        let summary = text(format!("Found {} folders. Total size: {}", count, format_size(total_size)))
            .size(16)
            .style(style::TITLE_COLOR);

        let clean_btn = button(text("Clean Selected Items").horizontal_alignment(iced::alignment::Horizontal::Center))
            .on_press(RedistMessage::CleanFoundItems)
            .padding(10)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(style::DangerButtonStyle)));

        content = content
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(summary)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(clean_btn);

        // List items (limited view)
        let list_col = results.iter().take(50).fold(Column::new().spacing(2), |col, item| {
            col.push(text(format!("{} - {}", item.path.display(), format_size(item.size))).size(12))
        });

        let scroll = scrollable(list_col).height(Length::Fixed(200.0));

        content = content.push(Space::with_height(Length::Fixed(10.0))).push(scroll);
        if count > 50 {
             content = content.push(text(format!("...and {} more.", count - 50)).size(12));
        }
    }

    if let Some(logs) = &state.last_clean_log {
        content = content.push(Space::with_height(Length::Fixed(20.0))).push(text("Clean Results:").size(16));
        let log_col = logs.iter().fold(Column::new().spacing(2), |col, msg| col.push(text(msg).size(12)));
        let scroll = scrollable(log_col).height(Length::Fixed(150.0));

        let log_container = container(scroll)
            .style(iced::theme::Container::Custom(Box::new(style::ConsoleContainerStyle)))
            .padding(10)
            .width(Length::Fill);

        content = content.push(log_container);
    }

    let back_btn = button(text("<- Back to Main").horizontal_alignment(iced::alignment::Horizontal::Center))
        .on_press(RedistMessage::Close)
        .padding(12)
        .width(Length::Fixed(200.0))
        .style(iced::theme::Button::Custom(Box::new(style::PrimaryButtonStyle)));

    content = content
        .push(Space::with_height(Length::Fixed(20.0)))
        .push(back_btn);

    container(scrollable(content))
        .padding(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(style::MainWindowStyle)))
        .into()
}
