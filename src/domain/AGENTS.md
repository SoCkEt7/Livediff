<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# domain

## Purpose
Core domain model for diffs, ignored paths, file modifications, statistics, and watcher session concepts.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Domain module exports. |
| `diff_engine.rs` | Line-based diff computation using `similar`. |
| `entities.rs` | Domain entities such as file modifications, stats, and domain events. |
| `ignore_engine.rs` | Ignore matching and VCS ignore loading logic. |
| `value_objects.rs` | Small domain value objects. |
| `watcher_session.rs` | Watcher/session domain state. |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `interfaces/` | Port traits for clean boundaries. |

## For Contributors

### Working In This Directory
Keep this layer independent from terminal rendering and OS-specific side effects where possible.

### Testing Requirements
Run focused domain tests plus full `cargo test`; add tests for edge cases in diff/ignore behavior.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
