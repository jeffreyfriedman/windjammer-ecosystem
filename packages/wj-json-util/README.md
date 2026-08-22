# wj-json-util

Pretty-print, dotted path get, and deep object merge helpers over `std::json`.

## API

```windjammer
use wj_json_util

println(pretty("{\"a\":1}"))

match path_get_str("{\"user\":{\"name\":\"Ada\"}}", "user.name") {
    Some(name) => println(name),
    None => {},
}

match merge("{\"a\":1,\"b\":2}", "{\"b\":9,\"c\":3}") {
    Ok(out) => println(out),
    Err(e) => println(e),
}
```

- `pretty(text)` — indented JSON, or the original text if parse fails
- `path_get(text, path)` — compact JSON at a dotted path (`user.name`, `nums.0`)
- `path_get_str(text, path)` — string leaf only
- `merge(base, overlay)` — deep-merge objects (overlay wins on non-object conflicts)
- `parse_nonneg_int(text)` — path index helper (`0`, `12`; rejects `01`)

## Layout

```
src/lib.wj
tests/json_util_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-json-util
$WJ test
```

## License

MIT OR Apache-2.0
