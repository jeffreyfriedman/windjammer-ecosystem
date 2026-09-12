# wj-router

HTTP path matching with `:param` captures (pure Windjammer).

## API

```windjammer
use wj_router

match match_route("/users/:id", "/users/42") {
    Some(params) => {
        match params.get("id") {
            Some(id) => println("id={id}"),
            None => {},
        }
    },
    None => {},
}
```

Trailing slashes are normalized. Segment counts must match.

## Layout

```
src/lib.wj
tests/router_test.wj
```

## License

MIT OR Apache-2.0
