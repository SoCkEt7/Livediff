# Developer audience context — Livediff

## Project
Livediff is a Rust terminal tool that watches a directory and shows beautiful real-time diffs as files change.

Repository: https://github.com/SoCkEt7/Livediff
Demo: https://socket7.github.io/Livediff/
Install: `cargo install livediff`

## Primary users
- Developers who want a live terminal companion while editing generated or fast-changing files.
- CLI/TUI fans who prefer a lightweight Rust binary over Electron dashboards.
- Rust developers interested in `ratatui`, filesystem watchers, and polished terminal UX.
- Developers comparing outputs from code generators, scripts, formatters, build tools, or refactors.

## Pain points
- `git diff` is useful after the fact, but not as a live visual monitor.
- File watchers often print noisy logs instead of showing exactly what changed.
- GUI diff tools are heavy for quick terminal workflows.
- Generated-file workflows need immediate feedback without constant command reruns.

## Positioning
A lightweight real-time diff monitor for the terminal: run it in a project folder and see file changes as they happen.

## Differentiators
- Native Rust CLI/TUI.
- Event-driven filesystem monitoring.
- Interactive terminal interface with syntax-highlighted diff views.
- Smart ignore support for `.gitignore` and custom patterns.
- Browser demo for zero-install evaluation.

## Discovery channels
- GitHub README, topics, releases, and demo page.
- dev.to / Hashnode technical article.
- Reddit communities where self-promotion is allowed only with useful context.
- Rust/TUI/devtool communities through helpful technical posts.

## Voice rules
- Direct, practical, developer-to-developer.
- No hype, no fake social proof, no fabricated metrics.
- Never mention how content was produced.
- Lead with the workflow problem, not with promotion.
