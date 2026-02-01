use iced::widget::{button, container, toggler};
use iced::{border, Color, Vector};

pub const BACKGROUND: Color = Color::from_rgb(0.08, 0.08, 0.10);
pub const GLASS_BG: Color = Color::from_rgba(0.18, 0.18, 0.22, 0.85);
pub const GLASS_BORDER: Color = Color::from_rgba(0.35, 0.35, 0.40, 0.5);
pub const TEXT: Color = Color::from_rgb(0.95, 0.95, 0.97);
pub const SUBTEXT: Color = Color::from_rgb(0.65, 0.65, 0.70);
pub const IOS_BLUE: Color = Color::from_rgb(0.35, 0.60, 1.0);
pub const IOS_BLUE_HOVER: Color = Color::from_rgb(0.50, 0.70, 1.0);
pub const IOS_GREEN: Color = Color::from_rgb(0.35, 0.80, 0.45);
pub const TITLE_COLOR: Color = Color::from_rgb(1.0, 1.0, 1.0);

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

pub struct OptionsBoxStyle;
impl container::StyleSheet for OptionsBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(GLASS_BG.into()),
            border: border::Border {
                color: GLASS_BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
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
            background: Some(IOS_BLUE.into()),
            border: border::Border {
                radius: 8.0.into(),
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
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 4.0),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.30, 0.50, 0.85).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

pub struct SuccessButtonStyle;
impl button::StyleSheet for SuccessButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(IOS_GREEN.into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 2.0),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.40, 0.90, 0.50).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 4.0),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.25, 0.70, 0.35).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

pub struct DangerButtonStyle;
impl button::StyleSheet for DangerButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.9, 0.3, 0.25).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 2.0),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(1.0, 0.4, 0.35).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            shadow_offset: Vector::new(0.0, 4.0),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb(0.75, 0.2, 0.18).into()),
            border: border::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

pub struct CustomTogglerStyle;
impl toggler::StyleSheet for CustomTogglerStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: if is_active {
                IOS_GREEN
            } else {
                Color::from_rgba(0.35, 0.35, 0.40, 0.7)
            },
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
                Color::from_rgb(0.40, 0.85, 0.50)
            } else {
                Color::from_rgba(0.40, 0.40, 0.45, 0.8)
            },
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Color::WHITE,
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
        }
    }
}

pub struct ConsoleContainerStyle;
impl container::StyleSheet for ConsoleContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgb(0.05, 0.05, 0.08).into()),
            border: border::Border {
                color: Color::from_rgb(0.25, 0.25, 0.30),
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(Color::from_rgb(0.45, 0.90, 0.45)),
            ..Default::default()
        }
    }
}
