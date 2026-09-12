# wj-notes-api

In-memory notes CRUD over HTTP — Wave 1 REST seed, hardened with ecosystem packages for routing, CORS, security headers, and rate limiting.

No Cargo crates, no `extern fn`, no `ffi/`. Domain routing and JSON live in Windjammer; the HTTP adapter binds `std::http`.

## Packages

| Layer | Module |
|---|---|
| **wj-router** | `/notes` and `/notes/:id` path matching |
| **wj-cors** | preflight + `Access-Control-Allow-Origin` |
| **wj-headers** | Helmet-style security headers on every response |
| **wj-rate-limit** | fixed-window per `X-Client-Key` (or `anonymous`) |

## Routes

| Method | Path | Status |
|---|---|---|
| `GET` | `/health` | 200 `{ "ok": true }` |
| `GET` | `/notes` | 200 JSON array |
| `POST` | `/notes` | 201 created / 400 invalid JSON |
| `GET` | `/notes/:id` | 200 / 404 |
| `PUT` | `/notes/:id` | 200 / 400 / 404 |
| `DELETE` | `/notes/:id` | 204 / 404 |
| `OPTIONS` | `*` | 204 CORS preflight when origin allowed |

Bind address is `0.0.0.0`. Port comes from `PORT` (default `8080`).

## Layout

```
src/
  domain/
    config.wj              # NotesConfig defaults + env (PORT, CORS_ORIGIN, RATE_LIMIT)
    store.wj               # NoteStore CRUD (lookup_note workaround for HashMap i64 get)
    api.wj                 # NotesApp routing — internal HttpMethod match, string public port
  adapters/http_server.wj  # std::http + Arc/Mutex; enum→label match, ServerResponse helpers
  main.wj                  # composition root
tests/
  config_test.wj
  store_test.wj
  api_test.wj
```

## HTTP idioms

| Layer | Pattern |
|---|---|
| **Adapter** | `match req.method { HttpMethod::GET => "GET", … }` — no `.as_str()` |
| **Domain internal** | `match method { HttpMethod::GET => … }` |
| **Public port + tests** | `handle(method: string, …)` → `parse_method()` → internal enum |
| **Status mapping** | `ServerResponse::bad_request()`, `with_status(404, body)`, etc. — not raw status ints in adapter |

String public port is a workaround until `bug_app_test_http_method_type_mismatch_test` is green.

## Build / test

Path dependencies must point at each package’s `build/` directory. Pre-build deps once:

```bash
unset CARGO_TARGET_DIR
export WJ=~/.cargo/bin/wj   # or windjammer/target/release/wj

for p in wj-router wj-cors wj-headers wj-rate-limit; do
  cd packages/$p && $WJ build src
done

cd apps/wj-notes-api
$WJ test
$WJ build --release src
```

Environment:

- `CORS_ORIGIN` — allowed origin (default `*`)
- `RATE_LIMIT` — requests per minute per client key (default `100`; `0` disables)
- `RATE_WINDOW_MS` — rate limit window in ms (default `60000`)
- `PORT` (default `8080`)

## License

MIT OR Apache-2.0
