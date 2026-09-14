# wj-notes-api

In-memory notes CRUD over HTTP — Wave 1 REST seed, hardened with ecosystem packages for routing, CORS, security headers, rate limiting, validation, and MIME.

No Cargo crates, no `extern fn`, no `ffi/`. Domain routing and JSON live in Windjammer; the HTTP adapter binds `std::http`.

## Packages

| Layer | Module |
|---|---|
| **wj-router** | `/notes` and `/notes/:id` path matching |
| **wj-cors** | preflight + `Access-Control-Allow-Origin` |
| **wj-headers** | Helmet-style security headers on every response |
| **wj-rate-limit** | fixed-window per `X-Client-Key` (or `anonymous`) |
| **wj-validate** | nonempty + max-len on note title/body |
| **wj-mime** | `Content-Type` on `HttpReply` (JSON replies) |

## Routes

| Method | Path | Status |
|---|---|---|
| `GET` | `/health` | 200 `{ "ok": true }` |
| `GET` | `/notes` | 200 JSON array |
| `POST` | `/notes` | 201 created / 400 invalid JSON or validation |
| `GET` | `/notes/:id` | 200 / 404 |
| `PUT` | `/notes/:id` | 200 / 400 / 404 |
| `DELETE` | `/notes/:id` | 204 / 404 |
| `OPTIONS` | `*` | 204 CORS preflight when origin allowed |

Bind address is `0.0.0.0`. Port comes from `PORT` (default `8080`).

## Layout

```
src/
  domain/
    config.wj              # NotesConfig defaults + env (PORT, CORS, rate, max lens)
    store.wj               # NoteStore CRUD + wj-validate field checks
    api.wj                 # NotesApp routing — string `handle` + HttpMethod `handle_http`
  adapters/http_server.wj  # std::http + Arc/Mutex; Content-Type from domain
  main.wj                  # composition root
tests/
  config_test.wj
  store_test.wj
  api_test.wj
```

## HTTP idioms

| Layer | Pattern |
|---|---|
| **Adapter** | `handle_http(req.method, …)` — same-crate `HttpMethod` (avoids demoted `method: &str`) |
| **Domain internal** | `match method { HttpMethod::GET => … }` |
| **Public port + tests** | `handle(method: string, …)` → `parse_method()` → internal enum |
| **Status mapping** | `ServerResponse::bad_request()`, `with_status(404, body)`, etc. |

## Build / test

Path dependencies must point at each package’s `build/` directory. Pre-build deps once:

```bash
unset CARGO_TARGET_DIR
export WJ=~/.cargo/bin/wj   # or windjammer/target/release/wj

for p in wj-router wj-cors wj-headers wj-rate-limit wj-validate wj-mime; do
  cd packages/$p && $WJ build src --library --module-file
done

cd apps/wj-notes-api
$WJ test
$WJ build --release src
```

Environment:

- `CORS_ORIGIN` — allowed origin (default `*`)
- `RATE_LIMIT` — requests per minute per client key (default `100`; `0` disables)
- `RATE_WINDOW_MS` — rate limit window in ms (default `60000`)
- `MAX_TITLE_LEN` — max note title length (default `200`)
- `MAX_BODY_LEN` — max note body length (default `10000`)
- `PORT` (default `8080`)
