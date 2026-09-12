# wj-webhook

Webhook worker: verify inbound POSTs (shared token or SHA-256 demo signature), record events, expose a tiny JSON API.

Reference-quality hexagonal Windjammer app — domain ports use `std::http::HttpMethod` and `ServerResponse` helpers, not stringly-typed HTTP.

No Cargo crates, no `extern fn`, no `ffi/`. Domain logic is pure Windjammer; HTTP lives in adapters.

## Endpoints

| Method | Path | Auth | Behavior |
|--------|------|------|----------|
| `GET` | `/health` | none | `{"ok":true}` |
| `POST` | `/webhook` | token or signature | parse `{event,id}`, validate fields, append to in-memory log + event bus |
| `GET` | `/events` | none | JSON array of recorded events |
| `GET` | `/bus/drain` | none | drain `wj-event` queue from last POSTs |
| `OPTIONS` | `*` | CORS | preflight when `CORS_ORIGIN` is set |

Auth (either):

- Header `X-Webhook-Token: <secret>` matching `WEBHOOK_SECRET`
- Header `X-Webhook-Signature: <hex>` where hex is `sha256("${secret}.${body}")` (demo until std HMAC)

## Usage

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj
export WEBHOOK_SECRET=secret
export CORS_ORIGIN=http://localhost:3000
export PORT=8090

cd apps/wj-webhook
$WJ test
$WJ build --release src
cd build && cargo run --release
```

```bash
curl -s http://127.0.0.1:8090/health
curl -s -X POST http://127.0.0.1:8090/webhook \
  -H 'Content-Type: application/json' \
  -H 'X-Webhook-Token: secret' \
  -d '{"event":"ping","id":"1"}'
curl -s http://127.0.0.1:8090/events
curl -s http://127.0.0.1:8090/bus/drain
```

## Layout

```
src/
  domain/config.wj       # env-backed WebhookConfig
  domain/webhook.wj      # HttpMethod routing (internal), auth, validate, wj-event
  adapters/http_server.wj  # enum→label at boundary, ServerResponse helpers
  main.wj                # composition root
tests/
  webhook_test.wj
  config_test.wj
  event_bus_test.wj
```

## Idiomatic HTTP patterns (copy this)

**Domain (internal)** — match on `HttpMethod`, not `"GET"` strings:

```windjammer
use std::http::HttpMethod

match method {
    HttpMethod::GET => json_ok(),
    _ => json_error(404, "not found"),
}
```

**Public port + tests** — accept `string` until `bug_app_test_http_method_type_mismatch_test` greens (lib vs test crate enum split):

```windjammer
pub fn handle(self, method: string, path: string, ...) -> HttpReply {
    let method = parse_method(method)  // "GET" → HttpMethod::GET
    self.handle_method(method, path, ...)
}
```

**Adapter** — `match req.method { HttpMethod::GET => "GET", … }`; do not use `.as_str()` (Rust leakage lint).

**Responses** — map domain status to `ServerResponse` constructors (`bad_request`, `unauthorized`, …), not raw `400`/`401` literals.

## Environment

| Variable | Default | Purpose |
|---|---|---|
| `WEBHOOK_SECRET` | `dev-secret` | Shared token + signature salt |
| `CORS_ORIGIN` | *(empty)* | Allowed browser origin for preflight |
| `WEBHOOK_MAX_EVENT_LEN` | `128` | Max `event` field length |
| `WEBHOOK_MAX_ID_LEN` | `128` | Max `id` field length |
| `PORT` | `8090` | Listen port |

## Packages dogfooded

| Package | Role |
|---|---|
| `wj-cors` | Origin allow-list + OPTIONS preflight |
| `wj-headers` | Default security header block on every response |
| `wj-validate` | Nonempty + max-length checks on `event` / `id` |
| `wj-event` | Queue-based event bus on webhook receive |

Also uses `std::crypto`, `std::json`, `std::http`, `std::env`, `std::strings`.

## License

MIT OR Apache-2.0
