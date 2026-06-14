# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
