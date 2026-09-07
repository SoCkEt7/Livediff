// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ThemeKind {
    #[default]
    Cyberpunk,
    Catppuccin,
    TokyoNight,
    Nord,
    Gruvbox,
}

impl ThemeKind {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeKind::Cyberpunk => "Cyberpunk",
            ThemeKind::Catppuccin => "Catppuccin Mocha",
            ThemeKind::TokyoNight => "Tokyo Night",
            ThemeKind::Nord => "Nord",
            ThemeKind::Gruvbox => "Gruvbox Dark",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeKind::Cyberpunk => ThemeKind::Catppuccin,
            ThemeKind::Catppuccin => ThemeKind::TokyoNight,
            ThemeKind::TokyoNight => ThemeKind::Nord,
            ThemeKind::Nord => ThemeKind::Gruvbox,
            ThemeKind::Gruvbox => ThemeKind::Cyberpunk,
        }
    }

    pub fn primary(&self) -> Color {
        match self {
            ThemeKind::Cyberpunk => Color::Rgb(0, 220, 220), // Cyan
            ThemeKind::Catppuccin => Color::Rgb(116, 199, 236), // Sapphire
            ThemeKind::TokyoNight => Color::Rgb(122, 162, 247), // Tokyo Blue
            ThemeKind::Nord => Color::Rgb(136, 192, 208),    // Frost Blue
            ThemeKind::Gruvbox => Color::Rgb(250, 189, 47),  // Warm Gold
        }
    }

    pub fn accent(&self) -> Color {
        match self {
            ThemeKind::Cyberpunk => Color::Rgb(220, 0, 220), // Magenta
            ThemeKind::Catppuccin => Color::Rgb(203, 166, 247), // Mauve
            ThemeKind::TokyoNight => Color::Rgb(187, 154, 247), // Purple
            ThemeKind::Nord => Color::Rgb(129, 161, 193),    // Polar Blue
            ThemeKind::Gruvbox => Color::Rgb(254, 128, 25),  // Orange
        }
    }

    pub fn border_focus(&self) -> Color {
        match self {
            ThemeKind::Cyberpunk => Color::Rgb(80, 80, 110),
            ThemeKind::Catppuccin => Color::Rgb(116, 199, 236),
            ThemeKind::TokyoNight => Color::Rgb(122, 162, 247),
            ThemeKind::Nord => Color::Rgb(136, 192, 208),
            ThemeKind::Gruvbox => Color::Rgb(250, 189, 47),
        }
    }

    pub fn border_dark(&self) -> Color {
        match self {
            ThemeKind::Cyberpunk => Color::Rgb(45, 45, 58),
            ThemeKind::Catppuccin => Color::Rgb(49, 50, 68),
            ThemeKind::TokyoNight => Color::Rgb(36, 40, 59),
            ThemeKind::Nord => Color::Rgb(59, 66, 82),
            ThemeKind::Gruvbox => Color::Rgb(60, 56, 54),
        }
    }
}

impl From<crate::domain::config::ThemeSetting> for ThemeKind {
    fn from(setting: crate::domain::config::ThemeSetting) -> Self {
        match setting {
            crate::domain::config::ThemeSetting::Cyberpunk => ThemeKind::Cyberpunk,
            crate::domain::config::ThemeSetting::Catppuccin => ThemeKind::Catppuccin,
            crate::domain::config::ThemeSetting::TokyoNight => ThemeKind::TokyoNight,
            crate::domain::config::ThemeSetting::Nord => ThemeKind::Nord,
            crate::domain::config::ThemeSetting::Gruvbox => ThemeKind::Gruvbox,
        }
    }
}

impl From<ThemeKind> for crate::domain::config::ThemeSetting {
    fn from(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Cyberpunk => crate::domain::config::ThemeSetting::Cyberpunk,
            ThemeKind::Catppuccin => crate::domain::config::ThemeSetting::Catppuccin,
            ThemeKind::TokyoNight => crate::domain::config::ThemeSetting::TokyoNight,
            ThemeKind::Nord => crate::domain::config::ThemeSetting::Nord,
            ThemeKind::Gruvbox => crate::domain::config::ThemeSetting::Gruvbox,
        }
    }
}
