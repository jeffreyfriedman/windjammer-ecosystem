# wj-sql

## Purpose
Build parameterised SQL (select, insert, update, delete) without string-splicing values.

## Inputs / outputs
- In: table and column names, comparison operators from a fixed list, string values, a `Dialect`.
- Out: `Result<Query, string>` where `Query` is `{ sql, params }`.

## Layout
- `src/lib.wj` (`Select`, `Insert`, `Update`, `Delete`, `quote_name`, `Dialect`), `tests/sql_test.wj`.

## Invariants
- Values only ever appear in `params`; the SQL text contains names, operators and placeholders.
- A name that is not `[A-Za-z0-9_]+` (optionally one dotted pair) is an `Err`, reported at `build`.
- `UPDATE` and `DELETE` need a `WHERE` unless `build_all_rows` is called.
- Pure computation: `[capabilities]` is empty.
