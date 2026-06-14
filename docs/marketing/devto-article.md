---
title: "I built a live diff monitor for the terminal in Rust"
published: false
description: "A practical look at Livediff, a Rust TUI that watches file changes and shows real-time diffs in the terminal."
tags: rust, opensource, productivity, cli
canonical_url: https://github.com/SoCkEt7/Livediff
---

# I built a live diff monitor for the terminal in Rust

I often want to see what changes while a tool is running, not only after the fact.

`git diff` is great when you decide to inspect the current state. It is less convenient when a formatter, generator, script, or refactor is changing files and you want a live view beside your editor.

That is the workflow Livediff is built for.

GitHub: https://github.com/SoCkEt7/Livediff  
Demo: https://socket7.github.io/Livediff/

## What Livediff does

Livediff watches a directory and displays file changes in an interactive terminal UI.

```bash
cargo install livediff
livediff
```

You can point it at a specific folder:

```bash
livediff ./src
```

Or ignore noisy paths:

```bash
livediff ./src --ignore "*.tmp" --ignore "target/"
```

## Why a terminal UI?

I wanted something that could stay open next to an editor or test runner.

A terminal UI fits that workflow well:

- no context switch to a separate desktop app;
- low resource usage;
- fast startup;
- works naturally over SSH or inside terminal-heavy setups;
- easy to keep open while scripts run.

## The core design

Livediff combines a few pieces:

- filesystem notifications for real-time updates;
- diff computation to show what changed;
- ignore handling for `.gitignore` and custom patterns;
- a Rust TUI built with `ratatui` and `crossterm`;
- syntax highlighting for a more readable diff view.

The goal is not to replace Git.

The goal is to give you a live “what is changing right now?” pane while you work.

## Where it helps

The tool is useful when you are:

- tweaking a code generator;
- editing templates that produce files elsewhere;
- watching a refactor touch multiple files;
- comparing formatter output while tuning configuration;
- monitoring docs or config changes during a script run.

## What I learned building it

The interesting part was not only computing diffs. It was making the interface quiet enough to leave open.

A watcher can become noisy quickly. The UI has to answer a simple question: what changed, and where should I look first?

That pushed the project toward:

- smart filtering;
- compact file lists;
- readable visual hierarchy;
- event-driven redraws instead of constant polling.

## Try it

```bash
cargo install livediff
livediff
```

Repository: https://github.com/SoCkEt7/Livediff  
Browser demo: https://socket7.github.io/Livediff/

If you work in terminal-heavy environments and often need to watch generated or changing files, I would like to know what workflow you would test it on first.
