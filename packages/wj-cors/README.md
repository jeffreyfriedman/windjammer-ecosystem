# wj-cors

CORS allow-list helpers for HTTP adapters (pure Windjammer, no framework).

## API

```windjammer
use wj_cors

let mut allowed = Vec::new()
allowed.push("https://app.example.com")

if is_origin_allowed(origin, allowed) {
    match allow_origin(origin, allowed) {
        Some(ao) => { /* set Access-Control-Allow-Origin */ },
        None => {},
    }
}

match preflight(origin, allowed, "GET,POST", "Content-Type") {
    Some(h) => { /* set ACAO / ACAM / ACAH from CorsHeaders */ },
    None => {},
}
```

| Function | Description |
|---|---|
| `is_origin_allowed(origin, allowed)` | Exact match or `*` |
| `allow_origin(origin, allowed)` | Echoed origin, `*`, or `None` |
| `preflight(...)` | `CorsHeaders` for OPTIONS, or `None` |

## Layout

```
src/lib.wj
tests/cors_test.wj
```

## License

MIT OR Apache-2.0
