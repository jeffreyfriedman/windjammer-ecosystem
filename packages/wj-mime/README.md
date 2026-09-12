# wj-mime

MIME type helpers: common constants, extension/path lookup, and `text` / `image` / `audio` / `video` predicates.

**Graduation status:** Fully thin-wraps `std::mime` (constants, lookup, predicates). P3.243 charset parity ✅ tip GREEN.

## API

```windjammer
use wj_mime

assert_eq(json(), "application/json; charset=utf-8")
assert_eq(from_extension("png"), "image/png")
assert_eq(from_path("index.html"), "text/html; charset=utf-8")
assert(is_text("application/json; charset=utf-8"))
assert(is_image("image/png"))
```

## Layout

```
src/lib.wj
tests/mime_test.wj
```

## License

MIT OR Apache-2.0
