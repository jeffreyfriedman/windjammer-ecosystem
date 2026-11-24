# wj-config

Layered string config: **defaults → file → process env** (env only overrides keys already defined).

**Parse** thin-wraps `std::config.parse_flat` (TOML/YAML; format is an implementation detail). **Merge / resolve** stay pure Windjammer until tip greens owned-`HashMap` call sites for `config.resolve` / `config.merge` (`bug_std_config_module_test`).

Compose with [`wj-dotenv`](../wj-dotenv) for `.env` layers. Structured interchange stays in `std::json` via `std::config.to_json`.

## API

```windjammer
use wj_config
use std::collections::HashMap

let mut defaults = HashMap::new()
defaults.insert("HOST", "localhost")
defaults.insert("server.PORT", "8080")

let cfg = resolve_toml(defaults, text, env_map)
```

- `merge` / `overlay_matching` / `resolve` — package-local (tip HashMap ownership)
- `from_toml` / `from_yaml` — `std::config.parse_flat`
- `resolve_toml` / `resolve_yaml` — parse + local resolve

## License

MIT OR Apache-2.0
