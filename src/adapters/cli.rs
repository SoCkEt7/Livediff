// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use clap::Parser;
use std::path::PathBuf;

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

    /// Do not respect ignore files (.gitignore, .ignore, etc.)
    #[arg(long, default_value_t = false)]
    pub no_ignore: bool,

    /// Do not respect ignore files in parent directories
    #[arg(long, default_value_t = false)]
    pub no_ignore_parent: bool,

    /// Do not respect git/VCS ignore files (.gitignore, etc.)
    #[arg(long, default_value_t = false)]
    pub no_ignore_vcs: bool,
}
