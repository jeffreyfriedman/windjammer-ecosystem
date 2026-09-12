# Windjammer Ecosystem

Apps and packages that show what everyday Windjammer development looks like: CLIs, small APIs, config libraries, and similar projects built with Windjammer’s standard library plus a few Cargo crates via rust-interop.

## Goals

- Idiomatic Windjammer for application logic
- At most a handful of Cargo dependencies (for example `clap`, `ureq`/`reqwest`, `serde_json`, one DB or file crate)
- No project-local `ffi/` or hand-written Rust in app/package sources
- Clear, tested examples others can copy

## Example projects

| Kind | Project | Status |
|---|---|---|
| Std-only CLI | [`apps/wj-hello`](apps/wj-hello) | Available |
| `.env` loader | [`packages/wj-dotenv`](packages/wj-dotenv) | Available |
| Layered config | [`packages/wj-config`](packages/wj-config) | Available |
| Logging helpers | [`packages/wj-log`](packages/wj-log) | Available |
| Argv helpers | [`packages/wj-cli-args`](packages/wj-cli-args) | Available |
| HTTP CLI | [`apps/wj-fetch`](apps/wj-fetch) | Available |
| CRUD API | [`apps/wj-notes-api`](apps/wj-notes-api) | Available (router + CORS + headers + rate limit) |
| Auth API | [`apps/wj-auth-api`](apps/wj-auth-api) | Available (register/login/me + CORS + gzip + HTML) |
| Static site generator | [`apps/wj-sitegen`](apps/wj-sitegen) | Available |
| Webhook worker | [`apps/wj-webhook`](apps/wj-webhook) | Available (CORS + headers + validate) |
| Reverse proxy | [`apps/wj-proxy`](apps/wj-proxy) | Available (rate limit + security headers) |
| HTTP client helpers | [`packages/wj-http-client`](packages/wj-http-client) | Available |
| JSON utilities | [`packages/wj-json-util`](packages/wj-json-util) | Available |
| Directory walk | [`packages/wj-fs-walk`](packages/wj-fs-walk) | Available |
| String templates | [`packages/wj-template`](packages/wj-template) | Available |
| UUID v1/v4/v5 | [`packages/wj-uuid`](packages/wj-uuid) | Available |
| Semver | [`packages/wj-semver`](packages/wj-semver) | Available |
| URL parse / join | [`packages/wj-url`](packages/wj-url) | Available |
| Base64 | [`packages/wj-base64`](packages/wj-base64) | Available |
| Retry / backoff | [`packages/wj-retry`](packages/wj-retry) | Available |
| SHA-256 hex | [`packages/wj-sha`](packages/wj-sha) | Available |
| CORS helpers | [`packages/wj-cors`](packages/wj-cors) | Available |
| HTTP path router | [`packages/wj-router`](packages/wj-router) | Available |
| Path helpers | [`packages/wj-path`](packages/wj-path) | Available |
| Cookie headers | [`packages/wj-cookie`](packages/wj-cookie) | Available |
| Duration parse/format | [`packages/wj-duration`](packages/wj-duration) | Available |
| Field validation | [`packages/wj-validate`](packages/wj-validate) | Available |
| Glob matching | [`packages/wj-glob`](packages/wj-glob) | Available |
| RFC3339 time | [`packages/wj-timefmt`](packages/wj-timefmt) | Available |
| MIME types | [`packages/wj-mime`](packages/wj-mime) | Available |
| YAML subset | [`packages/wj-yaml`](packages/wj-yaml) | Available |
| Rate limiting | [`packages/wj-rate-limit`](packages/wj-rate-limit) | Available |
| Security headers | [`packages/wj-headers`](packages/wj-headers) | Available |
| SQL migrations | [`packages/wj-migrate`](packages/wj-migrate) | Available (domain + apply + status) |
| Case / slug | [`packages/wj-inflect`](packages/wj-inflect) | Available |
| Query string | [`packages/wj-querystring`](packages/wj-querystring) | Available |
| In-process events | [`packages/wj-event`](packages/wj-event) | Available |
| Multipart form-data | [`packages/wj-multipart`](packages/wj-multipart) | Available |
| JWT HS256 | [`packages/wj-jwt`](packages/wj-jwt) | Available |
| CSV parse/write | [`packages/wj-csv`](packages/wj-csv) | Available |
| TOML config subset | [`packages/wj-toml`](packages/wj-toml) | Available |
| Password hashing | [`packages/wj-hash`](packages/wj-hash) | Available |
| Compress negotiate | [`packages/wj-compress`](packages/wj-compress) | Available |
| Regex | [`packages/wj-regex`](packages/wj-regex) | Available |
| Todo CLI | [`apps/wj-todo-cli`](apps/wj-todo-cli) | Available (validate + JSON list + env config) |
| Migrate CLI | [`apps/wj-migrate-cli`](apps/wj-migrate-cli) | Available (apply + status via `wj-migrate`) |

See [docs/ROADMAP.md](docs/ROADMAP.md) for the full package catalog, [docs/CROSS_ECOSYSTEM_TOP100.md](docs/CROSS_ECOSYSTEM_TOP100.md) for the cross-registry gap analysis, and [docs/STDLIB_GRADUATION.md](docs/STDLIB_GRADUATION.md) for which packages should move into `std::*`.

## Layout

```
apps/        # Binaries
packages/    # Libraries
docs/        # Roadmap, stdlib coverage, progress
```

## Building

```bash
unset CARGO_TARGET_DIR
cd /path/to/windjammer && cargo build --release

export WJ=/path/to/windjammer/target/release/wj
cd apps/wj-hello
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Contributor guidelines: [AGENTS.md](AGENTS.md).  
Stdlib usage across seeds: [docs/STDLIB_COVERAGE.md](docs/STDLIB_COVERAGE.md).

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
