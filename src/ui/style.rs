use iced::widget::{button, checkbox, container, toggler};
use iced::{border, Background, Color, Vector};

// --- RED RETRO VHS / CUTEGORE (DEFAULT / DARK VARIANT) ---
pub const BACKGROUND: Color = Color::from_rgb(0.04, 0.01, 0.02); // Very dark red-black
pub const CRT_RED: Color = Color::from_rgb(0.85, 0.05, 0.05); // Bright VHS red
pub const BLOOD_RED: Color = Color::from_rgb(0.65, 0.02, 0.02); // Deep blood red
pub const DANGER_RED: Color = Color::from_rgb(0.95, 0.1, 0.1); // Warning red
pub const VHS_PINK: Color = Color::from_rgb(0.75, 0.02, 0.35); // Pinkish red accent
pub const GHOST_WHITE: Color = Color::from_rgb(0.98, 0.95, 0.93); // White text with slight pink tint
pub const DIM_RED: Color = Color::from_rgb(0.45, 0.1, 0.15); // Dimmed red for inactive elements
pub const SCANLINE_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1); // Subtle black scanlines
pub const GLITCH_BORDER: Color = Color::from_rgba(0.9, 0.0, 0.1, 0.7); // Glowing red border
pub const GLOW_RED: Color = Color::from_rgba(0.8, 0.0, 0.05, 0.3); // Red glow effect
pub const TEXT: Color = GHOST_WHITE;
pub const SUBTEXT: Color = Color::from_rgb(0.8, 0.5, 0.55);
pub const TITLE_COLOR: Color = CRT_RED;
pub const ACCENT_GREEN: Color = Color::from_rgb(0.15, 0.6, 0.2); // Muted green
pub const ACCENT_BLUE: Color = Color::from_rgb(0.15, 0.3, 0.65); // Muted blue

// Helper for title colors
pub fn title_color(theme: &iced::Theme) -> Color {
    match theme {
        iced::Theme::Light => Color::from_rgb(0.1, 0.1, 0.2), // Dark text for titles
        iced::Theme::Dracula => Color::from_rgb(0.9, 0.9, 0.95), // White text for titles
        _ => TITLE_COLOR,
    }
}

// --- LIGHT MODE (WHITE) ---
pub const LIGHT_BG: Color = Color::from_rgb(0.98, 0.98, 0.99);
pub const LIGHT_SURFACE: Color = Color::from_rgb(1.0, 1.0, 1.0);
pub const LIGHT_TEXT: Color = Color::from_rgb(0.1, 0.1, 0.15);
pub const LIGHT_SUBTEXT: Color = Color::from_rgb(0.4, 0.4, 0.5);
pub const LIGHT_BORDER: Color = Color::from_rgb(0.85, 0.85, 0.9);
pub const LIGHT_PRIMARY: Color = Color::from_rgb(0.2, 0.5, 0.9); // Blue
pub const LIGHT_DANGER: Color = Color::from_rgb(0.9, 0.2, 0.2);
pub const LIGHT_SUCCESS: Color = Color::from_rgb(0.2, 0.7, 0.3);

// --- NEUTRAL DARK (DRACULA/STANDARD) ---
pub const DARK_BG: Color = Color::from_rgb(0.1, 0.1, 0.12);
pub const DARK_SURFACE: Color = Color::from_rgb(0.15, 0.15, 0.18);
pub const DARK_TEXT: Color = Color::from_rgb(0.9, 0.9, 0.95);
pub const DARK_SUBTEXT: Color = Color::from_rgb(0.6, 0.6, 0.7);
pub const DARK_BORDER: Color = Color::from_rgb(0.25, 0.25, 0.3);
pub const DARK_PRIMARY: Color = Color::from_rgb(0.4, 0.6, 0.9); // Soft Blue
pub const DARK_DANGER: Color = Color::from_rgb(0.95, 0.3, 0.3);
pub const DARK_SUCCESS: Color = Color::from_rgb(0.3, 0.8, 0.4);

pub struct MainWindowStyle;
impl container::StyleSheet for MainWindowStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            iced::Theme::Light => container::Appearance {
                background: Some(LIGHT_BG.into()),
                text_color: Some(LIGHT_TEXT),
                ..Default::default()
            },
            iced::Theme::Dracula => container::Appearance {
                background: Some(DARK_BG.into()),
                text_color: Some(DARK_TEXT),
                ..Default::default()
            },
            // Default to Red Retro (Dark)
            _ => container::Appearance {
                background: Some(BACKGROUND.into()),
                text_color: Some(TEXT),
                ..Default::default()
            },
        }
    }
}

