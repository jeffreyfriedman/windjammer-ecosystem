# wj-hash

Password hashing helpers over `std::crypto` bcrypt.

## API

```windjammer
use wj_hash

match hash_password("secret123") {
    Ok(digest) => {
        match verify_password("secret123", digest) {
            Ok(ok) => {},
            Err(e) => println(e),
        }
    },
    Err(e) => println(e),
}
```

| Function | Description |
|---|---|
| `hash_password(password)` | bcrypt hash (default cost) |
| `verify_password(password, hash)` | verify against bcrypt digest |

## Layout

```
src/lib.wj
tests/hash_test.wj
```

## License

MIT OR Apache-2.0
