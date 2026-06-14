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
}
