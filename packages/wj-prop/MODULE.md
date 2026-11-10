# wj-prop

## Purpose
Random-input testing of properties with reproducible seeds and shrinking.

## Inputs / outputs
- In: a `Config`, input bounds and a property closure.
- Out: a `Report` (`ok`, cases run, seed, failing case, first and shrunk counterexample).

## Layout
- `src/rng.wj` (PCG32), `src/check.wj` (generators, runner, shrinking); `tests/prop_test.wj` (PCG reference vectors, shrink results).

## Invariants
- Pure: no I/O, no capabilities, no clock. The same seed gives the same cases.
- Integer arithmetic on the generator state wraps on purpose (`wrapping_mul`, `wrapping_add` on `u64`); everything else is overflow-checked.
