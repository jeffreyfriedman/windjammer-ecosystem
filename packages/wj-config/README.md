# wj-config

Layered string config: **defaults → file → process env** (env only overrides keys already defined).

Compose with [`wj-dotenv`](../wj-dotenv) for `.env` file layers and [`wj-toml`](../wj-toml) for `.toml` config subsets. Build a small env map from `std::env` for keys you care about.

## API

```windjammer
use wj_config
use wj_dotenv
use std::collections::HashMap
use std::env

let mut defaults = HashMap::new()
defaults.insert("HOST", "localhost")
defaults.insert("PORT", "8080")

let file = wj_dotenv::load(".env")?

let mut env_map = HashMap::new()
match env.get("PORT") {
    Some(v) => { env_map.insert("PORT", v) },
    None => {},
}

let cfg = resolve(defaults, file, env_map)
match cfg.get("PORT") {
    Some(port) => println!("listening on {}", port),
    None => {},
}
```

- `merge(base, overlay)` — overlay wins on shared keys
- `overlay_matching(map, env_map)` — env overrides only keys already in `map`
- `resolve(defaults, file, env_map)` — `merge(defaults, file)` then `overlay_matching`

## Layout

```
src/lib.wj                      # merge, overlay_matching, resolve
tests/config_test.wj            # unit tests
```

Compose with [`wj-dotenv`](../wj-dotenv) / [`wj-toml`](../wj-toml) for file layers (see example above). Cross-package `wj test` dev-dependencies are not wired into the test harness yet — integration is documented and exercised manually via app composition.

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-config
$WJ test
```

## License

MIT OR Apache-2.0
