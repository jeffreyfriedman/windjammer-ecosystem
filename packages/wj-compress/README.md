# wj-compress

HTTP gzip helpers: Accept-Encoding negotiation plus Base64 gzip body codecs over `std::compress`.

## API

```windjammer
use wj_compress

if accepts_gzip(req_accept_encoding) {
    match gzip_encode(body) {
        Ok(encoded) => {
            let encoding = content_encoding_gzip()
        },
        Err(e) => println(e),
    }
}

match negotiate(req_accept_encoding) {
    Some(enc) => {},
    None => {},
}
```

| Function | Description |
|---|---|
| `accepts_gzip(header)` | True when Accept-Encoding allows gzip |
| `negotiate(header)` | `Some("gzip")` or `None` |
| `content_encoding_gzip()` | `"gzip"` response value |
| `prefer_identity(header)` | Empty / `identity` Accept-Encoding |
| `gzip_encode(body)` | Gzip → Base64 string |
| `gzip_decode(encoded)` | Base64 gzip → UTF-8 string |

## Layout

```
src/lib.wj
tests/compress_test.wj
```

## License

MIT OR Apache-2.0
