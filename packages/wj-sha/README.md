# wj-sha

Idiomatic SHA-256 hex digests over `std::crypto`.

## API

```windjammer
use wj_sha

let digest = hex("hello")
assert(verify("hello", digest))
```

| Function | Description |
|---|---|
| `hex(text)` | Lowercase hex SHA-256 of a string |
| `verify(text, expected_hex)` | Constant-time-enough equality check of digests |

## Layout

```
src/lib.wj
tests/sha_test.wj
```

## License

MIT OR Apache-2.0
