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

## Status

**Blocked** on compiler/runtime wiring for `encoding.base64_encode_string` /
`encoding.base64_decode_string` (declared in `std/encoding`, implemented under
platform native encoding, missing from `windjammer_runtime::encoding` exports).

Repro: `windjammer/tests/bug_std_encoding_base64_string_api_test.rs`

## Layout

```
src/lib.wj
tests/base64_test.wj
```

## License

MIT OR Apache-2.0
