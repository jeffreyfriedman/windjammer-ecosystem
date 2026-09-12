# wj-migrate-cli

Thin CLI for SQL migrations — hexagonal app dogfooding `wj-migrate`.

```
wj-migrate-cli apply --url postgres://localhost/mydb --dir ./migrations
wj-migrate-cli status --url :memory: ./fixtures
wj-migrate-cli list --dir ./migrations
wj-migrate-cli validate --dir ./migrations
```

`list` discovers and prints migration files from a directory — no database URL required.

`validate` checks migration filenames and duplicate version numbers — no database URL required.

## Packages

| Layer | Module |
|---|---|
| **wj-migrate** | `db_apply::apply_url`, `db_status::status_url`, `validate_unique_versions` |
| **wj-cli-args** | intended; blocked by cross-crate `Vec` borrow repro — local argv parsing until green |

## Layout

```
src/domain/     # parse_command, env config, stdout formatting
src/adapters/   # argv, db apply/status delegation
src/main.wj
tests/
```

## Environment

| Variable | Default | Purpose |
|---|---|---|
| `MIGRATE_URL` | (none) | Default database URL when `--url` omitted |
| `DATABASE_URL` | (none) | Fallback for `MIGRATE_URL` |
| `MIGRATIONS_DIR` | `./migrations` | Default migrations directory when `--dir` omitted |

## Build / test

Path dependency must point at `packages/wj-migrate/build`. Pre-build once:

```bash
unset CARGO_TARGET_DIR
export WJ=~/.cargo/bin/wj

cd packages/wj-migrate && $WJ build src

cd apps/wj-migrate-cli
$WJ test
$WJ build --release src
```

`wj test` — 21 unit tests (commands, format, list, validate, apply/status integration).

## License

MIT OR Apache-2.0
