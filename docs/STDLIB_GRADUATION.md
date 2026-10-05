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
| TOML/YAML config | `wj-config`: **parse** via `std::config.parse_flat`; `resolve` tip GREEN (p3515 cargo-check) | **`std::config`** | Format is impl detail; package keeps layering sugar |
| Form-urlencoded | `wj-querystring` thin-wraps **`std::encoding.form_parse` / `form_stringify`** (**14/14 GREEN** tip p3520) | **`std::encoding.form_*`** | HTTP week-one; wrap + `?` strip ✅ |
| Glob match | `wj-glob.is_match` thin-wraps **`std::path.glob_match`** (14 tests tip p3515) | **`std::path.glob_match`** | fs walk / sitegen; package keeps `filter` |
| Absolute URL parse/join | `wj-url.join_url` thin-wraps **`std::url.join`**; local `Url` kept (**P3.533 product 17/17 GREEN** p3505) | **`std::url`** | Week-one HTTP; query_* sugar stays package |

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
| `wj-glob` | `is_match` thin-wraps `std::path.glob_match`; `filter` stays sugar |
| `wj-querystring` | Sugar `get`/`append` over tip-green `encoding.form_*` |
| `wj-url` | Query_* sugar over tip-green `std::url` |
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
| `wj-url` | ✅ tip green | 17 tests on p3505 (P3.533 local `Url`; `join_url` → `std::url.join`) |
| `wj-querystring` | ✅ tip green | 14/14 thin-wrap `form_*` (P3.535 `?` strip GREEN) |
| `wj-multipart` | ✅ tip green | split_once scanners; owned `parse`/`parse_multipart`; dogfooded by `wj-form-parse` |

## Migration path (once gates go green)

1. Fix wiring so idiomatic `use std::X` compiles and runs.
2. Reimplement ecosystem package as **thin re-export / sugar** over std (or deprecate).
3. Keep package name stable for apps already importing `wj-*`.

## What the other agent should prioritize

See `windjammer/tests/STDLIB_ADOPTION_QUEUE.md`. Tip p3515 (18:31) **wiring cargo-check GREEN**: `encoding.form_parse` / `form_stringify`, `path.glob_match`, `url.parse` / `join`, `config.resolve`.

Still open (2026-10-04 tip shared cache 20:06 dogfood):
- **GREEN:** form-parse/find/sitegen/pipeline/toml; fetch 31
- **GREEN (tip e6aeefa3):** cron 30; proxy 25; notes-api 62; scheduler 37 (after regen cron build/); P3.644 sync pool index; cargo P3.649–651 **3/3**
- **GREEN (tip 20:06):** todo-cli P3.642 — first `decode_store(snapshot.clone())`; **60/60** tests (regen path-dep `wj-validate/build` after prune)
- **Still RED (tip 20:06):** notes-api P3.666 — `qs_get(…, "pretty".to_string())` into demoted `&str` key (P3.486 regression; 4 E0308)
- **GREEN (tip 19:18+):** sync SharedMap P3.660 (`g.get(&key)`; 49 tests); prior borrow gates csv/hash/regex/mime/json-util/toml/webhook/auth
- Cargo: P3.660 isolate + product SharedMap get **2/2 GREEN**
- querystring **14/14 GREEN**; `wj-url` 17/17 GREEN; `wj-cookie` **10/10** (`get_cookie` / `session_cookie`); `wj-event` **11/11**; `wj-cli-args` **10/10**; `wj-rate-limit` **10/10** (`rate_limit_header_lines`); proxy **25/25**

**Do not** invent new ecosystem wrappers for those std rows — thin-wrap existing `wj-*` packages.
