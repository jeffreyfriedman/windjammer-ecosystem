# wj-url

Parse, format, join, and query helpers for absolute URLs (pure Windjammer / `std::strings`).

## API

```windjammer
use wj_url

match parse("https://example.com:8443/api?x=1#top") {
    Ok(u) => {
        println(format_url(u))
        match join_url("https://example.com/a/b", "c") {
            Ok(joined) => println(joined),
            Err(e) => println(e),
        }
    },
    Err(e) => println(e),
}

match query_get("a=1&b=two", "b") {
    Some(v) => println(v),
    None => {},
}
let q = query_set("a=1", "b", "2")
```

| Function | Description |
|---|---|
| `parse(text)` | Absolute URL → `Url` |
| `format_url(u)` | `Url` → absolute string |
| `join_url(base, relative)` | Resolve relative against base (`join_url` avoids clashing with `strings.join`) |
| `query_get(query, key)` | First value for key |
| `query_set(query, key, value)` | Add/replace key |

## Layout

```
src/lib.wj
tests/url_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj
cd packages/wj-url
$WJ test
```

## License

MIT OR Apache-2.0
