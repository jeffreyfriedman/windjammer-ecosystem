# wj-cron

Five-field cron expression parsing and matching for Windjammer.

## API

| Function | Purpose |
|---|---|
| `parse_cron(expr)` | Parse `"m h dom mon dow"` or named aliases → `CronExpr` (retains `source`) |
| `matches_field(field, value)` | Match one field (`*`, `*/N`, `a,b`, `a-b`, `a-b/N`, exact) |
| `matches_cron(expr, …)` | Match all five fields against a civil time |
| `next_run(expr, start, max_minutes_ahead)` | First matching minute after `start`, or `None` |

Day-of-week uses `0` = Sunday … `6` = Saturday (standard cron).

### Named aliases

| Alias | Expands to |
|---|---|
| `@hourly` | `0 * * * *` |
| `@daily` | `0 0 * * *` |
| `@weekly` | `0 0 * * 0` |
| `@monthly` | `0 0 1 * *` |
| `@yearly` / `@annually` | `0 0 1 1 *` |

Unknown aliases (e.g. `@reboot`) return `Err`.

## Example

```windjammer
use crate::parse_cron
use crate::matches_cron
use crate::next_run
use crate::CronDateTime

match parse_cron("@hourly") {
    Ok(expr) => {
        assert(matches_cron(expr, 0, 9, 1, 1, 3), "top of hour")
    },
    Err(_e) => {},
}

match parse_cron("10-20/5 * * * *") {
    Ok(expr) => {
        assert(matches_cron(expr, 15, 0, 1, 1, 0), "range/step")
    },
    Err(_e) => {},
}
```

## Test

```bash
unset CARGO_TARGET_DIR
CARGO_TARGET_DIR=build/target wj test
```
