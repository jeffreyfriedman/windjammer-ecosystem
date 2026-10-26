# wj-table

## Purpose
Render rows of text as aligned plain text or markdown, and format numbers with a fixed number of decimals.

## Inputs / outputs
- In: header and cell strings, the right-aligned column indices.
- Out: a single `String` (lines joined with `\n`, no trailing newline).

## Layout
- `src/lib.wj`: `Table`, builders, `markdown`, `plain`, padding and number helpers.
- `tests/table_test.wj`: layouts and Python-formatted number expectations.

## Invariants
- Widths count characters, not bytes.
- `fixed` rounds the exact binary value, so it agrees with Python's formatting (`2.5` to 0 places is `2`).
