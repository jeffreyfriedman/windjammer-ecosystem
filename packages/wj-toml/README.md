# wj-toml

Pure Windjammer TOML config subset → flat `HashMap<string, string>` (useful for tip dogfooding / subset scanners).

**Prefer [`wj-config`](../wj-config)** for app config: `from_toml` / `resolve_toml` go through `std::config.parse_flat` (full TOML via runtime). This package remains for the intentional subset and compiler repros.

## Supported

- `#` comments (full-line and trailing outside quotes)
- `[section]` headers → dotted keys (`server.host`)
- Dotted left-hand keys (`server.port = 8080`)
- `key = "quoted"`, bare strings, ints, bools
- Single-line arrays → compact JSON (`tags = ["a","b"]` → `["a","b"]`, `[80,443]`, `[]`)
- Single-line inline tables → flattened dotted keys (`point = { x = 1, y = 2 }` → `point.x` / `point.y`)

## Not supported

Nested inline tables, multiline strings, multi-line arrays.

## API

```windjammer
use wj_toml
use wj_config
use std::collections::HashMap

let file = parse("host = \"localhost\"\n[server]\nport = 8080\n")
match get("host = \"x\"\n", "host") {
    Some(v) => {},
    None => {},
}

let mut defaults = HashMap::new()
defaults.insert("host", "127.0.0.1")
let env_map = HashMap::new()
let cfg = resolve(defaults, file, env_map)
```

## Layout

```
src/lib.wj
tests/toml_test.wj
```

## License

MIT OR Apache-2.0
