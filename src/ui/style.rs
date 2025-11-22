// src/ui/style.rs

use iced::widget::{button, container, toggler};
use iced::{border, Color, Vector};

// iOS-Style Light Glassmorphism Palette
// Window background - light semi-transparent (not pure transparent to avoid black)
pub const BACKGROUND: Color = Color::from_rgba(0.94, 0.96, 0.98, 0.92);

// Glass containers - translucent white/light
pub const GLASS_BG: Color = Color::from_rgba(0.98, 0.98, 1.0, 0.75);
pub const GLASS_BORDER: Color = Color::from_rgba(0.85, 0.85, 0.9, 0.4);

// Text colors - dark for light backgrounds
pub const TEXT: Color = Color::from_rgb(0.1, 0.1, 0.15);
pub const SUBTEXT: Color = Color::from_rgb(0.4, 0.4, 0.45);

// iOS accent colors
pub const IOS_BLUE: Color = Color::from_rgb(0.0, 0.478, 1.0); // #007AFF
pub const IOS_BLUE_HOVER: Color = Color::from_rgb(0.2, 0.58, 1.0);
pub const IOS_GREEN: Color = Color::from_rgb(0.204, 0.78, 0.349); // #34C759

pub const TITLE_COLOR: Color = TEXT;

// Main Window - transparent background
pub struct MainWindowStyle;
impl container::StyleSheet for MainWindowStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(BACKGROUND.into()),
            text_color: Some(TEXT),
            ..Default::default()
        }
    }
}

// Glass containers - iOS frosted glass effect
pub struct OptionsBoxStyle;
impl container::StyleSheet for OptionsBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(GLASS_BG.into()),
            border: border::Border {
                color: GLASS_BORDER,
                width: 1.5,
                radius: 20.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            ..Default::default()
        }
    }
}

// iOS-style buttons
pub struct PrimaryButtonStyle;
impl button::StyleSheet for PrimaryButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(IOS_BLUE.into()),
            border: border::Border {
                radius: 14.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 2.0),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(IOS_BLUE_HOVER.into()),
            border: border::Border {
                radius: 14.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 4.0),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.0, 0.42, 0.9).into()),
            border: border::Border {
                radius: 14.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

// iOS-style toggles (green when active)
pub struct CustomTogglerStyle;
impl toggler::StyleSheet for CustomTogglerStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: if is_active { IOS_GREEN } else { Color::from_rgba(0.8, 0.8, 0.82, 0.6) },
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Color::WHITE,
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: if is_active { 
                Color::from_rgb(0.25, 0.82, 0.4)
            } else { 
                Color::from_rgba(0.85, 0.85, 0.87, 0.7) 
            },
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Color::WHITE,
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
        }
    }
}

// Console/Log container style - dark terminal look
pub struct ConsoleContainerStyle;
impl container::StyleSheet for ConsoleContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()), // Almost black
            border: border::Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(Color::from_rgb(0.2, 0.8, 0.2)), // Terminal green text
            ..Default::default()
        }
    }
}

// Transparent button for draggable title bar
pub struct TransparentButtonStyle;
impl button::StyleSheet for TransparentButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border: border::Border::default(),
            text_color: TEXT,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
            border: border::Border::default(),
            text_color: TEXT,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.1).into()),
            ..Default::default()
        }
    }
}
