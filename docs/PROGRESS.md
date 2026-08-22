# Progress

Weekly build health for seed apps and packages.

| Week | Date | Apps / packages green | `extern fn` | Crates via interop | Compiler issues | Notes |
|------|------|----------------------|-------------|--------------------|-----------------|-------|
| 1 | 2026-08-15 | `wj-hello` | 0 | 0 | Multi-file CLI lib+bin (fixed upstream) | Initial skeleton |
| 2 | 2026-08-16 | `wj-hello`, `wj-dotenv` | 0 | 0 | Flat `lib.wj` + `wj test`, `strings::join` Vec, `fs` AsRef path (fixed upstream) | Idiomatic package seed |
| 2 | 2026-08-16 | + `wj-fetch` (domain + adapters) | 0 | 0 | `json.parse` borrow, HTTP status `u16`, `process::exit` path (fixed upstream) | Wave 1 CLI HTTP GET |
| 2 | 2026-08-17 | + `wj-notes-api` (domain CRUD) | 0 | 0 | HashMap::get non-Copy borrow-break `.cloned()` not `.copied()` (fixed upstream) | Wave 1 REST API seed |
| 2 | 2026-08-18 | + `wj-notes-api` (HTTP adapter) | 0 | 0 | Nested `use std::http::*` emitted `use super::HttpMethod` (fixed upstream) | Hexagonal REST: domain routing + `adapters/http_server.wj` |
| 2 | 2026-08-19 | + `wj-config` | 0 | 0 | HashMap variable-key `.get` codegen (deferred; tests use literal keys) | Layered defaults / file / env |
| 2 | 2026-08-20 | + `wj-log` | 0 | 0 | `use std::log::*` aliased `log_mod`; `error()` homonym skipped `&str` (fixed upstream) | Logging helpers over `std::log` |
| 2 | 2026-08-20 | + `wj-cli-args` | 0 | 0 | string slice/index types (use `std::strings`); for-loop elem → owned helper got `&` (fixed upstream) | Argv helpers until clap interop |
| 2 | 2026-08-20 | + `wj-sitegen` | 0 | 0 | multipass: `pub mod` codegen order; match-arm `&` into owned `string` (fixed upstream) | Wave 2 static site: markdown domain + fs adapters; 10 tests green |
| 2 | 2026-08-21 | + `wj-webhook` | 0 | 0 | | Wave 2 webhook worker: token/sha256 auth, event log, HTTP adapter; 7 tests green |
| 2 | 2026-08-21 | + `wj-http-client` | 0 | 0 | `http::post` body auto-borrow (repro + fixed upstream) | Wave 2 GET/POST helpers over `std::http`; 6 tests green |
| 2 | 2026-08-22 | + `wj-json-util` | 0 | 0 | `json::Value`/`Response` type imports; `json::keys`; owned `get`/`get_index` (fixed upstream) | Wave 2 pretty / path get / deep merge; 11 tests green |
| 2 | 2026-08-22 | + `wj-fs-walk` | 0 | 0 | nested match `for` + `Vec::push` (repro in `windjammer/tests/`; clone codegen fixed upstream) | Wave 2 recursive walk; 6 tests green |
| 2 | 2026-08-22 | + `wj-template` | 0 | 0 | — | Wave 2 `{{key}}` render / strict / missing-keys; 8 tests green |
| 2 | 2026-08-22 | + `wj-uuid` (blocked) | 0 | 0 | `random.range`, `crypto.sha1_bytes`, `time.utc_now` (repros in `windjammer/tests/`) | v1/v4/v5 + validation; 16 tests written, pending stdlib |
| 2 | 2026-08-22 | + `wj-semver` | 0 | 0 | — | Wave 2 parse / compare / format; 6 tests green |
| 2 | 2026-08-22 | + `wj-url` | 0 | 0 | user `join` vs `strings.join` name clash (renamed `join_url`; repro queued) | Wave 2 parse / format / join_url / query; 11 tests green |
| 2 | 2026-08-22 | + `wj-base64` (blocked) | 0 | 0 | `encoding.base64_encode_string` / `decode_string` not in runtime exports (repro queued) | Idiomatic API written; 6 tests pending stdlib |
| 2 | 2026-08-22 | + `wj-retry` | 0 | 0 | — | Wave 2 exponential backoff helpers; 4 tests green |

## Weekly checklist

- [ ] No project-local `ffi/` / hand-written `.rs` / `extern fn` in apps
- [ ] CI green on pinned `wj`
- [ ] Changed seeds pass `wj test`
- [ ] [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) updated when production code gains std usage
- [ ] Compiler bugs get failing tests in `windjammer/tests/`
