# Capability table

Replaces the old weekly log. Levels:

- **Wrapper**: thin layer over a `std::*` module.
- **Subset**: own implementation of a documented subset of the format.
- **Domain**: app-level logic with its own rules.

`pub fns` is counted from the source (`pub fn` in `src/`). The test columns are measured: tests passing under
`wj test --interpret` and `wj test` (2026-11-02, tip `wj`; a red cell shows the finding, see
[STDLIB_GRADUATION.md](STDLIB_GRADUATION.md) and [findings](findings/README.md)).

| Package | Level | pub fns | tests (interpreted / compiled) |
|---|---|---|---|
| wj-base64 | Wrapper + Basic-auth header helpers | 18 | 14 / 14 |
| wj-cli-args | Subset (flags, positionals) | 27 | 24 / 24 |
| wj-compress | Wrapper (Accept-Encoding negotiation, gzip) | 22 | 21 / 21 |
| wj-config | Subset (layered config) | 23 | 19 / 19 |
| wj-cookie | Subset (Cookie / Set-Cookie) | 23 | 20 / 20 |
| wj-cors | Subset | 17 | 17 / 17 |
| wj-cron | Subset (cron expressions) | 11 | 34 / 34 |
| wj-csv | Subset | 17 | 16 / 16 |
| wj-dotenv | Subset | 18 | 18 / 18 |
| wj-duration | Subset | 25 | 25 / 25 |
| wj-event | In-process event bus | 24 | 22 / 22 |
| wj-fs-walk | Wrapper | 22 | 18 / 18 |
| wj-glob | Subset | 19 | 28 / 28 |
| wj-hash | Wrapper (bcrypt) | 10 | 10 / 10 |
| wj-headers | Security-header presets | 21 | 20 / 20 |
| wj-http-client | Wrapper (sync, no middleware) | 24 | 13 / 13 |
| wj-inflect | Subset (case, slug) | 17 | 22 / 22 |
| wj-json-util | Wrapper | 18 | 23 / 23 |
| wj-jwt | Wrapper (HS256) | 15 | 12 / 12 |
| wj-log | Subset | 18 | 17 / 17 |
| wj-migrate | Domain (SQL migration planning) | 25 | 25 / 25 |
| wj-mime | Lookup table | 35 | 24 / 24 |
| wj-multipart | Subset | 22 | 21 / 21 |
| wj-path | Subset | 21 | 20 / 20 |
| wj-querystring | Wrapper + helpers | 27 | 29 / 29 |
| wj-rate-limit | Fixed-window limiter | 20 | 17 / 17 |
| wj-regex | Wrapper | 19 | 16 / 16 |
| wj-retry | Backoff policy | 30 | 27 / 27 |
| wj-router | Path matching | 12 | 13 / 13 |
| wj-semver | Subset | 20 | 17 / 17 |
| wj-sha | Wrapper (SHA-256 hex) | 15 | 11 / 11 |
| wj-sync | Channels, shared state, pool (generic) | 57 | 50 / 50 |
| wj-template | Subset (`{{var}}`) | 18 | 23 / 23 |
| wj-timefmt | Subset (RFC 3339) | 25 | 27 / 27 |
| wj-toml | Subset (flat keys, sections) | 22 | 30 / 30 |
| wj-url | Subset | 19 | 22 / 22 |
| wj-uuid | v1/v4/v5/v7 | 23 | 24 / 24 |
| wj-validate | Field validators | 21 | 36 / 36 |
| wj-yaml | Wrapper (YAML to JSON) | 17 | 26 / 26 |
| wj-archive | Tar and zip read/write, DEFLATE (Windjammer-native) | 21 | 17 / 17 |
| wj-decimal | Fixed-point decimals, rounding modes | 20 | 13 / 13 |
| wj-markdown | CommonMark subset to HTML, escaping on | 11 | 7 / 7 |
| wj-metrics | Counters, gauges, histograms, Prometheus text | 16 | 11 / 11 |
| wj-ws | WebSocket frames, messages, handshake | 22 | 16 / 16 |
| wj-cache | LRU + TTL cache, Redis protocol codec | 22 | 14 / 14 |
| wj-cli-ui | Colour, text width, progress, prompts | 51 | 12 / 12 |
| wj-oauth | OAuth 2.0 / OIDC: PKCE, requests, ID token validation | 27 | 12 / 12 |
| wj-smtp | MIME messages and SMTP planning | 21 | 17 / 17 |
| wj-gitignore | Subset (.gitignore matching) | 3 | 12 / 12 |
| wj-json-doc | Order-preserving JSON documents | 24 | 12 / 12 |
| wj-proc | Wrapper (process, git, CPU timing) | 12 | 8 / 8 |
| wj-stats | Descriptive statistics | 8 | 5 / 5 |
| wj-table | Terminal and markdown tables | 10 | 4 / 4 |


Apps are demonstration apps; measured tests (interpreted / compiled):

| App | tests (interpreted / compiled) |
|---|---|
| wj-auth-api | 48 / 48 |
| wj-chat | 20 / 20 |
| wj-fetch | 37 / 37 |
| wj-find | 12 / 12 |
| wj-first-hour | 4 / 4 |
| wj-form-parse | 12 / 12 |
| wj-grep | 38 / 38 |
| wj-hello | 8 / 8 |
| wj-migrate-cli | 27 / 27 |
| wj-notes-api | 62 / 62 |
| wj-pipeline | 11 / 11 |
| wj-proxy | 27 / 27 |
| wj-scheduler | 43 / 43 |
| wj-sitegen | 26 / 26 |
| wj-taskflow | 30 / 30 |
| wj-todo-cli | 65 / 65 |
| wj-webhook | 23 / 23 |
