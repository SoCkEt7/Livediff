// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeSetting {
    #[default]
    Cyberpunk,
    Catppuccin,
    TokyoNight,
    Nord,
    Gruvbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewModeSetting {
    #[default]
    Unified,
    Split,
}

fn default_debounce_ms() -> u64 {
    25
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserConfig {
    pub theme: ThemeSetting,
    pub view_mode: ViewModeSetting,
    pub wrap_lines: bool,
    pub ignore_whitespace: bool,
    pub respect_vcs_ignore: bool,
    pub tick_rate_ms: u64,
    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            theme: ThemeSetting::Cyberpunk,
            view_mode: ViewModeSetting::Unified,
            wrap_lines: false,
            ignore_whitespace: false,
            respect_vcs_ignore: true,
            tick_rate_ms: 150,
            debounce_ms: 25,
        }
    }
}
