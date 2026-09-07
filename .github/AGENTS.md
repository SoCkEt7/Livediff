<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-06-15 | Updated: 2026-06-15 -->

# .github

## Purpose
GitHub automation and community intake configuration for the repository.

## Key Files
| File | Description |
|------|-------------|
| `dependabot.yml` | Dependency update configuration. |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `ISSUE_TEMPLATE/` | Markdown/YAML issue forms. |
| `workflows/` | CI and release automation. |

## For Contributors

### Working In This Directory
Keep workflows conservative and reproducible. Prefer minimal permissions and pinned action versions when practical.

### Testing Requirements
Validate YAML syntax; for workflow changes use `gh workflow`/local lint if available.

### Common Patterns
Follow existing naming, module exports, and small-diff conventions. Prefer updating nearby tests/docs with behavior changes.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
