# LiveDiff 👁️

[![CI](https://github.com/socket7/livediff/actions/workflows/ci.yml/badge.svg)](https://github.com/socket7/livediff/actions)
[![Crates.io](https://img.shields.io/crates/v/livediff.svg)](https://crates.io/crates/livediff)
[![License](https://img.shields.io/crates/l/livediff.svg)](https://github.com/socket7/livediff/blob/main/LICENSE-MIT)

**LiveDiff** is a blazing-fast, real-time file monitoring CLI tool built in Rust. It provides a beautiful, interactive Terminal User Interface (TUI) to visualize file diffs as they happen.

![LiveDiff Screenshot](https://raw.githubusercontent.com/socket7/livediff/main/assets/screenshot.png) <!-- Placeholder for screenshot -->

## 🚀 Features

- **Real-Time Monitoring**: Instantly detect file changes using native OS filesystem events (powered by `notify`).
- **Beautiful TUI**: A sleek and interactive interface built with `ratatui` and `crossterm`.
- **Advanced Diffing**: See exactly what changed with character-level accuracy using `similar`.
- **High Performance**: Asynchronous architecture powered by `tokio` to handle large codebases without breaking a sweat.
- **Smart Filtering**: Built-in support for `.gitignore` files and custom glob patterns (powered by `ignore`).

## 📦 Installation

### Pre-compiled Binaries (Recommended)

You can download pre-compiled binaries for Windows, macOS, and Linux from the [Releases page](https://github.com/socket7/livediff/releases).

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

This project is dual-licensed under either the [MIT License](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE). You may choose whichever you prefer.
