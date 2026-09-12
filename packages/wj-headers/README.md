# wj-headers

Helmet-style secure HTTP response headers in pure Windjammer.

## API

```windjammer
use wj_headers

let h = defaults()
h.hsts_max_age_secs = 31536000
let text = format_headers(h)
```

Helpers: `nosniff()`, `frame_options("SAMEORIGIN")`, `hsts(max_age_secs)`.

## Layout

```
src/lib.wj
tests/headers_test.wj
```

## License

MIT OR Apache-2.0
