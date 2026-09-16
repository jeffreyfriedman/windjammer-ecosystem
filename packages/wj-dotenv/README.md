# wj-dotenv

Load `KEY=VALUE` lines from a `.env`-style file or string into a map.

## API

```windjammer
use wj_dotenv

let map = parse("HOST=localhost\n# comment\nPORT=8080\n")
let from_file = load(".env")?
```

- Blank lines and `#` comments are skipped
- Keys and values are trimmed
- Values may contain `=`

## Layout

```
src/lib.wj          # parse + load
tests/dotenv_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-dotenv
$WJ test
```

## License

MIT OR Apache-2.0

## Tip status

❌ tip RED (P3.311) — `while i < parts.len()` index loop emits `i += 1 as i32`. Idiomatic sources kept.
