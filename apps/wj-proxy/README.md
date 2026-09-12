# wj-proxy

Small reverse proxy and request logger for Windjammer dogfooding.

## What it does

- Forwards GET/POST/PUT/PATCH/DELETE/HEAD to a configured upstream (`PROXY_UPSTREAM`)
- Optional path prefix strip (`PROXY_STRIP_PREFIX`)
- Fixed-window rate limiting per client key (`X-Forwarded-For` or `anonymous`) via `wj-rate-limit`
- Ring-buffer request log capped by `PROXY_LOG_MAX` (default 1000; `0` = unlimited)
- Security headers on every response via `wj-headers`
- Local endpoints: `GET /health`, `GET /logs` (JSON request log), `DELETE /logs` (clear log buffer, returns `{"cleared":N}`), `GET /stats` (log count + rate-limit config)

## Layout (hexagonal)

```
src/
  domain/     config, URL join, rate limit, logging (pure, tested)
  adapters/   HTTP forward + server wiring
  main.wj     composition root
tests/
  url_join_test.wj
  proxy_test.wj
  config_test.wj
  stats_test.wj
```

## Run

```bash
cd apps/wj-proxy
PROXY_UPSTREAM=http://127.0.0.1:9000 PORT=8088 wj build --release src
# then run the generated binary from build/
```

## Test

```bash
unset CARGO_TARGET_DIR
wj test
```

## Environment

| Variable | Default | Purpose |
|---|---|---|
| `PROXY_UPSTREAM` | `http://127.0.0.1:9000` | Upstream base URL |
| `PROXY_STRIP_PREFIX` | *(empty)* | Strip prefix before forward |
| `PROXY_RATE_LIMIT` | `100` | Requests per window per client |
| `PROXY_WINDOW_MS` | `60000` | Rate limit window (ms) |
| `PROXY_LOG_MAX` | `1000` | Max request log entries (`0` = unlimited) |
| `PORT` | `8088` | Listen port |

## Packages dogfooded

| Package | Role |
|---|---|
| `wj-rate-limit` | Fixed-window per-key quota + `Retry-After` / `X-RateLimit-*` headers |
| `wj-headers` | Default security header block (`X-Frame-Options`, `X-Content-Type-Options`, …) |

Also uses `std::http`, `std::json`, `std::env`, `std::strings`.
