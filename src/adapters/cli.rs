// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use clap::{Parser, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

use crate::domain::config::ThemeSetting;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeArg {
    Cyberpunk,
    Catppuccin,
    TokyoNight,
    Nord,
    Gruvbox,
}

impl From<ThemeArg> for ThemeSetting {
    fn from(arg: ThemeArg) -> Self {
        match arg {
            ThemeArg::Cyberpunk => ThemeSetting::Cyberpunk,
            ThemeArg::Catppuccin => ThemeSetting::Catppuccin,
            ThemeArg::TokyoNight => ThemeSetting::TokyoNight,
            ThemeArg::Nord => ThemeSetting::Nord,
            ThemeArg::Gruvbox => ThemeSetting::Gruvbox,
        }
    }
}

/// Real-time file monitoring with beautiful diff visualization.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to monitor
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Ignore files matching this glob pattern (can be used multiple times)
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Show hidden files
    #[arg(long, default_value_t = false)]
    pub show_hidden: bool,

    /// Do not respect ignore files (.gitignore, .livediffignore, etc.)
    #[arg(long, default_value_t = false)]
    pub no_ignore: bool,

    /// Do not respect ignore files in parent directories
    #[arg(long, default_value_t = false)]
    pub no_ignore_parent: bool,

    /// Do not respect git/VCS ignore files (.gitignore, etc.)
    #[arg(long, default_value_t = false)]
    pub no_ignore_vcs: bool,

    /// Start in Side-by-Side (Split) diff view mode
    #[arg(short = 's', long, default_value_t = false)]
    pub split: bool,

    /// Start with whitespace changes ignored in diffs
    #[arg(short = 'w', long, default_value_t = false)]
    pub ignore_whitespace: bool,

    /// Start with soft line-wrapping enabled
    #[arg(short = 'W', long, default_value_t = false)]
    pub wrap_lines: bool,

    /// Initial color theme palette
    #[arg(long, value_enum)]
    pub theme: Option<ThemeArg>,

    /// Filesystem debounce window in milliseconds
    #[arg(long, default_value_t = 25)]
    pub debounce_ms: u64,

    /// Generate shell completions for the specified shell and exit
    #[arg(long, value_enum)]
    pub generate_completions: Option<Shell>,
}
