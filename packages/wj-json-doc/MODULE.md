# wj-json-doc

## Purpose
Order-preserving JSON values with a parser and a writer whose output matches Python's `json.dumps` byte for byte, so repository tooling can read and rewrite files that other tools also write.

## Inputs / outputs
- In: JSON text, or values built with `object`, `with`, `push`, `from_*`.
- Out: `Json` values; text from `dumps` / `dumps_unicode`; `Err(String)` with the character offset on malformed input.

## Layout
- `src/lib.wj`: `Json` and `Member` types, builders and accessors, recursive-descent `Parser`, `float_repr`, `quote`, `dumps`.
- `tests/json_doc_test.wj`: float repr, escapes, layouts, duplicate keys, surrogate pairs, errors.

## Invariants
- Object members keep document order; `sort_keys` is applied only when writing.
- `float_repr` round-trips: shortest digits, exponent form outside 1e-4 <= |x| < 1e16, whole numbers end in `.0`.
- No dependencies beyond `std::strings`.
