# wj-fuzzy

## Purpose
Edit distance, did-you-mean suggestions and subsequence scoring for pickers.

## Inputs / outputs
- In: text, candidate lists, a tolerance.
- Out: distances, percentages, ordered candidate lists, hint text, scores.

## Layout
- `src/lib.wj`, `tests/fuzzy_test.wj`, `tests/docs_test.wj`, `examples/command_hints.wj`.

## Invariants
- Distance is symmetric, zero only for equal text, and obeys the triangle inequality.
- Ordering is stable: equal scores keep input order.
- No I/O: `[capabilities]` grants only the docs test read of the README and example.
