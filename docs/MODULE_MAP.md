# Module map

## Layers (dependencies point down only)

1. `apps/<name>/src/main.wj`: composition root. Wires adapters to domain. May use anything below.
2. `apps/<name>/src/adapters/`: CLI, HTTP, DB and filesystem boundary. May use `domain/` and `std::*`.
3. `apps/<name>/src/domain/`: business rules in pure Windjammer. May use packages and `std::strings`/collections only; no IO.
4. `packages/<name>/src/lib.wj`: reusable library. May use `std::*` and, if declared in its `wj.toml`, other packages. Never apps.

## Packages

| Package | Responsibility | Depends on |
|---|---|---|
| wj-base64 | Base64 and Basic-auth header helpers | std |
| wj-cli-args | argv flags and positionals | std |
| wj-compress | Accept-Encoding negotiation, gzip | std |
| wj-config | Layered config lookup | std |
| wj-cookie | Cookie / Set-Cookie parse and format | std |
| wj-cors | CORS decisions and headers | std |
| wj-cron | Cron expression parse and match | std |
| wj-csv | CSV parse and write | std |
| wj-dotenv | `.env` parsing | std |
| wj-duration | Duration parse and format | std |
| wj-event | In-process event bus | std |
| wj-fs-walk | Directory traversal | std |
| wj-glob | Glob matching | std |
| wj-hash | Password hashing | std |
| wj-headers | Security header presets | std |
| wj-http-client | HTTP result helpers and status checks | std |
| wj-inflect | Case conversion, slugs | std |
| wj-json-util | JSON pretty-print and helpers | std |
| wj-jwt | JWT HS256 | std |
| wj-log | Tagged log helpers | std |
| wj-migrate | SQL migration planning and status | std |
| wj-mime | MIME lookup | std |
| wj-multipart | multipart/form-data parse | std |
| wj-path | Path join and parts | std |
| wj-querystring | Query string parse and build | std |
| wj-rate-limit | Fixed-window limiter | std |
| wj-regex | Regex wrapper | std |
| wj-retry | Backoff and retry policy | std |
| wj-router | Route matching | std |
| wj-semver | Version parse and compare | std |
| wj-sha | SHA-256 hex | std |
| wj-sync | Channels, shared state, pools | std |
| wj-template | Placeholder rendering | std |
| wj-timefmt | RFC 3339 time | std |
| wj-toml | TOML subset parser | std |
| wj-url | URL parse and join | std |
| wj-uuid | UUID generation and validation | std |
| wj-validate | Field validators | std |
| wj-yaml | YAML to JSON | std |

Rule of thumb: a package that needs another package lists it in its own `wj.toml`; apps list
every package they import (see `docs/COMPILER_WORKAROUNDS.md` for the path-dependency caveat).
Use `rg '^use wj_' packages/*/src` to check actual cross-package imports.
