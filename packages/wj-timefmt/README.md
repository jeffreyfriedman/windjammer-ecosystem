# wj-timefmt

RFC3339 parse/format in pure Windjammer.

Supports:

- Zulu (`…Z`) and numeric offsets (`+HH:MM` / `-HH:MM`)
- Epoch seconds conversion (`to_epoch_secs` / `from_epoch_secs`)
- Ordering (`compare` / `is_before` / `is_after`)
- Calendar helpers (`is_leap_year` / `days_in_month`)

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
| `to_epoch_secs(t)` | UTC unix seconds |
| `from_epoch_secs(epoch)` | Zulu `Rfc3339` from unix seconds |
| `compare` / `is_before` / `is_after` | UTC ordering |
| `is_leap_year` / `days_in_month` | Calendar helpers |

## Layout

```
src/lib.wj
tests/timefmt_test.wj
```

## License

MIT OR Apache-2.0
