# GitHub growth plan

## Current public metadata

- Repository: `SoCkEt7/Livediff`
- Visibility: public
- Current stars observed during setup: 9
- Current topics observed: `cli`, `developer-tools`, `diff`, `monitoring`, `productivity`, `rust`, `terminal`, `tui`
- Description observed: `Real-time file monitoring with beautiful, pulsing TUI diff visualization. Fast, lightweight (<10MB RAM), and built in Rust.`

## Recommended About description

`Rust TUI for live file diffs in the terminal.`

Alternative:

`Watch real-time file diffs in a lightweight Rust terminal UI.`

## Recommended topics

Keep current topics and add a few high-intent discovery terms:

```text
cli, command-line, developer-tools, diff, file-watcher, monitoring, productivity, rust, rust-cli, terminal, terminal-ui, tui
```

## README improvements

Already strong:

- GIF/demo above the fold.
- Browser showcase link.
- Clear install command.
- Feature list.
- Contributing and license sections.

Next improvements:

1. Add a short “When to use it” section above Features.
2. Add a “Why not just git diff?” section.
3. Add a compact commands table.
4. Add verified platform support once release artifacts are confirmed.
5. Add screenshots/GIF alt text that describes the workflow, not only the tool name.

## Suggested README insert

```markdown
## When to use Livediff

Use Livediff when you want to keep a live view of file changes while another process is running:

- code generators and template systems;
- formatters and migration scripts;
- refactors that touch many files;
- documentation or config changes;
- terminal-first workflows where a GUI diff app is too heavy.

Livediff does not replace Git. It complements `git diff` by showing changes as they happen.
```

## GitHub Discussions categories

If Discussions are enabled:

- Announcements
- Ideas
- Help
- Show your workflow

## Release post template

Title:

`Livediff v0.2.0 — live terminal diffs with smarter ignore controls`

Body:

```markdown
Livediff v0.2.0 improves the live terminal diff workflow for developers who want to keep a file-change monitor open while they work.

Highlights:

- interactive Rust TUI for real-time file diffs;
- `.gitignore` and custom ignore controls;
- lightweight event-driven monitoring;
- browser demo for quick evaluation.

Install:

```bash
cargo install livediff
```

Repository: https://github.com/SoCkEt7/Livediff  
Demo: https://socket7.github.io/Livediff/
```

## Issue ideas for contributors

- Add terminal compatibility notes for common terminals.
- Add a gallery of real workflows in docs.
- Improve install documentation per OS.
- Add snapshot tests for UI state rendering.
- Add examples for generated-file workflows.
