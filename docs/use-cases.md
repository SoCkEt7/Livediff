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

### Generated client or SDK review

1. Start Livediff on the generated output directory:

    ```bash
    livediff ./generated --ignore "*.tmp" --ignore "*.log"
    ```

2. Run the generator in another terminal.
3. Watch for unexpected file churn, deleted files, or broad rewrites before accepting the update.
4. Finish with `git diff --stat` and your normal test command before committing.

This works well when an API schema, parser grammar, or template bundle rewrites many files at once.

## Inspect migration output

```bash
livediff ./migrations
```

Run your migration generator in another terminal and keep Livediff open to see exactly which SQL files change.

### Database migration review

1. Open the migrations directory:

    ```bash
    livediff ./migrations
    ```

2. Generate the migration from your framework or migration tool.
3. Check whether the live diff contains only the expected `CREATE`, `ALTER`, or index changes.
4. If the migration rewrites older files, stop and inspect before applying it to a shared environment.

Livediff is especially useful here because migration tools often touch files quickly and then exit.

## Follow formatter or codemod changes

```bash
livediff ./src --ignore "target/"
```

Useful before accepting a broad formatting, lint-fix, or codemod pass.

### Large refactor or codemod pass

1. Watch the source tree with build outputs ignored:

    ```bash
    livediff ./src --ignore "target/" --ignore "dist/"
    ```

2. Run the formatter, lint fix, or codemod in a second terminal.
3. Use the file list to spot changes outside the intended package or module.
4. Review surprising files immediately instead of waiting until the final `git diff`.

This keeps broad mechanical edits easier to supervise.

## Monitor docs and config generation

```bash
livediff ./docs
livediff . --ignore "target/" --ignore "node_modules/"
```

Good for documentation generators, config rewrites, or build tools that update lockfiles and manifests.

### Documentation or config rebuild

1. Watch the repository root with generated dependency directories ignored:

    ```bash
    livediff . --ignore "target/" --ignore "node_modules/" --ignore ".git/"
    ```

2. Run the docs builder, static site generator, or config update command.
3. Confirm the live diff matches the intended docs, lockfile, or manifest changes.
4. If many unrelated files change, narrow the watch path and rerun the command.

This helps keep generated documentation and config updates reviewable.

## Monorepo package update

Watch only the package you are changing instead of the full repository:

```bash
livediff ./packages/payments --ignore "node_modules/" --ignore "dist/"
```

Then run the generator, formatter, or tests for that package in another terminal. If files outside the watched package should also change, open a second Livediff session for that path.

## Release preparation

Before tagging a release, keep Livediff open while updating version files, changelog entries, and packaging metadata:

```bash
livediff . --ignore "target/" --ignore ".git/"
```

Use the live view to confirm that only release-related files are changing, then run the full test suite and inspect `git diff` before tagging.

## Compare with `git diff`

`git diff` answers: "what changed since the last commit?"

Livediff answers: "what is changing right now?"

Use both: keep Livediff open during the edit loop, then use `git diff` for final review before committing.


## OpenAPI SDK generation (openapi-generator)

Watch the generated client while your OpenAPI spec evolves.

Terminal 1 — regenerate the TypeScript client whenever the spec changes:

```bash
npx @openapitools/openapi-generator-cli generate \
  -i openapi.yaml \
  -g typescript-fetch \
  -o ./sdk/generated
```

Terminal 2 — watch the output directory with Livediff:

```bash
livediff ./sdk/generated --ignore "*.tmp"
```

Spot unexpected endpoint renames, removed types, or stale `*.ts` files before they break downstream consumers.

## Prisma migration (SQL diff review)

Watch the `prisma/migrations/` folder while Prisma generates a new migration file.

Terminal 1 — create the migration:

```bash
npx prisma migrate dev --name add_user_roles
```

Terminal 2 — keep Livediff open on the migrations directory:

```bash
livediff ./prisma/migrations --ignore "migration_lock.toml"
```

Confirm the generated SQL contains only the expected `CREATE TABLE`, `ALTER TABLE`, or index statements before applying to a shared database.

## Prettier formatting pass

Watch the source tree while Prettier rewrites files in place.

Terminal 1 — run Prettier on the entire project:

```bash
npx prettier --write "src/**/*.{ts,tsx,js,jsx,css}"
```

Terminal 2 — watch for changes as they land:

```bash
livediff ./src --ignore "node_modules/" --ignore "dist/"
```

Use the live file list to catch any files outside `src/` that change unexpectedly, or to spot files that Prettier is rewriting in repeated passes.
