# wj-base64

Base64 encode/decode helpers over `std::encoding`.

## API

```windjammer
use wj_base64

let encoded = encode("Hello")
match decode(encoded) {
    Ok(text) => println(text),
    Err(e) => println(e),
}

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
