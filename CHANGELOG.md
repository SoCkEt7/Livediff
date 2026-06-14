# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-14

### Added
- **Interactive TUI**: Sleek terminal user interface built using `ratatui` and `crossterm`.
- **Real-Time Watcher**: Instant detection of file changes using OS native events (`notify` framework).
- **Domain Decoupling**: Clean architecture design that decouples file I/O from core diff logic for increased reliability.
- **Character-Level Diffing**: Detailed diff visualization highlighting added/removed lines using `similar`.
- **Smart Filters**: Native support for `.gitignore` files and custom glob options via CLI `--ignore`.
- **Custom Ignore Menu**: Add folders, file extensions, or specific files to the runtime ignore list directly from the TUI.
- **Cross-Platform Release Workflows**: GitHub Actions workflow to build release binaries for Linux, macOS, and Windows.
