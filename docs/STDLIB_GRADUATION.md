# Stdlib graduation — what belongs in `std` vs ecosystem packages

Strong stdlib is the adoption lever. Ecosystem packages should **dogfood gaps** and **stay thin wrappers** once `std::*` is wired — not permanently duplicate platform primitives.

Updated 2026-11-02: tip-health table re-measured; `std::crypto` gains HMAC-SHA256 and constant-time compare (compiler branch `wt/std-hmac`).

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

## Tip health (2026-11-02, `wj` 0.50.0 at `next/v0.50.0` 4b18e8c2e)

Every package and app under `packages/*` and `apps/*`, tests run interpreted (`wj test --interpret`) and
compiled (`wj test`) by `scripts/health_sweep.wj`; the table is rendered by `scripts/health_table.wj`. Counts
are tests passed. 61 targets: 53 are green in both modes today (4 packages red interpreted, 4 apps red compiled). Compiled results use the compiler with the one-line
`wj build src` fix described in [findings](findings/README.md) (finding 1); without it every dependency build fails.
Each red cell is a compiler finding with a generic repro in `docs/findings/repros/`; no package carries a shim for it.
`wj-webhook` and its HMAC signing need the `std::crypto` HMAC addition (branch `wt/std-hmac` in the compiler repo).

