# wj-validate

Composable field validators (zod / pydantic / FluentValidation style) in pure Windjammer.

## API

```windjammer
use wj_validate

match require_nonempty("name", name) {
    Ok(n) => {},
    Err(e) => {},
}

match require_email("email", email) {
    Ok(e) => {},
    Err(_) => {},
}

match require_int_range("age", age, 0, 120) {
    Ok(a) => {},
    Err(_) => {},
}

let mut errors = Vec::new()
match require_min_len("password", password, 8) {
    Ok(_) => {},
    Err(e) => errors.push(e),
}
match all_ok(errors) {
    Ok(_) => {},
    Err(msg) => {},
}
```

| Function | Description |
|---|---|
| `require_nonempty` | Trim + non-empty |
| `require_min_len` / `require_max_len` | Length bounds |
| `require_email` | Lightweight `local@domain` shape |
| `require_url` | Absolute `http://` / `https://` URL |
| `require_uuid` | Canonical UUID **v1 / v4 / v5 / v7** (`8-4-4-4-12`, RFC variant) |
| `require_int_range` | Inclusive int bounds |
| `require_one_of` | Allow-list membership |
| `all_ok` | Aggregate error list |

## Layout

```
src/lib.wj
tests/validate_test.wj
```

## License

MIT OR Apache-2.0
