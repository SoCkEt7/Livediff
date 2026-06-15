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
