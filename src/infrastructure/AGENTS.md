<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# infrastructure

## Purpose
Runtime infrastructure for terminal setup/restore and background logging.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Infrastructure module exports. |
| `logging.rs` | Tracing/log file initialization. |
| `terminal.rs` | Crossterm/Ratatui terminal initialization and restoration. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Terminal restoration must remain panic-safe; avoid logging to the TUI output stream.

### Testing Requirements
Run `cargo test`; manually smoke-run `cargo run -- .` for terminal changes when possible.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
