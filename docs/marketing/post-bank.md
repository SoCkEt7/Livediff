# Post bank

## Short post — terminal workflow

I built Livediff for the moment when `git diff` is not quite enough yet: files are still changing, and you want a live view in the terminal.

It watches a folder and shows real-time diffs in a Rust TUI.

Repo: https://github.com/SoCkEt7/Livediff
Demo: https://socket7.github.io/Livediff/

## Short post — Rust angle

Livediff is a Rust TUI built with `ratatui`, `crossterm`, `notify`, `similar`, and `ignore`.

The goal: keep a live diff pane open while generators, formatters, scripts, or refactors change files.

https://github.com/SoCkEt7/Livediff

## Short post — developer tools angle

If you often run tools that rewrite files, a live diff pane is surprisingly useful.

Livediff monitors a directory and updates an interactive terminal diff view as files change.

Install: `cargo install livediff`
Repo: https://github.com/SoCkEt7/Livediff

## Profile CTA

Building lightweight developer tools for terminal-first workflows. Current project: Livediff, a Rust TUI for live file diffs.

## Gentle CTA options

- If this matches a workflow you have, I would like to hear where it fits or fails.
- Feedback on terminal rendering and install friction is especially useful.
- I am collecting real workflows where live diffs beat a manual `git diff` loop.

## Replies

### Install question

The shortest path is:

```bash
cargo install livediff
livediff
```

There are also GitHub releases for prebuilt binaries when available: https://github.com/SoCkEt7/Livediff/releases

### Workflow question

The strongest use case is when another process is changing files while you work: codegen, templates, formatters, migration scripts, or large refactors.

### Comparison question

I still use Git for final review. Livediff is a live monitor: it shows changes as they happen so you can catch unexpected output earlier.
