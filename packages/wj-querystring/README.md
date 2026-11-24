# wj-querystring

`application/x-www-form-urlencoded` parse / stringify helpers in pure Windjammer.

## API

```windjammer
use wj_querystring

let pairs = parse("a=1&b=two")
let text = stringify(pairs)
match get("a=1&a=2", "a") {
    Some(v) => assert_eq(v, "1"),
    None => {},
}
let tags = get_all("tag=red&tag=blue", "tag")
assert(has("a=1", "a"), "present")
let dropped = remove("a=1&b=2", "a")
let both = append("a=1", "a", "2")
```

Spaces decode from `+` / `%20`; stringify uses `std::encoding.url_encode` (with `%20`→`+` for form bodies) and `url_decode` for values.

**Graduation:** tip RED gate `bug_std_encoding_form_urlencoded_wiring_test` targets `std::encoding.form_parse` / `form_stringify`. Once GREEN, this package thin-wraps those and keeps `get` / `append` / `remove` as sugar. See `docs/STDLIB_FORM_HANDOFF.md`.

## Layout

```
src/lib.wj
tests/querystring_test.wj
```

## License

MIT OR Apache-2.0
