# Chained `if` clamp mixes widths (not yet reduced)

`packages/wj-cli-ui/src/progress.wj` at commit `be71c1c` (before it was rewritten with a `clamp` helper) had:

```windjammer
let t = if total <= 0 { 1 } else { total }
let d = if done < 0 { 0 } else if done > t { t } else { done }
let filled = (d * width) / t
```

`wj test` stopped with E0308: the generated Rust has `if done > (t as i64) { t } else { done as usize }`, so one
branch is `i64` and the other `usize`. A smaller program with the same three lines compiles, so the trigger is
something else in the function (the result also feeds a helper taking `int`, and a `format` of `pct`).
Check out `be71c1c` and run `cd packages/wj-cli-ui && wj test` to see it.
