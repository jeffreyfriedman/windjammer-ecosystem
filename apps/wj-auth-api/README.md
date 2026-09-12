# wj-auth-api

Hexagonal sample API that dogfoods ecosystem packages plus stdlib crypto/JWT/compress.

Reference pattern for idiomatic HTTP in Windjammer apps (see also `wj-webhook`):

- **Domain** — internal `HttpMethod` enum matching; public port accepts `string` (test crate compatibility)
- **Adapter** — `match req.method { HttpMethod::GET => "GET", … }` + `ServerResponse` constructors
- **Config** — `domain/config.wj` with `AuthConfig::defaults()` and env parsing

## Packages

| Layer | Module |
|---|---|
| **wj-cors** | preflight + response `Access-Control-Allow-Origin` |
| **wj-compress** | `Accept-Encoding` negotiation + `Content-Encoding: gzip` |
| **wj-template** | HTML welcome page (`render_html`) |
| **std::crypto** | bcrypt register/login |
| **std::jwt** | HS256 bearer tokens |
| **std::compress** | gzip body encode/decode in transport layer |

## Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/health` | JSON `{ "ok": true }` |
| GET | `/` | HTML welcome page |
| POST | `/register` | `{ "username", "password" }` → 201 |
| POST | `/login` | credentials → `{ "token" }` |
| GET | `/me` | `Authorization: Bearer …` → profile JSON |
| OPTIONS | `*` | CORS preflight |

## Layout

```
src/
  domain/config.wj       # AuthConfig + env
  domain/auth.wj         # HttpMethod routing, auth rules
  adapters/http_server.wj
  main.wj
tests/
  auth_test.wj
  config_test.wj
```

## Build / test

Path dependencies must point at each package’s `build/` directory. Pre-build deps once:

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

for p in wj-cors wj-compress wj-template; do
  cd packages/$p && $WJ build src
done

cd apps/wj-auth-api
$WJ test
$WJ build --release src
```

Environment:

| Variable | Default | Purpose |
|---|---|---|
| `JWT_SECRET` | `dev-secret` | HS256 signing secret |
| `JWT_TTL_SECS` | `3600` | Token lifetime |
| `CORS_ORIGIN` | `*` | Allowed browser origin |
| `PORT` | `8091` | Listen port |

## License

MIT OR Apache-2.0
