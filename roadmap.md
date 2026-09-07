# Livediff — Project Roadmap & Technical Vision 👁️

> **"Live terminal diffs while files change."**
> A lightweight, terminal-first Rust TUI companion to `git diff` for generators, refactors, migrations, formatters, and real-time development workflows.

---

## 1. Executive Summary & Context

Livediff was built to solve a specific developer pain point: `git diff` is post-hoc, while modern development involves continuous background tooling (code generators, codemods, formatters, build pipelines, compilers). Livediff provides immediate visual feedback in the terminal as filesystem mutations occur.

This document consolidates:
- **Current project state & completed initiatives** (Funding, automated dependency pipelines, issue triage, BMC setup).
- **Deep repository health & triage findings**.
- **Multi-phase strategic roadmap (v0.2.0 → v1.0.0+)**.
- **Community, monetization, and distribution strategy**.

---

## 2. Recent Milestones & Completed Work

### 🎯 Funding & Sponsorship Activation
- [x] **GitHub Funding Manifest**: Created `.github/FUNDING.yml` configured for `github: [SoCkEt7]`, `buy_me_a_coffee: antoninnvh`, and custom sponsorship endpoints.
- [x] **README Sponsorship Badge**: Integrated official GitHub Sponsors badge (`img.shields.io/badge/donate-GitHub%20Sponsors-ee4aaa`) and interactive Buy Me a Coffee button linking to `https://www.buymeacoffee.com/antoninnvh`.
- [x] **Creator Profile Audit**: Audited `https://buymeacoffee.com/antoninnvh` and GitHub Sponsors presence via Chrome CDP (`browser-use`).

### 🤖 Automation & CI/CD Hardening
- [x] **Dependabot Auto-Merge Workflow**: Implemented `.github/workflows/dependabot-auto-merge.yml` utilizing `dependabot/fetch-metadata@v3` to automatically approve and squash-merge patch/minor updates while isolating major bumps for manual verification.
- [x] **Dependency Backlog Resolution**:
  - `#25` `serde` (1.0.228 → 1.0.229) — Merged.
  - `#31` `clap` (4.6.4 → 4.6.6) — Merged.
  - `#33` `globset` (0.4.19 → 0.4.20) — Merged.
  - `#34` `ignore` (0.4.31 → 0.4.33) — Merged.
  - `#35` `similar` (2.7.0 → 3.2.0 Major) — Verified MSRV compatibility & diff API stability, merged.
  - `#36` `tui-big-text` (0.8.8 → 0.8.9) — Merged.
- [x] **Formatting & Lint Integrity**: Fixed `.editorconfig` shell script indentation and verified `cargo fmt`, `cargo clippy -D warnings`, and `cargo test`.

---

## 3. Repository Health & Issue Audit Summary

