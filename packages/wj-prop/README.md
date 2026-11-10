# wj-prop

Property-based testing with shrinking. State a property, let `wj-prop` try it on random inputs, and get the smallest failing input back together with the seed and case number that reproduce it.

```windjammer
use wj_prop::check::assert_holds
use wj_prop::check::check_ints
use wj_prop::check::config

@test
fn test_sorting_is_idempotent() {
    assert_holds(check_ints(config(), 20, -100, 100, |v| sorted(sorted(v)) == sorted(v)))
}
```

A failure reads `property failed after 7 cases (seed 42, case 6)`, then the first failing input and the shrunk one.

## API

- `rng`: `new_rng(seed, stream)` is PCG32 (the reference algorithm, outputs checked against the published vectors); `next_u32`, `below(n)` without modulo bias, `range(lo, hi)`, `coin`, `unit`
- `check`: `check_int`, `check_int_pair`, `check_ints`, `check_string`, each taking a `Config` (`config()`, `with_cases`, `with_seed`) and a closure; `describe`, `assert_holds`

Case `i` uses stream `i` of the run's seed, so every case replays on its own. Shrinking drops elements, moves integers toward zero (or the lower bound) and replaces characters with the simplest one in the alphabet; it is bounded by `max_shrinks`.

## Build / test

```bash
cd packages/wj-prop
wj test --interpret
wj test
```
