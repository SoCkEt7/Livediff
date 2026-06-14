pub mod cli;

use clap::Parser;
use color_eyre::Result;
use tracing::{Level, info};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Panic/Error handler for TUI (restores terminal on panic)
    color_eyre::install()?;

    // 2. Parse CLI arguments
    let cli = cli::Cli::parse();

    // 3. Initialize background logging (to file, not stdout)
    let log_dir = std::env::temp_dir().join("livediff_logs");
    std::fs::create_dir_all(&log_dir)?;
    let file_appender = rolling::daily(log_dir, "livediff.log");

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(file_appender.with_max_level(Level::INFO))
        .with_ansi(false)
        .init();

    info!("Starting LiveDiff monitoring at path: {:?}", cli.path);

    // TODO: Initialize TUI and start monitoring here.
    // For now, just print success to standard output (before we take over terminal).
    println!("LiveDiff initialized successfully with path {:?}", cli.path);

    Ok(())
}
