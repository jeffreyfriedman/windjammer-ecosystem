# wj-pagination

## Purpose
Offset and keyset pagination arithmetic, page descriptions, page-number strips and `Link` headers.

## Inputs / outputs
- In: raw `page` / `per_page` values and a total item count.
- Out: `PageRequest`, `Page`, `Vec<int>` strips (0 is a gap), header text.

## Layout
- `src/lib.wj`, `tests/pagination_test.wj`, `tests/docs_test.wj`, `examples/list_items.wj`.

## Invariants
- Pages partition the items: walking every page visits each item exactly once.
- A described page is always renderable: clamped into range, never negative.
- No I/O: `[capabilities]` is empty.
