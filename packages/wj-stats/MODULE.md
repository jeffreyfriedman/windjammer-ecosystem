# wj-stats

## Purpose
Mean, median, extremes and standard deviations of benchmark-sized sample sets.

## Inputs / outputs
- In: `Vec<f64>` samples.
- Out: `f64`; an empty sample set yields `0.0`.

## Layout
- `src/lib.wj`: the functions. `tests/stats_test.wj`: values checked against Python's `statistics`.

## Invariants
- Inputs are never mutated (`sorted` and `median` work on copies).
- Pure: no I/O, no dependencies beyond `std::cmp`.
