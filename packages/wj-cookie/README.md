# wj-cookie

Parse and format HTTP `Cookie` / `Set-Cookie` headers (pure Windjammer).

## API

```windjammer
use wj_cookie

match parse_cookie_header("sid=abc; theme=dark") {
    Ok(map) => {
        let header = format_cookie_header(map)
    },
    Err(_) => {},
}

match parse_set_cookie("session=xyz; Path=/; HttpOnly; Secure; SameSite=Lax") {
    Ok(c) => {
        let out = format_set_cookie(c)
    },
    Err(_) => {},
}
```

| Function | Description |
|---|---|
| `parse_cookie_header` | `Cookie` request header → map |
| `format_cookie_header` | map → `Cookie` header |
| `parse_set_cookie` | single `Set-Cookie` → `SetCookie` |
| `format_set_cookie` | `SetCookie` → header |

## Layout

```
src/lib.wj
tests/cookie_test.wj
```

## License

MIT OR Apache-2.0
