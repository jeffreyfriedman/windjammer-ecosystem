# wj-migrate

Ordered SQL migration helpers in pure Windjammer (domain layer) plus a `std::db` apply adapter.

## API

```windjammer
use wj_migrate
use wj_migrate::db_apply
use wj_migrate::db_status

let m = parse_filename("001_create_users.sql")?
let sorted = sort_migrations(items)
let left = pending(all, applied_versions)
validate_unique_versions(all)?
let ddl = schema_table_sql()
let bookkeeping = record_applied_sql(m, "2026-08-23T00:00:00Z")

// Apply pending `.sql` files from a directory (SQLite file, `:memory:`, or Postgres URL)
let report = db_apply::apply_url("postgres://localhost/mydb", "./fixtures")?

// List applied vs pending versions (read-only)
let status = db_status::status_url("postgres://localhost/mydb", "./fixtures")?
```

## Filename convention

`NNN_name.sql` — numeric version prefix, underscore, snake_case name, `.sql` suffix.

## Layout

```
src/
  lib.wj          # domain: parse, sort, pending, SQL helpers (declare `pub mod` at top!)
  db_apply.wj     # adapter: std::db + std::fs apply orchestration
  db_status.wj    # adapter: read-only applied/pending report
tests/
  migrate_test.wj
  apply_test.wj
  status_test.wj
fixtures/*.sql
scripts/smoke_postgres.sh
```

## How this package is tested

1. **Unit tests (`wj test`)** — domain SQL helpers + filename discovery + SQLite `:memory:` / file apply.
2. **Postgres smoke (`scripts/smoke_postgres.sh`)** — applies fixture SQL + `schema_migrations` bookkeeping against the local `platform-db` Docker Postgres (`wj_migrate_smoke`). Uses `docker exec` + container env (no host password required).

```bash
unset CARGO_TARGET_DIR
wj test
./scripts/smoke_postgres.sh
```

## Apply adapter

`db_apply::apply_url(url, migrations_dir)`:

1. Ensures `schema_migrations` table exists
2. Discovers `*.sql` files in `migrations_dir`
3. Validates unique version numbers
4. Applies pending migrations in order (SQL body + bookkeeping row)

Supports `std::db` URLs: Postgres (`postgres://…`), SQLite file path, or `:memory:`.

## Status adapter

`db_status::status_url(url, migrations_dir)` returns applied and pending version lists without mutating the database (except reading `schema_migrations` when present).

## License

MIT OR Apache-2.0