pub struct OptionsBoxStyle;
impl container::StyleSheet for OptionsBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            iced::Theme::Light => container::Appearance {
                background: Some(LIGHT_SURFACE.into()),
                border: border::Border {
                    color: LIGHT_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            },
            iced::Theme::Dracula => container::Appearance {
                background: Some(DARK_SURFACE.into()),
                border: border::Border {
                    color: DARK_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            },
            _ => container::Appearance {
                background: Some(Color::from_rgba(0.08, 0.02, 0.04, 0.9).into()),
                border: border::Border {
                    color: GLITCH_BORDER,
                    width: 2.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow {
                    color: GLOW_RED,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            },
        }
    }
}

pub struct PrimaryButtonStyle;
impl button::StyleSheet for PrimaryButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(LIGHT_PRIMARY.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(DARK_PRIMARY.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            _ => button::Appearance {
                background: Some(CRT_RED.into()),
                border: border::Border {
                    color: Color::from_rgb(1.0, 0.0, 0.0),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: GLOW_RED,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.3, 0.6, 1.0).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.5, 0.7, 1.0).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(DANGER_RED.into()),
                border: border::Border {
                    color: Color::from_rgb(1.0, 0.2, 0.2),
                    width: 3.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.95, 0.0, 0.1, 0.5),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.1, 0.4, 0.8).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.3, 0.5, 0.8).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(BLOOD_RED.into()),
                border: border::Border {
                    color: Color::from_rgb(0.8, 0.0, 0.0),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                ..Default::default()
            },
        }
    }
}

pub struct SuccessButtonStyle;
impl button::StyleSheet for SuccessButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(LIGHT_SUCCESS.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(DARK_SUCCESS.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            _ => button::Appearance {
                background: Some(ACCENT_GREEN.into()),
                border: border::Border {
                    color: Color::from_rgb(0.2, 0.7, 0.25),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.2, 0.6, 0.2, 0.3),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.25, 0.75, 0.35).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.35, 0.85, 0.45).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(Color::from_rgb(0.2, 0.75, 0.28).into()),
                border: border::Border {
                    color: Color::from_rgb(0.3, 0.85, 0.35),
                    width: 3.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.25, 0.7, 0.25, 0.5),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light | iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.15, 0.6, 0.2).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(Color::from_rgb(0.12, 0.5, 0.15).into()),
                border: border::Border {
                    color: Color::from_rgb(0.15, 0.6, 0.2),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                ..Default::default()
            },
        }
    }
}

pub struct DangerButtonStyle;
impl button::StyleSheet for DangerButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(LIGHT_DANGER.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(DARK_DANGER.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            _ => button::Appearance {
                background: Some(VHS_PINK.into()),
                border: border::Border {
                    color: Color::from_rgb(1.0, 0.0, 0.4),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.75, 0.0, 0.3, 0.4),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.95, 0.3, 0.3).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(1.0, 0.4, 0.4).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(Color::from_rgb(0.85, 0.05, 0.45).into()),
                border: border::Border {
                    color: Color::from_rgb(1.0, 0.1, 0.5),
                    width: 3.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.85, 0.0, 0.4, 0.6),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            iced::Theme::Light | iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.8, 0.1, 0.1).into()),
                ..active
            },
            _ => button::Appearance {
                background: Some(Color::from_rgb(0.5, 0.0, 0.2).into()),
                border: border::Border {
                    color: Color::from_rgb(0.7, 0.0, 0.3),
                    width: 2.0,
                    radius: 2.0.into(),
                },
                text_color: GHOST_WHITE,
                ..Default::default()
            },
        }
    }
}

pub struct CustomTogglerStyle;
impl toggler::StyleSheet for CustomTogglerStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        match style {
            iced::Theme::Light => toggler::Appearance {
                background: if is_active {
                    LIGHT_PRIMARY
                } else {
                    Color::from_rgb(0.9, 0.9, 0.95)
                },
                background_border_width: 1.0,
                background_border_color: LIGHT_BORDER,
                foreground: Color::WHITE,
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            },
            iced::Theme::Dracula => toggler::Appearance {
                background: if is_active {
                    DARK_PRIMARY
                } else {
                    Color::from_rgb(0.2, 0.2, 0.25)
                },
                background_border_width: 1.0,
                background_border_color: DARK_BORDER,
                foreground: Color::WHITE,
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            },
            _ => toggler::Appearance {
                background: if is_active {
                    CRT_RED
                } else {
                    Color::from_rgba(0.2, 0.05, 0.08, 0.7)
                },
                background_border_width: 2.0,
                background_border_color: if is_active {
                    Color::from_rgb(1.0, 0.2, 0.2)
                } else {
                    Color::from_rgba(0.4, 0.1, 0.15, 0.5)
                },
                foreground: GHOST_WHITE,
                foreground_border_width: 1.0,
                foreground_border_color: Color::from_rgb(0.5, 0.1, 0.15),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        match style {
            iced::Theme::Light => toggler::Appearance {
                background: if is_active {
                    Color::from_rgb(0.3, 0.6, 1.0)
                } else {
                    Color::from_rgb(0.85, 0.85, 0.9)
                },
                ..self.active(style, is_active)
            },
            iced::Theme::Dracula => toggler::Appearance {
                background: if is_active {
                    Color::from_rgb(0.5, 0.7, 1.0)
                } else {
                    Color::from_rgb(0.25, 0.25, 0.3)
                },
                ..self.active(style, is_active)
            },
            _ => toggler::Appearance {
                background: if is_active {
                    DANGER_RED
                } else {
                    Color::from_rgba(0.3, 0.08, 0.12, 0.8)
                },
                background_border_width: 2.0,
                background_border_color: if is_active {
                    Color::from_rgb(1.0, 0.3, 0.3)
                } else {
                    Color::from_rgba(0.5, 0.15, 0.2, 0.6)
                },
                foreground: GHOST_WHITE,
                foreground_border_width: 1.0,
                foreground_border_color: Color::from_rgb(0.6, 0.15, 0.2),
            },
        }
    }
}

