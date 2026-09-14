# wj-base64

Base64 encode/decode helpers over `std::encoding`.

## API

```windjammer
use wj_base64

let encoded = encode_text("Hello")
match decode_text(encoded) {
    Ok(text) => println(text),
    Err(e) => println(e),
}

// `encode` / `decode` remain; prefer `*_text` at app call sites.
let raw = encode_bytes(bytes)
match decode_bytes(raw) {
    Ok(bytes) => {},
    Err(e) => println(e),
}
```

## Layout

```
src/lib.wj
tests/base64_test.wj
```

## License

MIT OR Apache-2.0
