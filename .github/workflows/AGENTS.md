<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# workflows

## Purpose
GitHub Actions definitions for continuous integration and release publishing.

## Key Files
| File | Description |
|------|-------------|
| `ci.yml` | Build/test/lint workflow for pull requests and pushes. |
| `release.yml` | Release packaging/publishing workflow. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Avoid adding secrets or production side effects without explicit approval. Keep matrix and cache changes scoped.

### Testing Requirements
Use `cargo fmt --check`, `cargo clippy`, and `cargo test` locally to mirror CI before claiming workflow correctness.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
