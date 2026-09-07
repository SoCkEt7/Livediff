# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.3.0] - 2026-09-07

### Added
- **Dynamic Multi-Palette Themes**: Instant runtime switching (`t` / `T`) between Cyberpunk, Catppuccin Mocha, Tokyo Night, Nord, and Gruvbox Dark.
- **Direct Clipboard Patch Yank**: Fast one-key copy (`y` / `Y`) of unified git patch directly into system clipboard via `YankDiffUseCase`.
- **Soft-Wrap Diff View Toggle**: Toggle text wrapping for long diff lines (`W`) with visual `[WRAP]` status indicator in both Unified and Side-by-Side Split views.
- **Visual Add/Del Ratio Distribution Gauge**: Graphical balance bar `[████░░░░]` dynamically rendered in the stats bar.
- **Side-by-Side (Split) Diff View**: Toggle between Unified and Split diff panes (`v` or `Tab`) with synchronized line scrolling and dual gutters.
- **Interactive File Filtering**: Live substring search filter activated via `/` with real-time matching indicators.
- **One-Key Patch Snapshot Export**: Instant snapshot export of the current diff into a standard `.patch` file via `s` / `S`.
- **Ignore Whitespace Toggle**: Dynamic toggle (`w`) and CLI flag `--ignore-whitespace` (`-w`) to ignore indentation and formatting variations.
- **CLI View Mode Flag**: `--split` (`-s`) CLI option to launch directly in Side-by-Side diff view.
- **Fast Navigation**: Top and bottom jumps (`g` / `G`) across file lists and diff buffers.
- **Line Number Gutters**: Dual line number tracking (original and destination lines) with syntax highlighting.
- **Adaptive Event Debouncing**: High-throughput file watcher with sub-50ms event debouncing and deduplication to eliminate TUI flicker during rapid batch writes.
- **Super Clean Hexagonal Architecture**: Strict decoupling into pure domain entities/services, dedicated application use cases (`ExportPatchUseCase`, `YankDiffUseCase`, `ManageIgnoresUseCase`, `ProcessFileChangeUseCase`), isolated adapters, and infrastructure.

### Changed
- Updated crate dependencies: `similar 3.2.0`, `clap 4.6.6`, `tokio 1.53.1`, `ignore 0.4.33`, `globset 0.4.20`, `serde 1.0.229`, `tui-big-text 0.8.9`.
- Enhanced footer status bar and help menu overlay with all new keybindings.

## [3.2.0] - 2026-07-11

### Added
- **Git Integration**: Direct Git integration within the TUI interface.

### Changed
- Rebranded copyright and UI elements to Nyxia.
- Updated all dependencies to latest compatible versions via `cargo update`, including `regex v1.13.0`, `ignore v0.4.28`, `rust-embed v8.12.0`, `bytes v1.12.1`, and 22 other indirect dependency bumps.
- Aligned `Cargo.toml` version with the established release sequence (3.x track).

### Fixed
- Fixed UI freeze on native Linux systems (specifically Fedora).
- Resolved `Cargo.toml` version drift (reported `0.2.5` while the published crate was `3.1.0`).

## [0.2.5] - 2026-06-14

### Changed
- Clarified README Homebrew wording so it stays a planned, non-live install channel until a tap or formula is verified.

## [0.2.4] - 2026-06-14

### Fixed
- Replaced the unavailable release upload action with `gh release upload` using the built-in GitHub CLI.

## [0.2.3] - 2026-06-14

### Fixed
- Granted release workflow write permission for GitHub release creation and asset upload.

## [0.2.2] - 2026-06-14

### Fixed
- Fixed the release workflow setup so the release job checks out the repository before reading `CHANGELOG.md`.

## [0.2.1] - 2026-06-14

### Fixed
- Corrected GitHub release artifact packaging to use the lowercase `livediff` binary name produced by Cargo.
- Aligned Cargo metadata with the public repository and interactive showcase URLs.

### Changed
- Reworked the README around concrete live-diff workflows, clearer installation, comparison guidance, and contributor entry points.
- Added practical use-case documentation plus issue templates for bugs and feature requests.

## [0.2.0] - 2026-06-14

### Added
- **Dynamic Graphics**: Integrated `tui-shimmer` for pulsing status indicators and shimmering panel titles.
- **Big Text Logo**: High-impact "LIVEDIF" logo using `tui-big-text` displayed when no changes are active.
- **Sleek Overlays**: Integrated `tui-overlay` for translucent, centered modal windows with professional background dimming.
- **Global Settings Menu**: Centralized control for `.gitignore` respect and file visibility.
- **Advanced Ignore Management**: New menu to view and remove active session-added ignore patterns with bulk clear support.
- **Toast Notifications**: Non-intrusive floating alerts for file saves, settings changes, and status updates.
- **Visual Heat-map**: File list icons now change color dynamically based on modification intensity.
- **Interactive Web Showcase**: A modern `demo.html` for a browser-based preview of the Livediff experience.
- **Desktop Entry**: Added `livediff.desktop` for easier integration with Linux desktop environments.

### Fixed
- **UI Logic Simplification**: Refactored the internal event loop to use unified overlay state management, significantly improving stability.
- **Save Prompt UX**: Translucent dimming now properly targets the editor area when saving, providing better focus.

## [0.1.0] - 2026-06-14

### Added
- **Interactive TUI**: Sleek terminal user interface built using `ratatui` and `crossterm`.
- **Real-Time Watcher**: Instant detection of file changes using OS native events (`notify` framework).
- **Domain Decoupling**: Clean architecture design that decouples file I/O from core diff logic for increased reliability.
- **Character-Level Diffing**: Detailed diff visualization highlighting added/removed lines using `similar`.
- **Smart Filters**: Native support for `.gitignore` files and custom glob options via CLI `--ignore`.
- **Custom Ignore Menu**: Add folders, file extensions, or specific files to the runtime ignore list directly from the TUI.
- **Cross-Platform Release Workflows**: GitHub Actions workflow to build release binaries for Linux, macOS, and Windows.
