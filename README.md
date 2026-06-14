# Livediff 👁️

[![CI](https://github.com/socket7/Livediff/actions/workflows/ci.yml/badge.svg)](https://github.com/socket7/Livediff/actions)
[![Crates.io](https://img.shields.io/crates/v/Livediff.svg)](https://crates.io/crates/Livediff)
[![License](https://img.shields.io/crates/l/Livediff.svg)](https://github.com/socket7/Livediff/blob/main/LICENSE)

**Livediff** is a blazing-fast, real-time file monitoring CLI tool built in Rust. It provides a beautiful, interactive Terminal User Interface (TUI) to visualize file diffs as they happen.

![Livediff Screenshot](https://raw.githubusercontent.com/socket7/Livediff/main/assets/screenshot.png) <!-- Placeholder for screenshot -->

## ⚡ Quick Start

```bash
cargo install livediff
```

## 🚀 Features

- **Real-Time Monitoring**: Instantly detect file changes using native OS filesystem events (powered by `notify`).
- **Beautiful TUI**: A sleek and interactive interface built with `ratatui` and `crossterm`.
- **Advanced Diffing**: See exactly what changed with character-level accuracy using `similar`.
- **High Performance**: Asynchronous architecture powered by `tokio` to handle large codebases without breaking a sweat.
- **Ultra-Low Resource Footprint**: Typically consumes less than 10MB of RAM and 0% CPU when idle, thanks to event-driven redrawing and native OS hooks.
- **Smart Filtering**: Built-in support for `.gitignore` files and custom glob patterns (powered by `ignore`).

## ⚡ Performance & Resource Efficiency

Livediff is engineered from the ground up for maximum resource efficiency, making it the perfect companion tool to keep open in the background without affecting system performance:

- **Zero-CPU Idle**: Uses native OS file system events (`inotify`, `FSEvents`, or `ReadDirectoryChangesW`) and event-driven TUI redrawing, meaning it consumes 0% CPU cycles when no files are changing.
- **Minimal RAM Footprint**: Optimized internal caches and diff representations keep memory usage extremely low (typically around 6-8 MB).
- **No Heavy Runtimes**: Built in pure Rust with zero Node.js/Electron bloating. A single lightweight standalone binary is all you need.

## 📦 Installation

### Pre-compiled Binaries (Recommended)

You can download pre-compiled binaries for Windows, macOS, and Linux from the [Releases page](https://github.com/socket7/Livediff/releases).

### Via Cargo

If you have Rust installed, you can build from source:

```bash
cargo install livediff
```

## 🛠️ Usage

Simply run `livediff` in any directory to start monitoring:

```bash
livediff
```

### Advanced Usage

You can specify a directory or apply filters using standard arguments:

```bash
# Monitor a specific directory
livediff ./src

# Ignore specific files
livediff ./src --ignore "*.tmp" --ignore "target/"
```

## 🤝 Contributing

Contributions are always welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## 📄 License

This project is licensed under either the [MIT License](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE) at your option.

