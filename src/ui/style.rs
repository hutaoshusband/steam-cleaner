// src/ui/style.rs

use iced::widget::{button, container, toggler};
use iced::{border, color, Color};

const BASE: Color = color!(0x24, 0x27, 0x3a);
const MANTLE: Color = color!(0x1e, 0x20, 0x30);
const CRUST: Color = color!(0x18, 0x19, 0x26);
const TEXT: Color = color!(0xc6, 0xd0, 0xf5);
const SUBTEXT: Color = color!(0xb5, 0xb8, 0xe3);
const BLUE: Color = color!(0x8a, 0xa9, 0xf7);
const SAPPHIRE: Color = color!(0x7d, 0xc4, 0xe4);
const OVERLAY1: Color = color!(0x6e, 0x73, 0x8d);

pub const TITLE_COLOR: Color = BLUE;

pub struct MainWindowStyle;
impl container::StyleSheet for MainWindowStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(BASE.into()),
            text_color: Some(TEXT),
            ..Default::default()
        }
    }
}


pub struct OptionsBoxStyle;
impl container::StyleSheet for OptionsBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(MANTLE.into()),
            border: border::Border {
                color: OVERLAY1,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}


pub struct PrimaryButtonStyle;
impl button::StyleSheet for PrimaryButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(BLUE.into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: CRUST,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(SAPPHIRE.into()),
            ..active
        }
    }
}


pub struct CustomTogglerStyle;
impl toggler::StyleSheet for CustomTogglerStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: if is_active { BLUE } else { MANTLE },

            background_border_color: OVERLAY1,
            background_border_width: 1.0,
            foreground: if is_active { CRUST } else { SUBTEXT },
            foreground_border_color: Color::TRANSPARENT, 
            foreground_border_width: 0.0,
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        let active = self.active(style, is_active);
        toggler::Appearance {
            background: if is_active { SAPPHIRE } else { BASE },
            ..active
        }
    }
}
