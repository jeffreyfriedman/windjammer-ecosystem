# wj-url

Parse, format, join, and query helpers for absolute URLs (pure Windjammer / `std::strings`).

## API

```windjammer
use wj_url

match parse("https://example.com:8443/api?x=1#top") {
    Ok(u) => {
        println(format_url(u))
        match join_url("https://example.com/a/b", "c?x=1#sec") {
            Ok(joined) => println(joined),
            Err(e) => println(e),
        }
        let u2 = with_query(u, "a=1&b=2")
        println(format_url(u2))
    },
    Err(e) => println(e),
}

match query_get("a=1&b=two", "b") {
    Some(v) => println(v),
    None => {},
}
let q = query_set("a=1", "b", "2")
let q2 = query_remove(q, "a")
assert(query_has(q2, "b"), "b remains")
```

| Function | Description |
|---|---|
| `parse(text)` | Absolute URL → `Url` |
| `format_url(u)` | `Url` → absolute string |
| `join_url(base, relative)` | Resolve relative against base (keeps `?query` / `#fragment`) |
| `query_get(query, key)` | First value for key (`+` / `%HH` decoded) |
| `query_has(query, key)` | Whether key is present |
| `query_set(query, key, value)` | Add/replace key (form-encoded) |
| `query_remove(query, key)` | Drop all pairs for key |

Query helpers use `std::encoding` form decode/encode (same rules as `wj-querystring`). Thin-wrap over `wj-querystring` once tip greening cross-crate owned→`&str` call sites.

**Graduation:** tip RED gate `bug_std_url_parse_wiring_test` → `std::url`; int/usize assign unify is `bug_int_loop_assign_end_bound_must_unify_test`. See `docs/STDLIB_URL_HANDOFF.md`.

## Layout
| `with_query(u, query)` | Copy of `Url` with query replaced |

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
