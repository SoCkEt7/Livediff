# Contributing to Livediff

First off, thank you for considering contributing to Livediff! It's people like you that make the open-source community such a great place to learn, inspire, and create.

## Getting Started

1. Fork the repository and create your branch from `main`.
2. Ensure you have the latest stable version of Rust installed.
3. Run `cargo build` to build the project.
4. Run `cargo test` to ensure all tests pass before making your changes.

## Development Workflow

- **Formatting**: We use `rustfmt`. Please run `cargo fmt` before committing.
- **Linting**: We use `clippy`. Please run `cargo clippy -- -D warnings` to catch common mistakes.
- **Testing**: Add unit tests or integration tests for your new features or bug fixes.

## Pull Request Process

1. Update the `CHANGELOG.md` with details of changes to the interface or behavior.
2. Ensure the test suite passes locally.
3. Submit a Pull Request with a clear title and description.

## Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md).
