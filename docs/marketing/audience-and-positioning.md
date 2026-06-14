# Audience and positioning

## One-line positioning

Livediff is a lightweight Rust TUI that shows live file diffs in your terminal while you work.

## Longer positioning

Run Livediff in any project folder and it watches file changes in real time, giving you an interactive terminal view of what changed without constantly rerunning `git diff` or opening a heavy GUI tool.

## Best-fit use cases

1. Watching generated files while tweaking templates, prompts, build scripts, or codegen.
2. Keeping a live diff pane open during refactors.
3. Monitoring configuration or documentation edits across a project.
4. Reviewing formatter/build output as it lands on disk.
5. Learning how polished Rust TUIs can be built with `ratatui`.

## Audience segments

### Rust CLI/TUI developers
- Hook: polished Rust TUI with filesystem watcher + diff visualization.
- Best channels: r/rust, dev.to `#rust`, GitHub topics.

### Terminal-first developers
- Hook: live diff monitor without leaving the terminal.
- Best channels: r/commandline, r/linux, dev.to `#productivity`.

### Tooling builders
- Hook: watch generated outputs and refactors in real time.
- Best channels: r/programming, Hacker News Show HN, GitHub.

## Message pillars

- Lightweight: single Rust binary, no Electron runtime.
- Immediate: event-driven updates as files change.
- Visual: interactive terminal UI instead of noisy logs.
- Practical: works in existing folders and respects ignore patterns.

## Avoid

- Claims about adoption unless verified from current GitHub/crates data.
- Asking for stars.
- Posting the same generic message everywhere.
- Overpromising support for workflows not tested locally.
