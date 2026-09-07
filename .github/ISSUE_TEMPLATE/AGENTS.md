<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# ISSUE_TEMPLATE

## Purpose
Issue templates for bug reports and feature requests in both Markdown and GitHub issue-form YAML formats.

## Key Files
| File | Description |
|------|-------------|
| `bug_report.md` | Classic bug report template. |
| `bug_report.yml` | Structured bug report issue form. |
| `feature_request.md` | Classic feature request template. |
| `feature_request.yml` | Structured feature request issue form. |

## Subdirectories
No subdirectories requiring separate agent guidance.

## For Contributors

### Working In This Directory
Keep duplicate Markdown/YAML templates semantically aligned when editing.

### Testing Requirements
Validate YAML forms and ensure required fields are clear.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
