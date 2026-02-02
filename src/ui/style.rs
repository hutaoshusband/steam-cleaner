use iced::widget::{button, checkbox, container, toggler};
use iced::{border, Background, Color, Vector};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct CustomThemeColors {
    #[serde(with = "color_serde")]
    pub background: Color,
    #[serde(with = "color_serde")]
    pub surface: Color,
    #[serde(with = "color_serde")]
    pub text: Color,
    #[serde(with = "color_serde")]
    pub primary: Color,
    #[serde(with = "color_serde")]
    pub danger: Color,
    #[serde(with = "color_serde")]
    pub success: Color,
}

mod color_serde {
    use iced::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [color.r, color.g, color.b, color.a].serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [r, g, b, a] = <[f32; 4]>::deserialize(deserializer)?;
        Ok(Color { r, g, b, a })
    }
}

impl Default for CustomThemeColors {
    fn default() -> Self {
        Self {
            background: Color::from_rgb(0.1, 0.11, 0.15),
            surface: Color::from_rgb(0.14, 0.16, 0.23),
            text: Color::from_rgb(0.75, 0.79, 0.96),
            primary: Color::from_rgb(0.48, 0.64, 0.97),
            danger: Color::from_rgb(0.97, 0.46, 0.56),
            success: Color::from_rgb(0.62, 0.93, 0.42),
        }
    }
}

impl CustomThemeColors {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("custom_theme.json", json)
    }

    pub fn load() -> Self {
        if let Ok(content) = std::fs::read_to_string("custom_theme.json") {
            if let Ok(colors) = serde_json::from_str(&content) {
                return colors;
            }
        }
        Self::default()
    }
}

// Helper for title colors
pub fn title_color(theme: &iced::Theme) -> Color {
    match theme {
        iced::Theme::Light => Color::from_rgb(0.1, 0.1, 0.2), // Dark text for titles
        iced::Theme::Dracula => Color::from_rgb(0.9, 0.9, 0.95), // White text for titles
        iced::Theme::Nord => NORD_PRIMARY,                    // Muted blue for Ultra Dark
        iced::Theme::SolarizedLight => CREAM_PRIMARY,         // Warm amber for Cream
        _ => TITLE_COLOR,
    }
}

// ... constants ...
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

// --- ULTRA DARK (NORD) ---
pub const NORD_BG: Color = Color::from_rgb(0.015, 0.018, 0.025); // Ultra dark - almost black (darker than default Dark)
pub const NORD_SURFACE: Color = Color::from_rgb(0.025, 0.028, 0.035); // Slightly lighter surface
pub const NORD_TEXT: Color = Color::from_rgb(0.95, 0.95, 0.97); // Nearly white text
pub const NORD_SUBTEXT: Color = Color::from_rgb(0.65, 0.68, 0.72);
pub const NORD_BORDER: Color = Color::from_rgb(0.08, 0.1, 0.15); // Very subtle dark border
pub const NORD_PRIMARY: Color = Color::from_rgb(0.4, 0.5, 0.65); // Muted blue accent
pub const NORD_DANGER: Color = Color::from_rgb(0.75, 0.3, 0.35); // Dark red
pub const NORD_SUCCESS: Color = Color::from_rgb(0.35, 0.6, 0.4); // Muted green

// --- CREAM (SOLARIZED LIGHT) ---
pub const CREAM_BG: Color = Color::from_rgb(0.96, 0.93, 0.88); // Warm cream background
pub const CREAM_SURFACE: Color = Color::from_rgb(0.99, 0.96, 0.91); // Lighter cream surface
pub const CREAM_TEXT: Color = Color::from_rgb(0.35, 0.3, 0.25); // Warm dark brown text
pub const CREAM_SUBTEXT: Color = Color::from_rgb(0.55, 0.5, 0.45);
pub const CREAM_BORDER: Color = Color::from_rgb(0.85, 0.8, 0.72); // Warm beige border
pub const CREAM_PRIMARY: Color = Color::from_rgb(0.75, 0.55, 0.35); // Warm amber/bronze
pub const CREAM_DANGER: Color = Color::from_rgb(0.85, 0.35, 0.3); // Warm red
pub const CREAM_SUCCESS: Color = Color::from_rgb(0.55, 0.7, 0.4); // Olive green

