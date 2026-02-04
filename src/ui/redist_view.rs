use crate::core::redist::{format_size, RedistCategory, RedistItem};
use crate::i18n;
use crate::ui::style;
use iced::widget::{button, checkbox, container, scrollable, text, Column, Space};
use iced::{Element, Font, Length};

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
        if self.category_common {
            cats.push(RedistCategory::CommonRedist);
        }
        if self.category_directx {
            cats.push(RedistCategory::DirectX);
        }
        if self.category_dotnet {
            cats.push(RedistCategory::DotNet);
        }
        if self.category_vcredist {
            cats.push(RedistCategory::VCRedist);
        }
        if self.category_installers {
            cats.push(RedistCategory::Installers);
        }
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

pub fn view<'a>(
    state: &'a RedistViewState,
    custom_colors: Option<style::CustomThemeColors>,
    translations: &i18n::Translations,
    lang: i18n::Language,
) -> Element<'a, RedistMessage> {
    let lang_font = lang.font();

    let header = container(
        text(&translations.redist.title)
            .size(22)
            .style(style::TITLE_COLOR)
            .font(lang_font.unwrap_or(Font::DEFAULT)),
    )
    .padding(20)
    .width(Length::Fill)
    .center_x();

    let toggles = container(
        Column::new()
            .push(
                checkbox(
                    &translations.redist.common_redistributables,
                    state.category_common,
                )
                .on_toggle(RedistMessage::ToggleCommon)
                .style(iced::theme::Checkbox::Custom(Box::new(
                    style::CustomCheckboxStyle { custom_colors },
                )))
                .font(lang_font.unwrap_or(Font::DEFAULT)),
            )
            .push(
                checkbox(
                    &translations.redist.directx_installers,
                    state.category_directx,
                )
                .on_toggle(RedistMessage::ToggleDirectX)
                .style(iced::theme::Checkbox::Custom(Box::new(
                    style::CustomCheckboxStyle { custom_colors },
                )))
                .font(lang_font.unwrap_or(Font::DEFAULT)),
            )
            .push(
                checkbox(&translations.redist.dotnet_framework, state.category_dotnet)
                    .on_toggle(RedistMessage::ToggleDotNet)
                    .style(iced::theme::Checkbox::Custom(Box::new(
                        style::CustomCheckboxStyle { custom_colors },
                    )))
                    .font(lang_font.unwrap_or(Font::DEFAULT)),
            )
            .push(
                checkbox(
                    &translations.redist.visual_c_redistributables,
                    state.category_vcredist,
                )
                .on_toggle(RedistMessage::ToggleVCRedist)
                .style(iced::theme::Checkbox::Custom(Box::new(
                    style::CustomCheckboxStyle { custom_colors },
                )))
                .font(lang_font.unwrap_or(Font::DEFAULT)),
            )
            .push(
                checkbox(
                    &translations.redist.other_installers_aggressive,
                    state.category_installers,
                )
                .on_toggle(RedistMessage::ToggleInstallers)
                .style(iced::theme::Checkbox::Custom(Box::new(
                    style::CustomCheckboxStyle { custom_colors },
                )))
                .font(lang_font.unwrap_or(Font::DEFAULT)),
            )
            .spacing(8)
            .padding(10),
    )
    .style(iced::theme::Container::Custom(Box::new(
        style::OptionsBoxStyle { custom_colors },
    )))
    .width(Length::Fill);

    let scan_btn = button(
        text(&translations.redist.scan_steam_libraries)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .font(lang_font.unwrap_or(Font::DEFAULT)),
    )
    .on_press(RedistMessage::StartScan)
    .padding(12)
    .width(Length::Fill)
    .style(iced::theme::Button::Custom(Box::new(
        style::PrimaryButtonStyle { custom_colors },
    )));

    let mut scrollable_content = Column::new()
        .push(
            text(&translations.redist.select_categories)
                .size(14)
                .font(lang_font.unwrap_or(Font::DEFAULT)),
        )
        .push(Space::with_height(Length::Fixed(10.0)))
        .push(toggles)
        .push(Space::with_height(Length::Fixed(15.0)))
        .push(scan_btn);

    if state.is_scanning {
        scrollable_content = scrollable_content
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(
                text(&translations.redist.scanning)
                    .size(16)
                    .font(lang_font.unwrap_or(Font::DEFAULT)),
            );
    } else if let Some(results) = &state.scan_results {
        let total_size: u64 = results.iter().map(|i| i.size).sum();
        let count = results.len();

        let summary = text(i18n::format_string(
            &translations.redist.found_folders,
            &[&count.to_string(), &format_size(total_size)],
        ))
        .size(15)
        .style(style::TITLE_COLOR)
        .font(lang_font.unwrap_or(Font::DEFAULT));

        let clean_btn = button(
            text(&translations.redist.clean_selected_items)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .font(lang_font.unwrap_or(Font::DEFAULT)),
        )
        .on_press(RedistMessage::CleanFoundItems)
        .padding(12)
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(
            style::DangerButtonStyle { custom_colors },
        )));

        scrollable_content = scrollable_content
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(summary)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(clean_btn);

        let list_col = results
            .iter()
            .take(50)
            .fold(Column::new().spacing(2), |col, item| {
                col.push(
                    text(format!(
                        "{} - {}",
                        item.path.display(),
                        format_size(item.size)
                    ))
                    .size(11),
                )
            });

        let scroll = scrollable(list_col).height(Length::Fixed(200.0));

        scrollable_content = scrollable_content
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(scroll);
        if count > 50 {
            scrollable_content = scrollable_content.push(
                text(i18n::format_string(
                    &translations.redist.and_more,
                    &[&(count - 50).to_string()],
                ))
                .size(12)
                .font(lang_font.unwrap_or(Font::DEFAULT)),
            );
        }
    }

    if let Some(logs) = &state.last_clean_log {
        scrollable_content = scrollable_content
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(
                text(&translations.redist.clean_results)
                    .size(15)
                    .font(lang_font.unwrap_or(Font::DEFAULT)),
            );
        let log_col = logs.iter().fold(Column::new().spacing(2), |col, msg| {
            col.push(text(msg).size(11))
        });
        let scroll = scrollable(log_col).height(Length::Fixed(150.0));

        let log_container = container(scroll)
            .style(iced::theme::Container::Custom(Box::new(
                style::ConsoleContainerStyle { custom_colors },
            )))
            .padding(10)
            .width(Length::Fill);

        scrollable_content = scrollable_content.push(log_container);
    }

    let back_btn = button(
        text(&translations.redist.back_to_main)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .font(lang_font.unwrap_or(Font::DEFAULT)),
    )
    .on_press(RedistMessage::Close)
    .padding(10)
    .width(Length::Fixed(180.0))
    .style(iced::theme::Button::Custom(Box::new(
        style::PrimaryButtonStyle { custom_colors },
    )));

    let footer = container(back_btn)
        .padding(20)
        .width(Length::Fill)
        .center_x();

    let main_layout = Column::new()
        .push(header)
        .push(
            container(scrollable(scrollable_content))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding([0, 30, 0, 30]),
        )
        .push(footer)
        .width(Length::Fill)
        .height(Length::Fill);

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(iced::theme::Container::Custom(Box::new(
            style::MainWindowStyle { custom_colors },
        )))
        .into()
}