pub struct ConsoleContainerStyle;
impl container::StyleSheet for ConsoleContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            iced::Theme::Light => container::Appearance {
                background: Some(Color::from_rgb(0.95, 0.95, 0.96).into()),
                border: border::Border {
                    color: LIGHT_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(Color::from_rgb(0.2, 0.2, 0.25)),
                ..Default::default()
            },
            iced::Theme::Dracula => container::Appearance {
                background: Some(Color::from_rgb(0.08, 0.08, 0.1).into()),
                border: border::Border {
                    color: DARK_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(Color::from_rgb(0.8, 0.8, 0.85)),
                ..Default::default()
            },
            _ => container::Appearance {
                background: Some(Color::from_rgb(0.02, 0.0, 0.0).into()), // Nearly black background
                border: border::Border {
                    color: GLITCH_BORDER,
                    width: 2.0,
                    radius: 2.0.into(), // Sharp corners for terminal look
                },
                text_color: Some(ACCENT_GREEN), // Retro terminal green
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.15, 0.4, 0.15, 0.2),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            },
        }
    }
}

pub enum ThemedButtonStyle {
    Primary,
    Success,
}

impl button::StyleSheet for ThemedButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary => PrimaryButtonStyle.active(style),
            ThemedButtonStyle::Success => SuccessButtonStyle.active(style),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary => PrimaryButtonStyle.hovered(style),
            ThemedButtonStyle::Success => SuccessButtonStyle.hovered(style),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary => PrimaryButtonStyle.pressed(style),
            ThemedButtonStyle::Success => SuccessButtonStyle.pressed(style),
        }
    }
}

pub struct CustomCheckboxStyle;
impl checkbox::StyleSheet for CustomCheckboxStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            iced::Theme::Light => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(LIGHT_PRIMARY)
                } else {
                    Background::Color(Color::WHITE)
                },
                icon_color: Color::WHITE,
                border: border::Border {
                    color: if is_checked {
                        LIGHT_PRIMARY
                    } else {
                        LIGHT_BORDER
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(LIGHT_TEXT),
            },
            iced::Theme::Dracula => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(DARK_PRIMARY)
                } else {
                    Background::Color(Color::from_rgb(0.2, 0.2, 0.25))
                },
                icon_color: Color::WHITE,
                border: border::Border {
                    color: if is_checked {
                        DARK_PRIMARY
                    } else {
                        DARK_BORDER
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(DARK_TEXT),
            },
            _ => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(CRT_RED)
                } else {
                    Background::Color(Color::from_rgba(0.15, 0.03, 0.06, 0.8))
                },
                icon_color: GHOST_WHITE,
                border: border::Border {
                    color: if is_checked {
                        Color::from_rgb(1.0, 0.2, 0.2)
                    } else {
                        Color::from_rgba(0.5, 0.1, 0.2, 0.6)
                    },
                    width: 2.0,
                    radius: 1.0.into(),
                },
                text_color: Some(TEXT),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            iced::Theme::Light => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(Color::from_rgb(0.3, 0.6, 1.0))
                } else {
                    Background::Color(Color::from_rgb(0.95, 0.95, 1.0))
                },
                ..self.active(style, is_checked)
            },
            iced::Theme::Dracula => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(Color::from_rgb(0.5, 0.7, 1.0))
                } else {
                    Background::Color(Color::from_rgb(0.25, 0.25, 0.3))
                },
                ..self.active(style, is_checked)
            },
            _ => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(DANGER_RED)
                } else {
                    Background::Color(Color::from_rgba(0.2, 0.05, 0.1, 0.9))
                },
                icon_color: GHOST_WHITE,
                border: border::Border {
                    color: if is_checked {
                        Color::from_rgb(1.0, 0.3, 0.3)
                    } else {
                        Color::from_rgba(0.6, 0.15, 0.25, 0.7)
                    },
                    width: 2.0,
                    radius: 1.0.into(),
                },
                text_color: Some(TEXT),
            },
        }
    }
}
