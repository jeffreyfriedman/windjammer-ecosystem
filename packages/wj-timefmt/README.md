# wj-timefmt

RFC3339 parse/format plus clock helpers over `std::time`.

Supports:

- Zulu (`…Z`) and numeric offsets (`+HH:MM` / `-HH:MM`)
- Epoch seconds conversion (`to_epoch_secs` / `from_epoch_secs`)
- Ordering (`compare` / `is_before` / `is_after`)
- Calendar helpers (`is_leap_year` / `days_in_month`)
- Live clock: `now_rfc3339()` / `now_epoch_secs()` (thin-wrap `std::time`)

## API

```windjammer
use wj_timefmt

match parse_rfc3339("2024-03-10T18:45:00+05:30") {
    Ok(t) => {
        let out = format_rfc3339(t)
        let epoch = to_epoch_secs(t)
    },
    Err(_) => {},
}

let zero = from_epoch_secs(0)
assert(is_leap_year(2024), "leap")
```

| Function | Description |
|---|---|
| `parse_rfc3339(text)` | Parse Zulu or `+/-HH:MM` timestamp |
| `format_rfc3339(t)` | Format (`Z` when offset is 0) |
| `now_rfc3339()` | Current UTC as RFC3339 (`std::time`) |
| `now_epoch_secs()` | Current UTC unix seconds (`std::time`) |
| `to_epoch_secs(t)` | UTC unix seconds |
| `from_epoch_secs(epoch)` | Zulu `Rfc3339` from unix seconds |
| `compare` / `is_before` / `is_after` | UTC ordering |
| `is_leap_year` / `days_in_month` | Calendar helpers |

## Layout

```
src/lib.wj
tests/timefmt_test.wj
```

## Tip status

❌ tip RED (P3.299) — `int` find-pos `>= 0` emits `as usize >= 0_i64` (plus related int/usize loop arithmetic). Idiomatic sources kept; gate in `windjammer/tests/`.

## License

MIT OR Apache-2.0
