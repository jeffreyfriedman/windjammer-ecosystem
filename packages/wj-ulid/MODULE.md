# wj-ulid

## Purpose
Time-sortable 128-bit identifiers in Crockford base32, with monotonic generation.

## Inputs / outputs
- In: a millisecond timestamp and 10 random bytes (or nothing, for `generate()`).
- Out: 26-character IDs, decoded bytes, timestamps.

## Layout
- `src/lib.wj`, `tests/ulid_test.wj`, `tests/docs_test.wj`, `examples/order_ids.wj`.

## Invariants
- `decode(encode(t, r))` returns `t` and `r` unchanged.
- Text order equals time order, and `monotonic` never returns an ID that sorts before its predecessor.
- Only `generate()` touches the clock and randomness.
