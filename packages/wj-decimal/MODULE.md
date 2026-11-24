# wj-decimal

## Purpose
Exact decimal arithmetic for money: parsing, formatting, rounding modes and proportional allocation.

## Inputs / outputs
- In: text such as `-12.50`, or `(units, scale)` pairs.
- Out: `Decimal { units, scale }`; failures (overflow, bad text, bad scale, division by zero) are `Err(message)`.

## Layout
- `src/lib.wj`: the whole library. `tests/`: behaviour tests and a Python `decimal` oracle table.

## Invariants
- Pure: no I/O, no capabilities. No floats.
- Overflow never wraps; scale is 0..18; at most 18 digits are parsed.
- Equality and ordering compare amounts, not scales.
