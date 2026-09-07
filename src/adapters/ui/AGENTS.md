<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# ui

## Purpose
Ratatui component library for the terminal interface: layout, palette, file list, diff view, stats, logs, footer/header, and popups.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | UI module exports, component trait, palette, file-type helpers, and top-level draw layout. |
| `diff_view.rs` | Diff panel rendering and syntax/line-change presentation. |
| `file_list.rs` | Watched-file list rendering and selection state display. |
| `footer.rs` | Footer/status bar rendering. |
| `header.rs` | Header/title rendering. |
| `logs.rs` | Activity log panel rendering. |
| `popups.rs` | Modal overlays for menus, help, ignore input, editor, active ignores, and settings. |
| `stats.rs` | Stats cards and activity sparkline rendering. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Keep rendering deterministic and avoid business logic in widgets. Reuse `Palette` and `Component` patterns instead of ad-hoc styles.

### Testing Requirements
Run `cargo fmt --check` and `cargo test`; for visual changes, run the TUI manually on a temp directory when practical.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