#[derive(Default)]
pub struct MainWindowStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl container::StyleSheet for MainWindowStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        if let Some(colors) = &self.custom_colors {
            return container::Appearance {
                background: Some(colors.background.into()),
                text_color: Some(colors.text),
                ..Default::default()
            };
        }

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
            iced::Theme::Nord => container::Appearance {
                background: Some(NORD_BG.into()),
                text_color: Some(NORD_TEXT),
                ..Default::default()
            },
            iced::Theme::SolarizedLight => container::Appearance {
                background: Some(CREAM_BG.into()),
                text_color: Some(CREAM_TEXT),
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

pub struct OptionsBoxStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl Default for OptionsBoxStyle {
    fn default() -> Self {
        Self { custom_colors: None }
    }
}

impl container::StyleSheet for OptionsBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        if let Some(colors) = &self.custom_colors {
            return container::Appearance {
                background: Some(colors.surface.into()),
                border: border::Border {
                    color: colors.primary,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            };
        }

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
            iced::Theme::Nord => container::Appearance {
                background: Some(NORD_SURFACE.into()),
                border: border::Border {
                    color: NORD_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                    offset: Vector::new(0.0, 3.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            },
            iced::Theme::SolarizedLight => container::Appearance {
                background: Some(CREAM_SURFACE.into()),
                border: border::Border {
                    color: CREAM_BORDER,
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

#[derive(Default)]
pub struct PrimaryButtonStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl button::StyleSheet for PrimaryButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        if let Some(colors) = &self.custom_colors {
            return button::Appearance {
                background: Some(colors.primary.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: colors.background, // Contrast?
                ..Default::default()
            };
        }

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
            iced::Theme::Nord => button::Appearance {
                background: Some(NORD_PRIMARY.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(CREAM_PRIMARY.into()),
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
        if let Some(_) = &self.custom_colors {
             return button::Appearance {
                shadow: iced::Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
                ..active
            };
        }

        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.3, 0.6, 1.0).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.5, 0.7, 1.0).into()),
                ..active
            },
            iced::Theme::Nord => button::Appearance {
                background: Some(Color::from_rgb(0.65, 0.75, 0.85).into()),
                ..active
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(Color::from_rgb(0.85, 0.65, 0.45).into()),
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
        if self.custom_colors.is_some() { return active; }
        
        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.1, 0.4, 0.8).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.3, 0.5, 0.8).into()),
                ..active
            },
            iced::Theme::Nord => button::Appearance {
                background: Some(Color::from_rgb(0.45, 0.55, 0.65).into()),
                ..active
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(Color::from_rgb(0.65, 0.45, 0.25).into()),
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

#[derive(Default)]
pub struct SuccessButtonStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl button::StyleSheet for SuccessButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        if let Some(colors) = &self.custom_colors {
            return button::Appearance {
                background: Some(colors.success.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: colors.background,
                ..Default::default()
            };
        }
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
            iced::Theme::Nord => button::Appearance {
                background: Some(NORD_SUCCESS.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(CREAM_SUCCESS.into()),
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
        if self.custom_colors.is_some() { return active; }

        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.25, 0.75, 0.35).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(0.35, 0.85, 0.45).into()),
                ..active
            },
            iced::Theme::Nord => button::Appearance {
                background: Some(Color::from_rgb(0.7, 0.8, 0.68).into()),
                ..active
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(Color::from_rgb(0.75, 0.85, 0.6).into()),
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
        if self.custom_colors.is_some() { return active; }

        match style {
            iced::Theme::Light
            | iced::Theme::Dracula
            | iced::Theme::Nord
            | iced::Theme::SolarizedLight => button::Appearance {
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

#[derive(Default)]
pub struct DangerButtonStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl button::StyleSheet for DangerButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        if let Some(colors) = &self.custom_colors {
             return button::Appearance {
                background: Some(colors.danger.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: colors.background,
                ..Default::default()
            };
        }

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
            iced::Theme::Nord => button::Appearance {
                background: Some(NORD_DANGER.into()),
                border: border::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                text_color: Color::WHITE,
                ..Default::default()
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(CREAM_DANGER.into()),
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
        if self.custom_colors.is_some() { return active; }

        match style {
            iced::Theme::Light => button::Appearance {
                background: Some(Color::from_rgb(0.95, 0.3, 0.3).into()),
                ..active
            },
            iced::Theme::Dracula => button::Appearance {
                background: Some(Color::from_rgb(1.0, 0.4, 0.4).into()),
                ..active
            },
            iced::Theme::Nord => button::Appearance {
                background: Some(Color::from_rgb(0.85, 0.5, 0.55).into()),
                ..active
            },
            iced::Theme::SolarizedLight => button::Appearance {
                background: Some(Color::from_rgb(0.95, 0.55, 0.5).into()),
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
        if self.custom_colors.is_some() { return active; }

        match style {
            iced::Theme::Light
            | iced::Theme::Dracula
            | iced::Theme::Nord
            | iced::Theme::SolarizedLight => button::Appearance {
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

#[derive(Default)]
pub struct CustomTogglerStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl toggler::StyleSheet for CustomTogglerStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
        if let Some(colors) = &self.custom_colors {
             return toggler::Appearance {
                background: if is_active {
                    colors.primary
                } else {
                    colors.surface
                },
                background_border_width: 1.0,
                background_border_color: colors.primary,
                foreground: colors.background, // Toggle circle color? Contrast
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            };
        }

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
            iced::Theme::Nord => toggler::Appearance {
                background: if is_active {
                    NORD_PRIMARY
                } else {
                    Color::from_rgb(0.2, 0.22, 0.28)
                },
                background_border_width: 1.0,
                background_border_color: NORD_BORDER,
                foreground: Color::WHITE,
                foreground_border_width: 0.0,
                foreground_border_color: Color::TRANSPARENT,
            },
            iced::Theme::SolarizedLight => toggler::Appearance {
                background: if is_active {
                    CREAM_PRIMARY
                } else {
                    Color::from_rgb(0.9, 0.87, 0.82)
                },
                background_border_width: 1.0,
                background_border_color: CREAM_BORDER,
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
        if let Some(_) = &self.custom_colors {
            return self.active(style, is_active);
        }
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
            iced::Theme::Nord => toggler::Appearance {
                background: if is_active {
                    Color::from_rgb(0.63, 0.75, 0.85)
                } else {
                    Color::from_rgb(0.25, 0.28, 0.35)
                },
                ..self.active(style, is_active)
            },
            iced::Theme::SolarizedLight => toggler::Appearance {
                background: if is_active {
                    Color::from_rgb(0.83, 0.63, 0.43)
                } else {
                    Color::from_rgb(0.88, 0.83, 0.78)
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

#[derive(Default)]
pub struct ConsoleContainerStyle {
    pub custom_colors: Option<CustomThemeColors>,
}

impl container::StyleSheet for ConsoleContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        if let Some(colors) = &self.custom_colors {
             return container::Appearance {
                background: Some(colors.background.into()),
                border: border::Border {
                    color: colors.primary,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(colors.success), // Console text often green/accent
                ..Default::default()
            };
        }

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
            iced::Theme::Nord => container::Appearance {
                background: Some(Color::from_rgb(0.12, 0.14, 0.18).into()),
                border: border::Border {
                    color: NORD_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(Color::from_rgb(0.8, 0.85, 0.75)),
                ..Default::default()
            },
            iced::Theme::SolarizedLight => container::Appearance {
                background: Some(Color::from_rgb(0.94, 0.91, 0.86).into()),
                border: border::Border {
                    color: CREAM_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(Color::from_rgb(0.4, 0.5, 0.35)),
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
    Primary(Option<CustomThemeColors>),
    Success(Option<CustomThemeColors>),
    Danger(Option<CustomThemeColors>),
}

impl button::StyleSheet for ThemedButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary(colors) => PrimaryButtonStyle { custom_colors: *colors }.active(style),
            ThemedButtonStyle::Success(colors) => SuccessButtonStyle { custom_colors: *colors }.active(style),
            ThemedButtonStyle::Danger(colors) => DangerButtonStyle { custom_colors: *colors }.active(style),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary(colors) => PrimaryButtonStyle { custom_colors: *colors }.hovered(style),
            ThemedButtonStyle::Success(colors) => SuccessButtonStyle { custom_colors: *colors }.hovered(style),
            ThemedButtonStyle::Danger(colors) => DangerButtonStyle { custom_colors: *colors }.hovered(style),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ThemedButtonStyle::Primary(colors) => PrimaryButtonStyle { custom_colors: *colors }.pressed(style),
            ThemedButtonStyle::Success(colors) => SuccessButtonStyle { custom_colors: *colors }.pressed(style),
            ThemedButtonStyle::Danger(colors) => DangerButtonStyle { custom_colors: *colors }.pressed(style),
        }
    }
}

pub struct CustomCheckboxStyle {
     pub custom_colors: Option<CustomThemeColors>,
}

pub struct ColorPreviewStyle {
    pub color: Color,
}

impl container::StyleSheet for ColorPreviewStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.color.into()),
            border: border::Border {
                color: Color::WHITE,
                width: 2.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

impl button::StyleSheet for ColorPreviewStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.color.into()),
            border: border::Border {
                color: Color::WHITE,
                width: 2.0,
                radius: 4.0.into(),
            },
            text_color: if self.color.r + self.color.g + self.color.b > 1.5 {
                Color::BLACK
            } else {
                Color::WHITE
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            shadow: iced::Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            shadow: iced::Shadow::default(),
            ..active
        }
    }
}

pub struct PreviewBoxStyle {
    pub bg: Color,
    pub text: Color,
}

impl container::StyleSheet for PreviewBoxStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(self.bg.into()),
            text_color: Some(self.text),
            border: border::Border {
                color: self.text,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}

impl checkbox::StyleSheet for CustomCheckboxStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        if let Some(colors) = &self.custom_colors {
            return checkbox::Appearance {
                background: if is_checked {
                    Background::Color(colors.primary)
                } else {
                    Background::Color(colors.surface)
                },
                icon_color: colors.background,
                border: border::Border {
                    color: if is_checked {
                        colors.primary
                    } else {
                        colors.text
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(colors.text),
            };
        }

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
            iced::Theme::Nord => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(NORD_PRIMARY)
                } else {
                    Background::Color(Color::from_rgb(0.22, 0.25, 0.32))
                },
                icon_color: Color::WHITE,
                border: border::Border {
                    color: if is_checked {
                        NORD_PRIMARY
                    } else {
                        NORD_BORDER
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(NORD_TEXT),
            },
            iced::Theme::SolarizedLight => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(CREAM_PRIMARY)
                } else {
                    Background::Color(Color::from_rgb(0.95, 0.92, 0.87))
                },
                icon_color: Color::WHITE,
                border: border::Border {
                    color: if is_checked {
                        CREAM_PRIMARY
                    } else {
                        CREAM_BORDER
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(CREAM_TEXT),
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
            iced::Theme::Nord => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(Color::from_rgb(0.65, 0.75, 0.85))
                } else {
                    Background::Color(Color::from_rgb(0.28, 0.31, 0.38))
                },
                ..self.active(style, is_checked)
            },
            iced::Theme::SolarizedLight => checkbox::Appearance {
                background: if is_checked {
                    Background::Color(Color::from_rgb(0.85, 0.65, 0.45))
                } else {
                    Background::Color(Color::from_rgb(0.9, 0.87, 0.82))
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
