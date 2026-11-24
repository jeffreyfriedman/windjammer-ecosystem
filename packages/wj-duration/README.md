# wj-duration

Parse and format human durations as milliseconds (`5ms`, `3s`, `1h30m`).

Thin-wraps `std::time.parse_duration_ms` / `format_duration_ms`.

## API

```windjammer
use wj_duration

match parse_ms("1h30m") {
    Ok(ms) => {
        let text = format_ms(ms)  // "1h30m"
    },
    Err(_) => {},
}
```

Bare numbers are milliseconds. Units: `ms`, `s`, `m`, `h`, `d`.

## Layout

```
src/lib.wj
tests/duration_test.wj
```

## Tip status

✅ tip green — thin-wrap over `std::time` (parse/format duration ms).

## License

MIT OR Apache-2.0
