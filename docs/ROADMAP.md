# Roadmap

Catalog of seed apps and packages for common Windjammer application work.

## Design principles

- Idiomatic Windjammer for business logic
- Standard library first; rust-interop for a small set of Cargo crates when needed
- No project-local `ffi/` or hand-edited generated Rust
- Hexagonal layout in apps (domain / adapters / `main.wj`)
- Tests in Windjammer (`tests/*_test.wj`)

## Wave 1

| Name | Kind | Use case | Dependencies |
|---|---|---|---|
| `wj-hello` | app | Build-path smoke test | std only |
| `wj-dotenv` | package | `.env` / `KEY=VALUE` loading | `std::fs`, `std::strings` |
| `wj-fetch` | app | CLI HTTP GET + JSON | `std::http` / `std::cli` / `std::json`, or clap + ureq later |
| `wj-notes-api` | app | Small CRUD REST API | HTTP std + in-memory; optional `std::db` |
| `wj-config` | package | Layered config (file + env + defaults) | builds on `wj-dotenv` |
| `wj-log` | package | Logging helpers over `std::log` | std |
| `wj-cli-args` | package | Argv helpers until clap interop | std / later clap |

## Wave 2

| Name | Kind | Use case |
|---|---|---|
| `wj-sitegen` | app | Static site / docs generator |
| `wj-webhook` | app | Bot / webhook worker |
| `wj-http-client` | package | Idiomatic GET/POST JSON client |
| `wj-json-util` | package | Pretty-print, path get, merge |
| `wj-fs-walk` | package | Recursive directory walk |
| `wj-template` | package | Simple string templates |
| `wj-uuid` | package | UUID v4 |
| `wj-semver` | package | Parse / compare semver |
| `wj-url` | package | Parse / join URLs |
| `wj-base64` | package | Encode / decode |
| `wj-sha` | package | SHA-256 hex digest |
| `wj-retry` | package | Retry with backoff |
| `wj-timefmt` | package | RFC3339 format / parse |
| `wj-migrate` | package | Ordered SQL migrations |
| `wj-router` | package | HTTP path match + params |
| `wj-cors` | package | CORS header helpers |
| `wj-todo-cli` | app | Local JSON file CRUD CLI |
| `wj-proxy` | app | Small reverse proxy / request logger |

## Wave 3 (rust-interop allowlist)

Prefer one crate per concern after Wave 1 apps are stable:

| Concern | Preferred crates |
|---|---|
| CLI args | `clap` |
| HTTP client | `ureq` or `reqwest` |
| JSON | `serde` / `serde_json` (or `std::json`) |
| SQLite | `rusqlite` |
| Logging | `env_logger` / `tracing` (later) |
| Async | `tokio` when needed |

## Out of scope here

Browser/WASM product apps, OS kernels, full SQL engines, game engines, and wgpu wrappers belong in other Windjammer repositories.
