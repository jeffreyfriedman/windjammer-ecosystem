# wj-config

Layered string config: **defaults → file → process env** (env only overrides keys already defined).

Compose with [`wj-dotenv`](../wj-dotenv) for `.env` file layers and [`wj-toml`](../wj-toml) for `.toml` config subsets (`from_toml` / `resolve_toml`).

## API

```windjammer
use wj_config
use std::collections::HashMap
use std::env

let mut defaults = HashMap::new()
defaults.insert("HOST", "localhost")
defaults.insert("server.PORT", "8080")

let text = std::fs::read_to_string("config.toml")?
let mut env_map = HashMap::new()
match env.get("LOG") {
    Some(v) => { env_map.insert("LOG", v) },
    None => {},
}

let cfg = resolve_toml(defaults, text, env_map)
match cfg.get("server.PORT") {
    Some(port) => println!("listening on {}", port),
    None => {},
}
```

- `merge(base, overlay)` — overlay wins on shared keys
- `overlay_matching(map, env_map)` — env overrides only keys already in `map`
- `resolve(defaults, file, env_map)` — `merge(defaults, file)` then `overlay_matching`
- `from_toml(text)` — flat map via `wj-toml::parse`
- `resolve_toml(defaults, toml_text, env_map)` — `resolve` with a TOML file layer

## Layout

```
src/lib.wj                      # merge, overlay_matching, resolve, from_toml, resolve_toml
tests/config_test.wj            # unit tests
```

## Build / test

Path dep points at `wj-toml`'s `build/`:

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-toml && $WJ build src
cd ../wj-config && $WJ test
```

## License

MIT OR Apache-2.0