| Metric | Status | Notes |
|---|---|---|
| **Open Issues** | `0` | Zero unresolved bugs or feature requests in queue. |
| **Closed Issues** | `6` | 100% triage rate on reported bugs (#16, #17), docs (#5, #6, #7), survey (#11). |
| **Pull Requests** | Clean | All open Dependabot PRs merged; automated pipeline active. |
| **CI Status** | 🟢 Passing | GitHub Actions workflow executing full Rust matrix. |
| **MSRV** | Rust 1.85+ | Aligned with Rust 2024 edition and modern crate ecosystem. |

### Historical Triage Retrospective
- **Bug #16 (Fedora TUI freeze)**: Resolved terminal raw mode / alternative screen edge case.
- **Bug #17 (`.gitignore` bypass via `--no-ignore-parent`)**: Hardened ignore hierarchy walker.
- **Docs #5, #6, #7**: Added interactive browser showcase, GIF demo, and `docs/use-cases.md`.

---

## 4. Strategic Product Roadmap

```text
v0.1.x (Current)            v0.2.0 (Polish & Core)         v0.3.0 (Advanced TUI)           v1.0.0 (Production Stable)
─────────────────          ──────────────────────         ───────────────────────         ──────────────────────────
• Crates.io release        • Homebrew / Binstall          • Split / Unified view modes    • Plugin / Hook Architecture
• Basic TUI & notify       • Multi-thread notify engine   • Syntax theme engine           • LSP / Tree-sitter diffing
• Funding & Auto-merge     • Custom ignore regex rules    • Fuzzy search & file filter    • Full cross-platform parity
```

---

### Phase 1: Distribution & Developer Experience (v0.2.0)
*Target: Frictionless onboarding across all OS environments and packaging ecosystems.*

- [ ] **Homebrew Packaging (Issue #5 follow-up)**
  - Create `homebrew-tap` repository under `SoCkEt7`.
  - Add automated GitHub Actions formula generation on tag release.
  - Provide `brew install socket7/tap/livediff`.
- [ ] **Cargo Binstall & Binary Artifacts**
  - Add `cargo-binstall` metadata to `Cargo.toml`.
  - Build universal macOS binaries (`x86_64` + `aarch64` / Apple Silicon).
  - Produce static Linux musl releases and Windows zip bundles.
- [ ] **AUR (Arch User Repository) & Nix Flake**
  - Publish `livediff-bin` to Arch AUR.
  - Add root `flake.nix` for zero-install Nix environments (`nix run github:SoCkEt7/Livediff`).
- [ ] **Shell Completions**
  - Generate Bash, Zsh, and Fish completions during build via `clap_complete`.

---

### Phase 2: Core Engine & Performance (v0.2.x - v0.3.0)
*Target: Maximum throughput with zero CPU idle load on large repositories (100k+ files).*

- [ ] **Adaptive Filesystem Debouncing**
  - Introduce configurable debounce window (`--debounce-ms <DEFAULT: 50ms>`) for rapid generator batches.
  - Implement bulk change grouping to eliminate intermediate redraw flicker.
- [ ] **Multi-threaded Ignore & Discovery Engine**
  - Decouple filesystem traversal from the TUI event loop using `rayon` or bounded `crossbeam` channels.
  - Support nested `.livediffignore` files alongside standard `.gitignore`.
- [ ] **Memory Footprint Optimization**
  - Stream diff computations directly to terminal spans without retaining unbounded buffer history.
  - Add LRU cache for diff slices on inactive tabs.

---

### Phase 3: Visual UX & Advanced TUI Features (v0.3.0 - v0.4.0)
*Target: Rich, intuitive terminal diff inspection for complex multi-file edits.*

- [ ] **Side-by-Side (Split) View Mode**
  - Toggle between Unified Diff (current) and Split Diff (`Tab` or `v` keybind).
  - Synchronized vertical and horizontal scrolling across panes.
- [ ] **Syntax Highlighting & Color Themes**
  - Integrate lightweight `syntect` or Tree-sitter highlighting for modified lines.
  - Support popular color themes: Catppuccin, Tokyo Night, Gruvbox, Nord, Solarized.
  - Respect `NO_COLOR` and `CLICOLOR_FORCE` environment variables.
- [ ] **Fuzzy File Filtering & Jump-to-File**
  - Interactive fuzzy search (`/` search prompt) to quickly filter modified files.
  - Quick navigation shortcuts (`j`/`k`, `g`/`G`, `n`/`N`).
- [ ] **Diff Export & Snapshotting**
  - Hotkey (`s` or `--save-patch`) to dump current diff snapshot to standard `.patch` or HTML format.

---

### Phase 4: Integrations & Ecosystem (v1.0.0+)
*Target: Deep integration into developer toolchains, automated workflows, and editor environments.*

- [ ] **Semantic / AST-Aware Diffing**
  - Optional Tree-sitter diffing mode to highlight meaningful semantic changes vs whitespace/formatting.
- [ ] **Coding Companion Mode**
  - Headless IPC/JSON mode (`livediff --json-stream`) allowing external tools and pipelines to observe mutations in real time.
  - Visual change HUD for background workflows.
- [ ] **Pre-commit & CI Watcher Modes**
  - `--fail-on-diff` or `--timeout` flags for ephemeral CI verification.
  - Integration with Git hook systems (Husky, Lefthook).

---

## 5. Community, Monetization & Outreach Strategy

### ☕ Buy Me a Coffee Profile Optimization (Antonin — `antoninnvh`)

Complete optimization roadmap for the Buy Me a Coffee presence (`https://buymeacoffee.com/antoninnvh`), executed and verified via browser inspection:

#### 1. Identity & Visual Branding
- [ ] **Profile Header & Tagline**
  - **Display Name**: Antonin
  - **URL Slug**: `buymeacoffee.com/antoninnvh`
  - **Headline**: *"Building high-performance, terminal-first developer tools in Rust."*
  - **Avatar**: High-contrast, clean developer portrait matching GitHub (`@SoCkEt7`).
  - **Cover Image**: Custom 1200×300 banner featuring terminal dark aesthetics, live diff preview snippet, and clean Rust typography.

#### 2. "About Me" Storytelling & Conversion Copy
- [ ] **Structured Bio & Mission Statement**
  - **The Hook**: Explain why developer tools matter — saving developer seconds across millions of daily edits.
  - **Maintained Projects**:
    - **Livediff**: Real-time terminal file diff engine for refactors, migrations, and formatters.
    - **Hydra Tools**: Modular infrastructure and developer automation utilities.
  - **Transparency Promise**: Clear breakdown of where sponsorships go:
    - Cross-platform CI runners (macOS Apple Silicon & Windows test hardware).
    - Domain and static documentation hosting.
    - Continuous maintenance and zero-delay dependency updates.

#### 3. Support Tiers & Contribution Structure
- [ ] **One-Time Support Options**:
  - ☕ **1 Coffee ($3)**: Quick high-five for saving debugging time.
  - ☕☕☕ **3 Coffees ($10)**: Fuel for a focused coding session or bugfix.
  - 🍕 **Lunch ($25)**: Significant milestone celebration & contributor shoutout.
- [ ] **Monthly Membership Tiers**:
  - **Supporter ($3/month)**: Backer name listed in `README.md` and release notes.
  - **Power Backer ($10/month)**: Priority triage on feature requests, early previews of new CLI utilities.
  - **Ecosystem Sponsor ($50/month)**: Company/Individual logo featured prominently in the repository README and web demo.

#### 4. Interactive Page Modules & Goals
- [ ] **Active Funding Goal**
  - **Objective**: "Dedicated macOS M-series CI test runner for Livediff release builds"
  - **Target**: $100 / month
  - **Progress Visibility**: Public progress bar on the profile frontpage.
- [ ] **Supporter Updates & Posts**
  - Publish quarterly "State of Livediff" changelog notes directly to BMC backers.
  - Pin the latest release link (`v0.1.x`) and the interactive web showcase (`socket7.github.io/Livediff`).

#### 5. Browser Automation & Verification Protocol
- [ ] **Studio Dashboard Configuration (`https://studio.buymeacoffee.com/dashboard`)**
  - Use browser navigation to inspect Profile, Membership, Goal, and Embed settings.
  - Verify live rendering on `https://buymeacoffee.com/antoninnvh` in both desktop (1920×1080) and mobile (375×812) viewports.
  - Validate embedded widget and button links from GitHub repository README.

---

### 🌟 Community Growth Action Items
1. **Showcase Article**: Write and publish a technical breakdown on Dev.to and Hacker News (*"Why we built Livediff: Live diffing while code generators run"*).
2. **Terminal Showcase Recording**: Re-record crisp terminal demo via `vhs` (`demo.tape`) showcasing side-by-side mode.
3. **Rust Ecosystem Submissions**: Submit to *This Week in Rust* (Crate of the Week) and Reddit `/r/rust`.

---

## 6. Architecture Overview

```
livediff/
├── src/
│   ├── adapters/          # Inbound & outbound ports
│   │   ├── cli/           # CLI argument parsing (clap)
│   │   ├── watcher/       # Filesystem event listener (notify)
│   │   └── ui/            # Terminal interface & widgets (ratatui + crossterm)
│   ├── domain/            # Pure business logic
│   │   ├── diff/          # Diff computation & character highlight (similar)
│   │   ├── ignore/        # Ignore hierarchy & glob matching (ignore, globset)
│   │   └── session/       # Active file states & history buffers
│   └── infrastructure/    # Side-effect implementations
│       ├── terminal/      # Raw mode & alternate screen management
│       └── logging/       # Tracing & diagnostic logs
└── docs/                  # Documentation & use cases
```

---

*Document maintained automatically. Last updated: September 2026.*
