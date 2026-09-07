<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# adapters

## Purpose
Boundary layer for CLI parsing, filesystem reads, OS file watching, and terminal UI rendering.

## Key Files
| File | Description |
|------|-------------|
| `cli.rs` | Clap CLI arguments and flags. |
| `fs_adapter.rs` | Filesystem helpers for reading file metadata/content. |
| `mod.rs` | Adapter module exports. |
| `watcher.rs` | Notify/ignore based file monitor that sends app events. |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `ui/` | Ratatui components and rendering helpers. |

## For Contributors

### Working In This Directory
Keep IO-specific logic here and convert external events into app/domain types before crossing inward.

### Testing Requirements
Unit/integration tests via `cargo test`; watcher changes should be exercised with temp directories when possible.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
