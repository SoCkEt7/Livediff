<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# interfaces

## Purpose
Port trait definitions used to decouple domain/use-case logic from adapters and infrastructure.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Interface module exports. |
| `ports.rs` | Trait definitions for external capabilities. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Add ports only when a real boundary is needed; keep trait names behavior-oriented.

### Testing Requirements
Compile and run tests for any signature changes.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
