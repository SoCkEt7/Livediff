<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# app

## Purpose
Application-level tests for the event loop/state behavior that do not belong to a single domain module.

## Key Files
| File | Description |
|------|-------------|
| `app_tests.rs` | Integration-style tests for app/domain/UI state interactions. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Keep tests focused on observable behavior and avoid coupling to fragile rendering details.

### Testing Requirements
Run `cargo test app_tests` or full `cargo test`.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
