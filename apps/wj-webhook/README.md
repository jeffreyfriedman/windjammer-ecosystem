# wj-webhook

Webhook worker: verify inbound POSTs (shared token or SHA-256 demo signature), record events, expose a tiny JSON API.

No Cargo crates, no `extern fn`, no `ffi/`. Domain logic is pure Windjammer; HTTP lives in adapters.

## Endpoints

| Method | Path | Auth | Behavior |
|--------|------|------|----------|
| `GET` | `/health` | none | `{"ok":true}` |
| `POST` | `/webhook` | token or signature | parse `{event,id}`, append to in-memory log |
| `GET` | `/events` | none | JSON array of recorded events |

Auth (either):

- Header `X-Webhook-Token: <secret>` matching `WEBHOOK_SECRET`
- Header `X-Webhook-Signature: <hex>` where hex is `sha256("${secret}.${body}")` (demo until std HMAC)

## Usage

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj
export WEBHOOK_SECRET=secret
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
```

## Layout

```
src/
  domain/webhook.wj      # auth, parse, event log
  adapters/http_server.wj
  main.wj                # composition root
tests/
  webhook_test.wj
```

## License

MIT OR Apache-2.0
