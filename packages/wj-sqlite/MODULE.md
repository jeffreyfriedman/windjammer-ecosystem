# wj-sqlite

## Purpose
Read tables out of a SQLite database file without a SQLite engine.

## Inputs / outputs
- In: the bytes of a database file (or its path).
- Out: tables, columns, rows of `Value`; failures are `Err(message)`, never a panic on a corrupt page reference.

## Layout
- `src/format.wj` (varints, serial types, records), `src/database.wj` (header, pages, b-tree walk, schema); `tests/fixture_test.wj`, `tests/differential_test.wj`.

## Invariants
- Read-only. Capabilities: `fs.read` for `read_file`; the tests write scratch databases under `/tmp`.
- Rows come back in rowid order; page numbers are range-checked and the b-tree depth is bounded.
