# wj-http-client

Idiomatic GET/POST helpers over `std::http`: status + body as `HttpResult`, plus require-2xx variants.

## API

```windjammer
use wj_http_client

match get_text(url) {
    Ok(result) => {
        if result.is_success() {
            println(result.body)
        }
    },
    Err(e) => println(e),
}

match post_ok(url, body) {
    Ok(text) => println(text),
    Err(e) => println(e),
}
```

- `HttpResult { status, body }` — response snapshot
- `from_parts` / `status_is_success` / `error_message` / `require_ok` — pure helpers (easy to unit-test)
- `get_text` / `post_text` — status + body without requiring 2xx
- `get_ok` / `post_ok` — body on 2xx, otherwise an error string

## Layout

```
src/lib.wj
tests/http_client_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-http-client
$WJ test
```

## License

MIT OR Apache-2.0
