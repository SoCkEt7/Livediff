<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# use_cases

## Purpose
Application use cases that coordinate domain logic for incoming file changes.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Use-case module exports. |
| `process_file_change.rs` | Processes file-change events into domain state updates and diff results. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Keep orchestration here thin; move pure rules to domain and side effects to adapters/infrastructure.

### Testing Requirements
Run focused tests if present plus full `cargo test`.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
