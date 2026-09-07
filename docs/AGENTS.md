<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# docs

## Purpose
Static project documentation and hosted web showcase content.

## Key Files
| File | Description |
|------|-------------|
| `index.html` | Interactive static showcase/demo page. |
| `use-cases.md` | Practical workflow recipes for using Livediff. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Keep docs truthful to current CLI behavior and README. If options change in `src/adapters/cli.rs`, update docs together.

### Testing Requirements
For docs-only changes, proofread links; for demo changes, open locally if possible.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