| Package | Interpreted | Compiled | Notes |
|---|---|---|---|
| `packages/wj-base64` | pass (14) | pass (14) |  |
| `packages/wj-cli-args` | pass (24) | pass (24) |  |
| `packages/wj-compress` | pass (21) | pass (21) |  |
| `packages/wj-config` | **FAIL** (17 pass) | pass (19) | Interpreted: `Some(v) => Some(v), None => None` yields `()` on a miss (finding 2) |
| `packages/wj-cookie` | **FAIL** (17 pass) | pass (20) | Interpreted: same engine finding (finding 2) |
| `packages/wj-cors` | pass (17) | pass (17) |  |
| `packages/wj-cron` | pass (34) | pass (34) |  |
| `packages/wj-csv` | pass (16) | pass (16) |  |
| `packages/wj-dotenv` | **FAIL** (16 pass) | pass (18) | Interpreted: same engine finding (finding 2) |
| `packages/wj-duration` | pass (25) | pass (25) |  |
| `packages/wj-event` | pass (22) | pass (22) |  |
| `packages/wj-fs-walk` | pass (18) | pass (18) |  |
| `packages/wj-gitignore` | pass (12) | pass (12) |  |
| `packages/wj-glob` | pass (28) | pass (28) |  |
| `packages/wj-hash` | pass (10) | pass (10) |  |
| `packages/wj-headers` | pass (20) | pass (20) |  |
| `packages/wj-http-client` | pass (13) | pass (13) |  |
| `packages/wj-inflect` | pass (22) | pass (22) |  |
| `packages/wj-json-doc` | pass (12) | pass (12) |  |
| `packages/wj-json-util` | pass (23) | pass (23) |  |
| `packages/wj-jwt` | pass (12) | pass (12) |  |
| `packages/wj-log` | pass (17) | pass (17) |  |
| `packages/wj-migrate` | pass (25) | pass (25) |  |
| `packages/wj-mime` | pass (24) | pass (24) |  |
| `packages/wj-multipart` | pass (21) | pass (21) |  |
| `packages/wj-path` | pass (20) | pass (20) |  |
| `packages/wj-proc` | pass (8) | pass (8) |  |
| `packages/wj-querystring` | pass (29) | pass (29) |  |
| `packages/wj-rate-limit` | pass (17) | pass (17) |  |
| `packages/wj-regex` | pass (16) | pass (16) |  |
| `packages/wj-retry` | pass (26) | pass (26) | Spin-wait `pause_ms` kept (finding 4) |
| `packages/wj-router` | pass (13) | pass (13) |  |
| `packages/wj-semver` | pass (17) | pass (17) |  |
| `packages/wj-sha` | pass (11) | pass (11) |  |
| `packages/wj-stats` | pass (5) | pass (5) |  |
| `packages/wj-sync` | pass (50) | pass (50) |  |
| `packages/wj-table` | pass (4) | pass (4) |  |
| `packages/wj-template` | pass (23) | pass (23) |  |
| `packages/wj-timefmt` | pass (27) | pass (27) |  |
| `packages/wj-toml` | **FAIL** (26 pass) | pass (30) | Interpreted: same engine finding (finding 2) |
| `packages/wj-url` | pass (22) | pass (22) |  |
| `packages/wj-uuid` | pass (24) | pass (24) |  |
| `packages/wj-validate` | pass (36) | pass (36) |  |
| `packages/wj-yaml` | pass (26) | pass (26) |  |
| `apps/wj-auth-api` | pass (48) | **FAIL** (build) | Compiled: cross-crate `&Vec` call (finding 3) |
| `apps/wj-chat` | pass (20) | pass (20) |  |
| `apps/wj-fetch` | pass (37) | pass (37) | `--timeout` not applied (finding 5) |
| `apps/wj-find` | pass (12) | pass (12) |  |
| `apps/wj-first-hour` | pass (4) | pass (4) |  |
| `apps/wj-form-parse` | pass (12) | pass (12) |  |
| `apps/wj-grep` | pass (38) | **FAIL** (build) | Compiled: `usize` pushed into `Vec<int>` (finding 6) and a loop move (finding 8) |
| `apps/wj-hello` | pass (8) | pass (8) |  |
| `apps/wj-migrate-cli` | pass (27) | pass (27) |  |
| `apps/wj-notes-api` | pass (62) | **FAIL** (build) | Compiled: cross-crate `&Vec` call (finding 3) |
| `apps/wj-pipeline` | pass (11) | pass (11) |  |
| `apps/wj-proxy` | pass (27) | pass (27) |  |
| `apps/wj-scheduler` | pass (43) | pass (43) |  |
| `apps/wj-sitegen` | pass (26) | pass (26) |  |
| `apps/wj-taskflow` | pass (30) | pass (30) |  |
| `apps/wj-todo-cli` | pass (65) | pass (65) |  |
| `apps/wj-webhook` | pass (23) | **FAIL** (build) | Compiled: cross-crate `&Vec` call (finding 3); HMAC calls resolve (needs `wt/std-hmac`) |

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
- **GREEN (tip 21:42):** notes-api P3.666 — `qs_get(…, "pretty"|"encoding"|"q"|"limit")` bare keys (no `.to_string()`); product transpile + cargo gates **3/3**
- **GREEN (tip 23:43):** `wj-json-util` P3.669 — merge overlay demotes `take_field(value: &Value)` / `&overlay` (E0382 fixed)
- **GREEN (tip 19:18+):** sync SharedMap P3.660 (`g.get(&key)`; 49 tests); prior borrow gates csv/hash/regex/mime/json-util/toml/webhook/auth
- Cargo: P3.660 isolate + product SharedMap get **2/2 GREEN**
- querystring **16/16 GREEN** (`set`/`get_or`); `wj-url` **18/18 GREEN** (`origin`/`is_https`); `wj-retry` **9/9 GREEN** (`attempts_remaining`/`default_backoff`); `wj-inflect` **15/15 GREEN** (`title_case`); `wj-json-util` **15/15 GREEN** (P3.669); `wj-log` **9/9 GREEN** (`level_rank`/`is_enabled`); `wj-hash` **5/5 GREEN** (`looks_like_bcrypt`); `wj-csv` **7/7 GREEN** (`row_count`/`column_count`); `wj-yaml` **18/18 GREEN** (`get_str_or`/`has_path`); `wj-fs-walk` **8/8 GREEN** (`count_files`); `wj-dotenv` **has/require GREEN**; `wj-toml` **get_or/has GREEN**; `wj-timefmt` **22/22 GREEN** (`is_equal`/`add_secs`); `wj-validate` **28/28 GREEN** (`require_len_range`); `wj-event` **12/12 GREEN** (`has_pending`); `wj-cookie` **12/12 GREEN** (`has_cookie`/`cookie_count`); `wj-cli-args` **11/11 GREEN** (`positional_count`); `wj-rate-limit` **11/11 GREEN** (`peek_fixed_window`); `wj-cors` **10/10 GREEN** (`format_cors_headers`); `wj-headers` **13/13 GREEN** (`hsts_enabled`); `wj-duration` **15/15 GREEN** (`ms_to_secs`/`secs_to_ms`); `wj-mime` **15/15 GREEN** (`is_html`); `wj-semver` **9/9 GREEN** (`is_equal`); `wj-path` **13/13 GREEN** (`stem`); `wj-compress` **13/13 GREEN** (`content_encoding_header`); `wj-sha` **6/6 GREEN** (`looks_like_sha256_hex`); `wj-glob` **17/17 GREEN** (`match_count`); `wj-base64` **8/8 GREEN** (`is_basic_auth_header`); `wj-jwt` **8/8 GREEN** (`is_bearer_header`); `wj-template` **17/17 GREEN** (`placeholder_count`); `wj-multipart` **11/11 GREEN** (`has_field`); proxy **25/25**
- **GREEN (tip 18:38):** notes-api P3.671 — `qs_get(query.clone(), …)` (no `format!`)
- **Still RED (tip 18:38):** P3.676 — owned HashMap `has` with `Some(_)` → `matches!` + DEFER DROP spawn (E0308); cookie/dotenv use bound arms until tip greens
- **Still RED (tip 18:38):** P3.679 — `while i < 64` literal + substring → `64_usize` / uncast `i`; sha uses `chars` until tip greens

**Do not** invent new ecosystem wrappers for those std rows — thin-wrap existing `wj-*` packages.
