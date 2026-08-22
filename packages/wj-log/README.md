# wj-log

Helpers over `std::log`: parse a level from text, initialize logging, and emit tagged messages.

## API

```windjammer
use wj_log

init_from_level_text("info")?
log_tagged("info", "api", "listening")
```

- `parse_level(text)` — `trace` / `debug` / `info` / `warn` / `error` (case-insensitive aliases)
- `init_from_level_text(text)` — `Result<(), string>`; unknown levels error
- `log_tagged(level, tag, message)` — routes to the matching std log function; unknown levels fall back to info

## Layout

```
src/lib.wj          # parse_level, init_from_level_text, log_tagged
tests/log_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-log
$WJ test
```

## License

MIT OR Apache-2.0
