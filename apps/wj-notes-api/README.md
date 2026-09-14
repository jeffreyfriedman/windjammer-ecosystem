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
| **wj-dotenv** + **wj-config** + **wj-toml** | `config_from_dotenv` / `config_from_toml` (flat + `[limits]` section keys) |
| **wj-duration** | rate-limit window as ms digits or duration (`2m`, `30s`) |
| **wj-compress** | `Accept-Encoding` negotiation + `Content-Encoding: gzip` |
| **wj-template** | HTML welcome page at `GET /` (`render_html` + escaped placeholders) |
| **wj-querystring** | `GET /notes?limit=N` pagination cap |
| **wj-uuid** | UUID v7 `uid` on each note at create |
| **wj-timefmt** | RFC3339 `created_at` from request clock (`now_ms`) |
| **wj-inflect** | `slug` from title (`Hello World!` → `hello-world`) |
| **wj-sha** | SHA-256 `ETag` on `GET /notes/:id` + `If-None-Match` → 304 |
| **wj-json-util** | `GET /notes?pretty=1` indented JSON |
| **wj-base64** | `GET /notes/:id?encoding=base64` (`encode_text`) |
| **wj-log** | `LOG_LEVEL` / `log_level` + tagged access lines (`[notes] GET /health -> 200`) |

## Routes

| Method | Path | Status |
|---|---|---|
| `GET` | `/` | 200 HTML welcome (wj-template) |
| `GET` | `/health` | 200 `{ "ok": true }` |
| `GET` | `/notes` | 200 JSON array (optional `?limit=N`, `?pretty=1`) |
| `POST` | `/notes` | 201 created / 400 invalid JSON or validation |
| `GET` | `/notes/:id` | 200 / 304 (ETag / If-None-Match) / 404; optional `?encoding=base64` |
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

for p in wj-router wj-cors wj-headers wj-rate-limit wj-validate wj-mime wj-dotenv wj-config wj-toml wj-log wj-duration wj-compress wj-template wj-querystring wj-uuid wj-timefmt wj-inflect wj-sha wj-json-util wj-base64; do
  cd packages/$p && $WJ build src --library --module-file
done

cd apps/wj-notes-api
$WJ test
$WJ build --release src
```

Environment:

- `CORS_ORIGIN` — allowed origin (default `*`)
- `RATE_LIMIT` — requests per minute per client key (default `100`; `0` disables)
- `RATE_WINDOW` / `RATE_WINDOW_MS` — rate limit window (`2m`, `30s`, or bare ms; default `60000`)
- `MAX_TITLE_LEN` — max note title length (default `200`)
- `MAX_BODY_LEN` — max note body length (default `10000`)
- `PORT` (default `8080`)

Also `config_from_dotenv(text)` and `config_from_toml(text)` merge the same keys over defaults
(`cors_origin`, `rate_limit`, `window_ms`, `max_title_len`, `max_body_len`, `log_level`; TOML also
accepts `[limits]` section keys).
