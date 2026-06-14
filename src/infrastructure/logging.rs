// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use color_eyre::Result;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logging() -> Result<()> {
    let log_dir = std::env::temp_dir().join("Livediff_logs");
    std::fs::create_dir_all(&log_dir)?;
    let file_appender = rolling::daily(log_dir, "Livediff.log");

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(file_appender.with_max_level(Level::INFO))
        .with_ansi(false)
        .init();

    Ok(())
}
