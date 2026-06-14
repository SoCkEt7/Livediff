# Reddit playbook

## Rules

- Read each subreddit sidebar and recent moderator comments before posting.
- Do not ask for upvotes, stars, awards, or follows.
- Do not post the same title/body to multiple communities.
- Prefer technical context, screenshots/GIFs, and lessons learned.
- If a community has a weekly self-promotion thread, use that thread instead of making a standalone post.
- Reply with implementation details, tradeoffs, and limitations.

## Candidate communities

| Community | Angle | Risk |
| --- | --- | --- |
| r/rust | Rust TUI implementation, `ratatui`, file watcher, diff UX | Medium: must be technical |
| r/commandline | Useful CLI/TUI workflow | Medium: avoid sales tone |
| r/linux | Lightweight terminal workflow | Medium/high: self-promo rules vary |
| r/programming | Show project + implementation lessons | High: needs strong technical value |
| r/opensource | Public repo launch and contributor invitation | Medium |
| r/SideProject | Tool showcase and learning story | Low/medium |

## Draft 1 — r/rust

Title: `I built a Rust TUI that shows live file diffs while you work`

Body:

> I built Livediff, a small Rust terminal tool that watches a directory and shows file diffs as they happen.
>
> Repo: https://github.com/SoCkEt7/Livediff  
> Demo: https://socket7.github.io/Livediff/
>
> The stack is `ratatui` + `crossterm` for the interface, `notify` for filesystem events, `similar` for diffs, and `ignore` for `.gitignore` / custom filtering.
>
> The use case is keeping a live diff pane open while a generator, formatter, script, or refactor is changing files. It is not meant to replace Git; it is more of a “what changed right now?” terminal companion.
>
> The hardest part was keeping the UI quiet enough to leave open: event-driven redraws, ignore handling, and a file list that points to the most relevant changes.
>
> I would be interested in feedback from people building TUIs in Rust: what patterns have worked well for keeping terminal UIs responsive without making the codebase messy?

## Draft 2 — r/commandline

Title: `Livediff: a terminal UI for watching file diffs in real time`

Body:

> I made a CLI tool for a workflow I kept wanting: leave a pane open and see file diffs update as files change.
>
> `cargo install livediff`  
> Repo: https://github.com/SoCkEt7/Livediff  
> Demo: https://socket7.github.io/Livediff/
>
> It watches a directory, respects ignore patterns, and shows an interactive terminal diff view. I use it for generated files, formatting output, and refactors where I want immediate feedback without rerunning `git diff` constantly.
>
> Curious if other terminal-first people have similar workflows, or if there are existing tools you prefer for this.

## Draft 3 — r/opensource

Title: `Open-source Rust CLI: live terminal diffs while files change`

Body:

> I published Livediff, a Rust CLI/TUI that monitors a directory and shows real-time file diffs in the terminal.
>
> GitHub: https://github.com/SoCkEt7/Livediff  
> Demo: https://socket7.github.io/Livediff/
>
> It is aimed at developers who want a lightweight live view of changes from generators, formatters, scripts, or refactors. The project already has CI, issue templates, contributing docs, and dual MIT/Apache licensing.
>
> Useful feedback would be:
>
> - installation friction on Linux/macOS/Windows;
> - terminal rendering issues;
> - workflows where live diffs are useful;
> - missing ignore/filter behavior.

## Comment reply templates

### “Why not just git diff?”

`git diff` is still the source of truth. Livediff is for the moment before you manually inspect state: it stays open and updates while files are changing, so it works more like a live monitor than a final review command.

### “What is the resource usage?”

The design is event-driven: filesystem notifications trigger updates instead of constant polling. The README currently documents a low idle footprint; I am keeping that measurable rather than turning it into a broad performance claim.

### “Does it support ignore files?”

Yes. It uses ignore handling for `.gitignore` and custom patterns, so generated folders and build output can be filtered out.
