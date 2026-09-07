# CLAUDE.md — Livediff Guidelines & Project Instructions

> **Livediff**: Real-time terminal file diff companion to `git diff` built with Rust 2024, `ratatui`, `notify`, `similar`, and `syntect`.
> Current crate version: `3.2.0` | Target Edition: `2024` (MSRV 1.85+)

---

## 🛠️ Build, Lint & Verification Commands

All workflows use the standard Rust toolchain (use `cargo` for crate workflows, `bun`/`bunx` if JS/TS tooling is invoked — **never** `npm`/`npx`):

```bash
# Code formatting check
cargo fmt --check

# Apply formatting automatically
cargo fmt

# Strict linting (zero warnings permitted in CI)
cargo clippy --all-targets -- -D warnings

# Run all test suites
cargo test

# Run tests with verbose output & unmuted stdout
cargo test -- --nocapture

# Build release binary (optimized)
cargo build --release

# Run livediff locally against the current repository
cargo run -- .
```

---

## 🏗️ Architecture & Layering Rules

The codebase follows Clean/Hexagonal architecture across strict domain boundaries:

```
src/
├── adapters/          # Inbound & outbound ports: CLI parsing, notify watcher, TUI (ratatui)
│   ├── cli/           # Clap CLI arguments and options parser
│   ├── ui/            # Ratatui UI rendering, components, syntax highlighting
│   └── watcher/       # Notify filesystem event debouncer & receiver
├── app.rs             # Core App state machine, UI state, history, popups & navigation
├── domain/            # Pure domain models: diff calculation, ignore matching, sessions
│   ├── diff_engine.rs # Similar-based line & character level diff computation
│   ├── ignore_engine.rs # Ignore/gitignore rule matching & directory filtering
│   ├── git_info.rs    # Repository status & branch inspection
│   └── models.rs      # Entities, DiffResult, FileChange events
├── infrastructure/    # Side-effects & OS boundaries: raw terminal, tracing/logging
│   ├── logging/       # Tracing-subscriber, appender, log files
│   └── terminal/      # Crossterm raw mode setup, restore & panic hooks
├── use_cases/         # Application workflow coordinators
│   └── monitor_repository.rs # Orchestrates watcher events -> diff -> app state
└── main.rs            # Entrypoint, tokio async runtime & main event loop
```

### Core Invariants:
1. **Domain Isolation**: `domain/` must remain pure logic without terminal, async, or IO dependencies.
2. **Side-Effect Encapsulation**: Terminal escape sequences, raw mode, and OS signals belong strictly in `infrastructure/terminal/`.
3. **Robust Error Handling**: Zero raw `unwrap()` or `panic!` in domain logic. Use `Result` with explicit error propagation.
4. **Rust 2024 Idioms**: Adhere to modern Rust 2024 idioms, strict clippy lints, and zero dead-code warnings.

---

## 🧪 Testing & Verification Guidelines

- **Unit tests** reside adjacent to domain logic within `#[cfg(test)] mod tests { ... }`.
- **Application & integration tests** reside in `src/app/` (e.g. `app_tests.rs`).
- **Regression tests**: Every bug fix or edge case **must** have an accompanying unit or integration test.
- **Pre-commit verification**: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` must pass cleanly with zero warnings.

---

## 📦 Dependency & Maintenance Protocol

- **Crate versions**: Pinned and audited in `Cargo.toml`.
- **Git Push Policy**: **NEVER** run `git push` or publish releases/tags without explicit prior confirmation and approval from the user.
- **Automated updates**: Patch and minor dependency PRs from Dependabot are automatically validated and merged via `.github/workflows/dependabot-auto-merge.yml`.
- **Major updates**: Major bumps (e.g. `similar` 3.x, `ratatui` 0.30+) require local verification, MSRV checks, and `cargo test` runs before merging.
- **Sponsorship & Community**: Maintained in `.github/FUNDING.yml` (`SoCkEt7`, `buymeacoffee.com/antoninnvh`).

---

## 💎 Project Philosophy & Documentation Rules

- **Pure Open Source Excellence**: Keep all documentation, commit messages, PR descriptions, and release notes strictly professional, accurate, and focused on terminal tooling, developer workflows, and Rust craft.
- **Zero AI / IA Mentions**: Never mention AI, LLMs, agents, or automated generators in public-facing documentation, READMEs, changelogs, or PR descriptions. Livediff is presented purely as a high-performance terminal utility for developers.
- **Performance First**: Maintain instantaneous startup, low memory footprint, non-blocking UI rendering, and efficient file-watching debouncing.
