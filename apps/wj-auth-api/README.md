# wj-auth-api

Hexagonal sample API that dogfoods ecosystem packages plus stdlib crypto/JWT/compress.

Reference pattern for idiomatic HTTP in Windjammer apps (see also `wj-webhook`):

- **Domain** — `handle(string, …)` for tests (dual-runtime `HttpMethod` mismatch); `handle_http(HttpMethod, …)` for same-crate adapter
- **Config** — `AuthConfig::defaults()`, `config_from_env`, `config_from_toml`, `config_from_dotenv` (dogfoods `wj-config` + `wj-toml` + `wj-dotenv`)
- **Adapter** — passes `req.method` into `handle_http` (avoids string-lit → demoted `&str` + `.to_string()`)

## Packages

| Layer | Module |
|---|---|
| **wj-cors** | preflight + response `Access-Control-Allow-Origin` |
| **wj-compress** | `Accept-Encoding` negotiation + `Content-Encoding: gzip` |
| **wj-template** | HTML welcome page (`render_html`) |
| **wj-uuid** | RFC 9562 **v7** user ids on register |
| **wj-config** / **wj-toml** | `config_from_toml` (flat + `[jwt]` section keys) |
| **wj-dotenv** | `config_from_dotenv` (`JWT_SECRET=…` env-file layer via `wj-config::merge`) |
| **wj-timefmt** | `/me.expires_at` RFC3339 Zulu from JWT `exp` |
| **wj-inflect** | slugify usernames on register/login (`Carl User` → `carl-user`) |
| **wj-cookie** | login `Set-Cookie: access_token=…; HttpOnly; Path=/; SameSite=Lax`; `/me` accepts cookie |
| **wj-rate-limit** | fixed-window limiter + `X-RateLimit-*` / `Retry-After` on 429 |
| **wj-headers** | helmet-style defaults (`X-Frame-Options`, `X-Content-Type-Options`, …) |
| **wj-validate** | username/password nonempty + min/max length on register/login |
| **wj-hash** | bcrypt register/login (thin wrap of `std::crypto`) |
| **wj-jwt** | HS256 sign/verify; `/me` exposes `tenant` from claims |
| **wj-router** | path normalize + match (`/health`, trailing slash, missing leading `/`) |
| **wj-mime** | `Content-Type` for JSON / HTML replies (`application/json`, `text/html`) |
| **wj-duration** | human JWT TTL / rate-limit window (`2h`, `30m`, `2m`) in TOML/env |
| **std::compress** | gzip body encode/decode in transport layer |

## Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/health` | JSON `{ "ok": true }` |
| GET | `/` | HTML welcome page |
| POST | `/register` | `{ "username", "password" }` → 201 `{ "created", "id", "username" }` (id is UUID v7; username is slugified) |
| POST | `/login` | credentials → `{ "token" }` + `Set-Cookie` access_token |
| POST | `/logout` | clears access_token cookie |
| GET | `/me` | `Authorization: Bearer …` **or** `Cookie: access_token=…` → `{ "username", "sub", "tenant", "expires_at" }` |
| OPTIONS | `*` | CORS preflight |

## Layout

```
src/
  domain/config.wj       # AuthConfig + env + TOML
  domain/auth.wj         # HttpMethod routing, auth rules, cookies, rate limit
  adapters/http_server.wj
  main.wj
tests/
  auth_test.wj
  config_test.wj
```

## Build / test

Path dependencies must point at each package’s `build/` directory. Prefer `--library --module-file` so `metadata.json` is emitted for cross-crate ownership:

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj   # or a known-good pinned wj

for p in wj-cors wj-compress wj-template wj-uuid wj-toml wj-config wj-cookie wj-rate-limit wj-headers wj-validate wj-hash wj-jwt wj-router wj-mime wj-duration wj-dotenv wj-timefmt wj-inflect; do
  cd packages/$p && $WJ build src --library --module-file
done

cd apps/wj-auth-api
$WJ test --no-runtime-copy
$WJ build --release src
```

Environment:

| Variable | Default | Purpose |
|---|---|---|
| `JWT_SECRET` | `dev-secret` | HS256 signing secret |
| `JWT_TTL_SECS` / `JWT_TTL` | `3600` | Token lifetime (seconds or duration like `1h`) |
| `TENANT_SLUG` | `default` | JWT `tenant_slug` claim + `/me.tenant` |
| `CORS_ORIGIN` | `*` | Allowed browser origin |
| `RATE_LIMIT` | `0` (off) | Fixed-window request limit |
| `WINDOW_MS` / `WINDOW` | `60000` | Rate-limit window (ms or duration like `2m`) |
| `MAX_USERNAME_LEN` | `64` | Register username max length |
| `MIN_PASSWORD_LEN` | `8` | Register/login password min length |
| `MAX_PASSWORD_LEN` | `128` | Register/login password max length |
| `PORT` | `8091` | Listen port |

Request headers: `X-Client-Key` selects the rate-limit bucket (defaults to `anon`).

## License

MIT OR Apache-2.0
