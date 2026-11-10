# Decision: no `wj-sqlite` package (use `std::db`)

**Question.** Should the ecosystem ship a `wj-sqlite` package, either Windjammer-native or wrapping `rusqlite` through a
generated signature surface now that Rust crate interop exists?

**Decision.** No. `std::db` already is the SQLite binding, in both modes:

- `db.connect(":memory:")` or `db.connect("file.db")` opens SQLite; `postgres://` URLs open Postgres. One API: `query`,
  `execute`, `begin` / `commit` / `rollback`, `transaction`, typed row getters.
- It works interpreted and compiled. A program that creates a table, inserts and selects with `?` parameters printed the same
  rows under `wj run --interpret` and `wj run --compiled` at `6ed79d644`.
- It is covered by the capability model (`fs.read` / `fs.write` for the database file), the effect port (`--record`,
  `--replay`, `--simulate`) and `wj check`'s effect listing. A `rusqlite` wrapper would add a second SQLite engine and an
  `ffi(rusqlite)` effect that bypasses those, for no new capability.
- A native SQLite engine (file format, B-tree, SQL planner) is a multi-year project with an obvious existing answer; the
  ambition ladder would start with a read-only file-format reader (page and B-tree walker for `sqlite_master` and table
  scans), proven against files written by SQLite itself, before any writer or SQL layer.

**What to build instead.** Helpers belong next to the code that uses them: `wj-migrate` already plans and applies SQL
migrations over `std::db`. Typed row mapping with `@json`-style derives is a candidate for `std::db` itself.

**Revisit when** `std::db` lacks a feature a project needs (user-defined functions, backup API, extensions): then wrap
`rusqlite` through `wj rust-surface` for that feature only, behind a package that declares `ffi(rusqlite)`.

**Update (2026-11-10).** The first rung of the native ladder now exists as `packages/wj-sqlite`: a read-only file-format
reader (header, `sqlite_master`, table b-trees, overflow chains, records), proven against Python's `sqlite3` and against
SQLite itself through `std::db`. Queries, writes and transactions stay with `std::db`. Next rungs, in order: index b-trees
(so `WITHOUT ROWID` tables and index-only scans work), then a page-level writer for appending rows, then SQL.
