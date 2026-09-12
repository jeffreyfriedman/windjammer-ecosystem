# wj-yaml

YAML config helpers: `to_json` via `std::yaml`, plus dotted-path getters.

## API

```windjammer
use wj_yaml

let json = to_json("name: alice\nage: 30\n")?
assert_eq(get_str("name: bob\n", "name"), Some("bob"))
assert_eq(get_int("port: 8080\n", "port"), Some(8080))
assert_eq(get_str("server:\n  host: localhost\n", "server.host"), Some("localhost"))
```

## Architecture

| Function | Source |
|---|---|
| `to_json` | Thin-wraps `std::yaml.to_json` (serde_yaml backend) |
| `get_str` / `get_int` / `get_bool` | Package sugar: `to_json` → `json.parse` → dotted path walk |

Empty/whitespace-only input is rejected by `std::yaml.to_json` (P3.244 ✅ tip GREEN).

## Supported (via std)

Full YAML 1.x through serde_yaml: block/flow maps and lists, scalars, block scalars (`|`, `>`), comments, multi-document (first doc wins for JSON bridge).

## Layout

```
src/lib.wj
tests/yaml_test.wj
```

## License

MIT OR Apache-2.0
