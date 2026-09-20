# Stdlib graduation — what belongs in `std` vs ecosystem packages

Strong stdlib is the adoption lever. Ecosystem packages should **dogfood gaps** and **stay thin wrappers** once `std::*` is wired — not permanently duplicate platform primitives.

Updated 2026-09-19: `std::config` consolidates toml/yaml config; uuid v7 graduated.

## Principle

| In **stdlib** | Stay in **ecosystem** |
|---|---|
| Universal primitives every app needs in week one | Opinionated product shapes, middleware stacks, apps |
| Same API across CLI / HTTP / WASM targets | Domain policies that change per product |
| Must work without Cargo discovery | May compose several std modules |

## Graduate to std (or finish wiring) — P0/P1

| Concern | Ecosystem today | Std target | Why std |
|---|---|---|---|
| Path join / basename / ext | `wj-path` (`join_path`/`basename` → `std::path`; normalize/dirname/extname sugar) | **`std::path`** | Wiring ✅; package keeps extra helpers |
| MIME lookup | `wj-mime` thin-wraps **`std::mime`** | **`std::mime`** | HTTP static files — wiring ✅; charset parity ✅ P3.243 |
| Base64 / hex / URL encode | `wj-base64` thin-wraps **`std::encoding`** | **`std::encoding`** string APIs | Encoding is platform — ✅ graduated |
| UUID v4/v5/v7 | `wj-uuid` (v1/v4/v5/v7 + NIL/MAX; **v4/v7 thin-wrap `std::uuid`**) | **`std::uuid`** (v4 + v7 ✅) | Identity primitive — v5 stays package |
| Random range | (via uuid) | **`std::random.range` → runtime `int_range`** | ✅ wired (WJ `range` name; runtime `int_range`) |
| SHA-1 bytes / SHA-256 hex | `wj-sha` thin | **`std::crypto`** complete wiring | Crypto must be std |
| UTC now / millis / RFC3339 | `wj-timefmt` (structured Rfc3339 + **`now_rfc3339`/`now_epoch_secs` via `std::time`**) | **`std::time`** | Clocks are std; structured parse stays package sugar |
| Human duration `1h30m` | `wj-duration` thin-wraps **`std::time.parse_duration_ms`** / `format_duration_ms` | **`std::time`** | ✅ graduated (P3.391) |
| YAML config | `wj-yaml` thin-wraps **`std::yaml.to_json`**; getters sugar | **`std::yaml`** wiring ✅; empty-input parity ✅ P3.244 | PyYAML-scale ubiquity |
| JWT HS256 | `wj-jwt` thin-wraps **`std::jwt`** | **`std::jwt`** | Auth primitive — ✅ graduated |
| CSV parse/write | `wj-csv` thin-wraps **`std::csv`** parse/write | **`std::csv`** idiomatic API | Data interchange — ✅ graduated |
| TOML/YAML config | `wj-config`: **parse** via `std::config.parse_flat`; merge/resolve local until tip HashMap ownership GREEN | **`std::config`** | Format is impl detail; tip `resolve`/`merge` call-site still RED |
| Form-urlencoded | `wj-querystring` pure WJ over `url_encode` | **`std::encoding.form_parse` / `form_stringify`** | HTTP week-one; RED gate filed |
| Glob match | `wj-glob` pure WJ | **`std::path.glob_match`** | fs walk / sitegen; RED gate filed |
| Absolute URL parse/join | `wj-url` (pure WJ; query sugar local) | **`std::url`** | Week-one HTTP; RED gate filed (`STDLIB_URL_HANDOFF.md`) |

| DB execute/query | migrate smoke via docker | **`std::db`** ergonomic apply path | Persistence |

## Keep as ecosystem packages

| Package | Reason |
|---|---|
| `wj-sync` | Concurrency dogfood; **`std::sync`** channel/Shared/atomic GREEN — package thin-wraps `unbounded`/`bounded`; Pending/Pool stay ecosystem |
| `wj-validate` | App schema DSL; grows with products (zod-like) |
| `wj-rate-limit` | Policy / storage backends vary |
| `wj-headers` / helmet | Opinionated security defaults |
| `wj-cors` | Middleware composition |
| `wj-router` | Can thin-wrap `std::http` routing later; API experiments OK in packages |
| `wj-migrate` | Orchestration + filename conventions; uses `std::db` + `std::fs` |
| `wj-config` / `wj-dotenv` | Layering policy on `std::fs` / env |
| `wj-retry`, `wj-template`, `wj-inflect` | Convenience; not platform |
| `wj-http-client`, `wj-json-util` | Ergonomic veneers over `std::http` / `std::json` |
| `wj-glob` | Until `std::path.glob_match` greens (RED gate filed) |
| `wj-querystring` | Until `std::encoding.form_*` greens (RED gate filed); sugar `get`/`append` stays package |
| `wj-url` | Until `std::url` greens; query_* sugar stays package |
| Apps (`wj-todo-cli`, `wj-proxy`, `wj-pipeline`, …) | Never std |

## Tip health (2026-09-19 local tip `wj` 0.50.0)

| Package | Tests | Notes |
|---|---|---|
| `wj-path`, `wj-base64`, `wj-jwt`, `wj-csv`, `wj-yaml`, `wj-sha`, `wj-mime`, `wj-cron` | ✅ green | Thin-wraps / tip recheck |
| `wj-dotenv` | ✅ green | P3.311 tip GREEN |
| `wj-duration` | ✅ green | Thin-wrap `std::time` parse/format duration ms |
| `wj-validate` | ✅ green | P3.314 tip GREEN — 27 tests |
| `wj-cli-args`, `wj-compress`, `wj-glob` | ✅ green | tip cargo-check |
| `wj-sync` + `wj-pipeline` | ✅ tip green | 49 tests; ≤1.2× Rust; `std::sync` thin-wrap |
| `wj-uuid` | ✅ green | tip cargo-check |
| `wj-timefmt` | ✅ tip green | 20 tests (P3.329) |
| `wj-semver` | ✅ tip green | 6 tests — owned→demoted `&str` borrow (eco gate) |
| `wj-toml` | ✅ tip green | 17 tests — demoted key `.to_string()` into owned tuple push |
| `wj-config` | ✅ tip green | parse via `std::config`; merge/resolve package-local (tip HashMap ownership) |
| `wj-url` | ✅ tip green | form-decode query_*; authority substring scan; tip owns `std::url` + int unify |
| `wj-multipart` | ✅ tip green | split_once scanners; owned `parse`/`parse_multipart`; dogfooded by `wj-form-parse` |

## Migration path (once gates go green)

1. Fix wiring so idiomatic `use std::X` compiles and runs.
2. Reimplement ecosystem package as **thin re-export / sugar** over std (or deprecate).
3. Keep package name stable for apps already importing `wj-*`.

## What the other agent should prioritize

See `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` and failing `bug_std_*` tests. Current tip RED (beta blockers):

1. `std::config.resolve` / `merge` owned-HashMap call sites (`STDLIB_CONFIG_HANDOFF.md`)
2. `std::encoding.form_parse` / `form_stringify` (`STDLIB_FORM_HANDOFF.md`)
3. `std::path.glob_match` (same handoff)
4. `strings.contains` owned interpolated needle demotion
5. `std::url` parse/format/join + int/usize loop assign unify (`STDLIB_URL_HANDOFF.md`)

**Do not** invent new ecosystem wrappers for the P0 rows above — write failing std repros and fix std/runtime.
