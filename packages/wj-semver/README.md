# wj-semver

Parse, compare, and format semantic versions (`major.minor.patch`, optional `-pre`, `+build`).

## API

```windjammer
use wj_semver

match parse("1.2.3-alpha.1+meta") {
    Ok(v) => {
        if compare(v, other) < 0 {
            println("older")
        }
        println(format_version(v))
    },
    Err(e) => println(e),
}
```

- `parse(text)` → `Result<SemVer, string>`
- `compare(a, b)` → `-1 | 0 | 1` (ignores build metadata)
- `format_version(v)` → canonical string

## Layout

```
src/lib.wj
tests/semver_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-semver
$WJ test
```

## License

MIT OR Apache-2.0
