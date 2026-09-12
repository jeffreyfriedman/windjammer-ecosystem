# wj-duration

Parse and format human durations as milliseconds (`5ms`, `3s`, `1h30m`).

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

## License

MIT OR Apache-2.0
