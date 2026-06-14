# Livediff use cases

Livediff is most useful when another process changes files and you want to see the diff live instead of repeatedly running `git diff`.

## Watch generated files

Terminal 1:

```bash
cargo run --bin codegen
```

Terminal 2:

```bash
livediff ./generated --ignore "*.tmp"
```

Use this for template systems, SDK generation, static site output, or any workflow that rewrites many files.

## Inspect migration output

```bash
livediff ./migrations
```

Run your migration generator in another terminal and keep Livediff open to see exactly which SQL files change.

## Follow formatter or codemod changes

```bash
livediff ./src --ignore "target/"
```

Useful before accepting a broad formatting, lint-fix, or codemod pass.

## Monitor docs and config generation

```bash
livediff ./docs
livediff . --ignore "target/" --ignore "node_modules/"
```

Good for documentation generators, config rewrites, or build tools that update lockfiles and manifests.

## Compare with `git diff`

`git diff` answers: "what changed since the last commit?"

Livediff answers: "what is changing right now?"

Use both: keep Livediff open during the edit loop, then use `git diff` for final review before committing.
