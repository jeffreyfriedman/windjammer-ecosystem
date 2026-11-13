# wj-money

## Purpose
Currency-tagged integer money: parsing, formatting, checked arithmetic, basis-point rates and exact allocation.

## Inputs / outputs
- In: minor units and an ISO 4217 code, or decimal text.
- Out: `Money { minor, currency }`, display text, or a readable error.

## Layout
- `src/lib.wj`, `tests/money_test.wj`, `tests/docs_test.wj`, `examples/invoice.wj`.

## Invariants
- Allocation conserves the total for every amount and ratio set.
- No float anywhere; overflow and currency mismatch are `Err`.
- No I/O: `[capabilities]` grants only the docs test read of the README and example.
