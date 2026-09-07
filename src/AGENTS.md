<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-09-07 | Updated: 2026-09-07 -->

# src

## Purpose
Rust source for the Livediff binary. The tree separates terminal/UI adapters, application state, domain logic, infrastructure side effects, and use-case orchestration.

## Key Files
| File | Description |
|------|-------------|
| `main.rs` | Binary entrypoint wiring CLI parsing, logging, watcher events, app state, terminal UI, and event loop. |
| `app.rs` | Application state machine, UI state, events, popups, navigation, logs, and interaction logic. |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `adapters/` | CLI (`clap`), filesystem watcher (`notify`), and TUI adapter layer (`ratatui`). |
| `app/` | Integration and application test suites. |
| `domain/` | Core diff (`similar`), ignore matching (`ignore`), git inspection, and domain models. Pure logic without IO. |
| `infrastructure/` | Terminal raw mode management (`crossterm`) and logging infrastructure (`tracing`). |
| `use_cases/` | Application use cases that coordinate domain operations and watcher events. |

## For Contributors

### Working In This Directory
Preserve strict layer boundaries:
- `domain/` stays 100% pure (no IO, no async runtime, no unwrap/panic).
- `adapters/` handles external IO, UI rendering, and CLI arguments.
- `infrastructure/` handles runtime environment, panic hooks, and terminal setup.
- `use_cases/` coordinates watcher events into domain diffs and state updates.

### Testing Requirements
Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` for source changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
