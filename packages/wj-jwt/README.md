# wj-jwt

HS256 JWT sign/verify helpers over `std::jwt`.

## API

```windjammer
use wj_jwt

match sign("user-1", "acme", "secret", 3600) {
    Ok(token) => {
        match verify(token, "secret") {
            Ok(claims) => println(claims.tenant_slug),
            Err(e) => println(e),
        }
    },
    Err(e) => println(e),
}

match sign_scoped("user-1", "acme", "t-1", "e-1", "a@b.c", "secret", 3600) {
    Ok(token) => {},
    Err(e) => println(e),
}
```

## Layout

```
src/lib.wj
tests/jwt_test.wj
```

## License

MIT OR Apache-2.0
