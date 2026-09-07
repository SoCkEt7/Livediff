<!-- Generated: 2026-09-07 | Updated: 2026-09-07 -->

# livediff

## Purpose
Terminal-first Rust CLI/TUI that watches filesystem changes and renders live diffs for developer workflows. The root contains crate metadata, public docs, examples, demo assets, release/community files, and the main source tree.
Current crate version: `3.2.0` | Rust Edition: `2024`.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Rust crate manifest, dependency list, lints, and release profile. |
| `Cargo.lock` | Pinned dependency graph for reproducible builds. |
| `CLAUDE.md` | Core developer instructions, Clean Architecture invariants, and workflow commands. |
| `README.md` | Public overview, installation, CLI options, and roadmap. |
| `CHANGELOG.md` | Release history and notable changes. |
| `CONTRIBUTING.md` | Contribution workflow and project expectations. |
| `SECURITY.md` | Security policy for reporting vulnerabilities. |
| `rustfmt.toml` | Formatting rules used by rustfmt. |
| `deployment.yaml` | Deployment metadata for the project/demo environment. |
| `demo.html` | Standalone browser demo shell. |
| `demo.gif` | Animated TUI demonstration used by README/docs. |
| `demo.tape` | Terminal recording source for the demo asset. |
| `cassette.tape` | Additional terminal recording source. |
| `write_examples.sh` | Helper script for example/demo generation. |
| `skills-lock.json` | Locked local skill metadata. |
| `livediff_demo` | Demo binary or captured demo artifact. |
| `axum.rs` | Example Rust file used as watched diff input. |
| `compiler.rs` | Example Rust file used as watched diff input. |
| `hooks.tsx` | Example TSX file used as watched diff input. |
| `templates.cpp` | Example C++ file used as watched diff input. |
| `curry.hs` | Example Haskell file used as watched diff input. |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `.claude/` | Local Claude/Codex tool settings. |
| `.github/` | GitHub issue templates, Dependabot, CI, and release workflows. |
| `.internal/` | Internal launch, skill, and open-source preparation material. |
| `docs/` | Static documentation and hosted showcase pages. |
| `src/` | Rust application source organized by adapters, domain, infrastructure, and use cases. |

## For Contributors

### Working In This Directory
Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` after code changes. Keep public-facing docs aligned with Cargo metadata and CLI options. Do not edit generated caches or build outputs.

### Architecture & Layering Requirements
Rust 2024 crate using Clean Hexagonal Architecture:
1. `domain/`: Pure algorithms & entities (diff computation, ignore matching, git info). No IO, terminal, or async deps.
2. `adapters/`: CLI, notify file watcher, Ratatui TUI rendering.
3. `infrastructure/`: Terminal raw mode, signal handling, tracing/logging.
4. `use_cases/`: Event coordination between watcher and app state.

### Testing Requirements
- Unit tests live in `mod tests` beside each module.
- Application/integration tests reside in `src/app/`.
- Zero compiler warnings, zero clippy warnings (`-D warnings`).

### Documentation & Open Source Policy
- Zero AI / LLM mentions in public docs, release notes, or commit messages.
- Pure open-source terminal utility focus.
- **Git Push Policy**: NEVER execute `git push` without explicit user confirmation.

## Dependencies

### Internal
See parent/child AGENTS.md files for adjacent layers and local responsibilities.

### External
Rust crate dependencies are declared in root `Cargo.toml`; GitHub automation depends on GitHub Actions; docs are static HTML/Markdown.

<!-- MANUAL: Any manually added notes below this line are preserved on regeneration -->
