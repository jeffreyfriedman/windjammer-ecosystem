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
| 2 | 2026-08-22 | + `wj-sha` | 0 | 0 | — | Wave 2 SHA-256 hex over `std::crypto`; 4 tests green |
| 2 | 2026-08-22 | + `wj-cors` | 0 | 0 | — | Wave 2 CORS allow-list / preflight helpers; 7 tests green |
| 2 | 2026-08-22 | + `wj-router` | 0 | 0 | — | Wave 2 `:param` path match; 7 tests green |
| 2 | 2026-08-22 | + `wj-path` | 0 | 0 | — | Pareto P0 path helpers (`join_path` / normalize / basename / dirname / extname); 10 tests green |
| 2 | 2026-08-22 | + `wj-cookie` | 1 | 0 | `HashMap.get("lit")` after Result match; `for-in` HashMap post-loop drop (repros queued) | Cookie / Set-Cookie parse+format; 8 tests green |
| 2 | 2026-08-22 | + `wj-duration` | 0 | 0 | — | Parse/format `1h30m` ↔ ms; 12 tests green |
| 2 | 2026-08-23 | + `wj-validate` | 2 | 0 | read-only helper ownership; `substring` int→usize (repros queued) | Field validators; 16 tests green |
| 2 | 2026-08-23 | + `wj-glob` | 1 | 0 | loop reuse read-only `pattern` in `filter` (repro queued) | `*`/`?` segment globs; 9 tests green |
| 2 | 2026-08-23 | + `wj-timefmt` | 0 | 0 | — | Pure RFC3339 Zulu parse/format; 6 tests green |
| 2 | 2026-08-23 | + `wj-mime` | 0 | 0 | `std::mime` wiring (repro queued); avoid module `const string` (codegen `&str`) | Pure WJ mime lookup + predicates; 12 tests green |
| 2 | 2026-08-23 | docs | — | — | — | Added `CROSS_ECOSYSTEM_TOP100.md` (npm/PyPI/crates/Go gap analysis) |
| 2 | 2026-08-23 | + `wj-yaml` | 2 | 0 | recursive owned Vec call-site borrow; Vec[int] index (repros queued) | Pure WJ YAML subset + path getters; 12 tests green |
| 2 | 2026-08-23 | + `wj-rate-limit` | 0 | 0 | — | Fixed-window buckets + Retry-After / X-RateLimit-*; 8 tests green |
| 2 | 2026-08-23 | + `wj-headers` | 0 | 0 | — | Helmet-style secure defaults; 8 tests green |
| 2 | 2026-08-23 | + `wj-migrate` | 1 | 0 | `vec.len() - int` loop bound usize/i64 (repro queued) | Parse/sort/pending migrations + schema SQL; 10 tests green |
| 2 | 2026-08-23 | + `wj-inflect` | 0 | 0 | — | snake/camel/pascal/slugify; 8 tests green |
| 2 | 2026-08-23 | `wj-migrate` smoke | 0 | 0 | — | Postgres smoke via `platform-db`/`wj_migrate_smoke` + `scripts/smoke_postgres.sh` |
| 2 | 2026-08-23 | + `wj-todo-cli` | 2 | 0 | string ordinal compare; HashMap.insert if-arm Option (repros queued) | Hexagonal todo CLI; 14 tests green |
| 2 | 2026-08-23 | stdlib focus | — | — | New `bug_std_*` for yaml/jwt/uuid/path/csv/db/time/sha256 | Added `STDLIB_GRADUATION.md` + `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` |
| 2 | 2026-08-24 | + `wj-proxy` | 0 | 0 | ownership/type codegen in adapters (fixed in `.wj`) | Hexagonal reverse proxy; 9 tests |
| 2 | 2026-08-24 | + `wj-querystring` | 0 | 0 | `Vec::push((key, ""))` empty literal (repro queued) | parse/stringify/get/get_all; 12 tests green |
| 2 | 2026-08-24 | + `wj-event` | 0 | 0 | — | Queue-based event bus + wildcard patterns; 8 tests |
| 2 | 2026-08-25 | + `wj-multipart` | 0 | 0 | reused string after owned call; demoted `&str` auto-borrow (repros queued) | form-data parse/format; 9 tests green |
| 2 | 2026-08-25 | unblock `wj-base64` | 0 | 0 | `Vec<u8>` push int→u8 (repro queued) | 6 tests green over `std::encoding` |
| 2 | 2026-08-25 | unblock `wj-uuid` | 0 | 0 | module const→owned formal (repro queued); `sha1_bytes` now returns `Vec` | 16 tests green |
| 2 | 2026-08-25 | + `wj-jwt` | 0 | 0 | — | HS256 sign/verify over `std::jwt`; 5 tests green |
| 2 | 2026-08-25 | + `wj-csv` | 0 | 0 | `csv.write` via user `fn write` clone-without-borrow (repro queued) | parse via `std::csv`; pure WJ write; 6 tests green |
| 2 | 2026-08-25 | + `wj-toml` | 0 | 0 | — | TOML config subset → flat map; 8 tests green |
| 2 | 2026-09-11 | deepen `wj-toml` | 0 | 0 | — | Single-line arrays → compact JSON; 12 tests green |
| 2 | 2026-08-26 | deepen `wj-timefmt` | 0 | 0 | — | offsets + epoch + compare; 18 tests green |
| 2 | 2026-08-26 | + `wj-hash` | 0 | 0 | — | bcrypt hash/verify over `std::crypto`; 4 tests green |
| 2 | 2026-08-26 | `wj-csv` → std write | 0 | 0 | write-homonym gate green | thin `std::csv` parse/write |
| 2 | 2026-08-26 | + `wj-compress` | 0 | 0 | `while int < strings.len` (repro queued) | Accept-Encoding + gzip codecs; 10 tests green |
| 2 | 2026-08-26 | + `wj-regex` | 0 | 0 | — | Thin `std::regex` wrappers; 9 tests green |
| 2 | 2026-08-27 | deepen `wj-url` | 0 | 0 | demoted `string` reuse / owned formal `&` (repros in `bug_match_none_arm_string_after_split_test`) | `query_has` / `query_remove` / `with_query` + join keeps query/fragment; 15 tests green |
| 2 | 2026-08-27 | `wj-querystring` → `std::encoding` | 0 | 0 | — | `%HH` / form `+` via `url_encode`/`url_decode`; 14 tests green |
| 2 | 2026-08-27 | deepen `wj-glob` | 0 | 0 | — | recursive `**` segment match; 14 tests green |
| 2 | 2026-08-29 | deepen `wj-template` | 0 | 0 | — | `render_with_defaults`, `escape_html`, `render_html`, trimmed placeholders; 13 tests green |
| 2 | 2026-08-29 | + `wj-auth-api` | 0 | 0 | cross-crate owned/borrow forwarder (`bug_app_cross_crate_owned_forwarder_emits_borrow_test`); domain uses `std::crypto`/`jwt`/`compress` for hash/JWT/gzip body until green | Hexagonal auth API: register/login/me + CORS + gzip + HTML welcome; 10 tests green |
| 2 | 2026-08-29 | deepen `wj-notes-api` | 0 | 0 | HashMap field `.get(i64)` auto-borrow (`bug_hashmap_field_get_i64_key_auto_borrow_test`); cross-crate `WindowBucket` Serialize on app struct; demoted string after OPTIONS borrow | Dogfood `wj-router`/`wj-cors`/`wj-headers`/`wj-rate-limit`; health + CORS + 429; 23 tests green |
| 2 | 2026-09-02 | deepen `wj-proxy` | 0 | 0 | — | `PROXY_LOG_MAX` ring buffer trims oldest entries; config + proxy tests; **21/21 tests green** |
| 2 | 2026-09-02 | deepen `wj-proxy` | 0 | 0 | read-only helper + owned `Vec` param → owned `self` (P3.221 repro; stats inlined in `handle_local`) | `DELETE /logs` clears buffer + `cleared` count; `bind_addr` var for `server_serve`; **19/19 tests green** |
| 2 | 2026-08-30 | deepen `wj-proxy` | 0 | 0 | — | hexagonal `domain/config.wj`, `GET /stats`, config + stats tests; 14 tests green |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | `Vec::len()` usize vs `int` tuple (workaround: `pending + done`) | `stats` / `stats --json`; `import --merge` run test; `count_todos` in query; **54/54 tests green** |
| 2 | 2026-09-02 | deepen `wj-todo-cli` | 0 | 0 | `match` in `if` without `return` on `Err` arm (E0308); `todos_from_json(body)` emits `&body`; string literal at `rename` call site | `edit <id> <title>` command; `TodoStore::rename`; **60/60 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | `json.is_array`/`json.len` owned `Value` borrow (P3.214 repro; `[` prefix + index loop workaround) | `import` / `import --merge`; `todos_from_json` + `merge_from`; **48/48 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | — | `export` / `export --out` / filters; `todos_to_json` in codec; store preserved via snapshot; **42/42 tests green** |
| 2 | 2026-09-01 | deepen `wj-todo-cli` | 0 | 0 | — | `clear` / `clear --done` on `TodoStore`; **35/35 tests green** |
| 2 | 2026-09-01 | deepen `wj-sitegen` | 0 | 0 | — | `robots.txt` via `render_robots` when `SITEGEN_BASE_URL` set; **23/23 tests green** |
| 2 | 2026-09-01 | deepen `wj-sitegen` | 0 | 0 | cross-crate `join_url`/`render` owned→borrow multipass (`bug_multipass_cross_crate_join_url_owned_base_test`, `bug_multipass_cross_crate_render_owned_template_helper_test`) | sitemap + RSS via `domain/feeds.wj`, `SITEGEN_BASE_URL`/`SITEGEN_TITLE`, inline template + `join_page_url`; **21/21 tests green** |
| 2 | 2026-08-30 | deepen `wj-sitegen` | 0 | 0 | app multipass cross-crate `own()` forwarder still emits `&` (`bug_app_multipass_cross_crate_owned_forwarder_module_file_test`) | recursive `*.md` walk, `SITEGEN_SRC`/`SITEGEN_OUT`, `wj-template` escape + layout; 14 tests green |
| 2 | 2026-09-01 | deepen `wj-fetch` | 0 | 0 | — | `-s` / `--silent` body-only stdout; silent + `--output` writes file with no stdout; **28/28 tests green** |
| 2 | 2026-09-02 | deepen `wj-fetch` | 0 | 1 | `use std::net::Request` import not codegen'd (P3.219); adapter still uses `std::http::get` | `--timeout=N` / `FETCH_TIMEOUT_SECS` parsing on `FetchRequest`; **31/31 tests green** |
| 2 | 2026-09-01 | deepen `wj-fetch` | 0 | 0 | `match Some(string)` / `None => ""` E0308 (P3.217 repro; mut reassignment workaround) | `--output` / `-o` write body to file; `resolve_fetch_output` in format; **24/24 tests green** |
| 2 | 2026-08-31 | deepen `wj-fetch` | 0 | 0 | `std::async_runtime` import RED — `wj-retry::pause_ms` uses `std::time` spin-wait | Backoff pause between retries; **20/20 tests green** (wj-retry 5/5) |
| 2 | 2026-08-31 | deepen `wj-todo-cli` | 0 | 0 | owned struct field → fn `string` param emits `.clone()` not borrow | `list --pending`/`--done`, sorted by id, `domain/query.wj`; **28/28 tests green** |
| 2 | 2026-08-30 | deepen `wj-proxy` | 0 | 0 | — | Dogfood `wj-rate-limit` + `wj-headers`; security + rate-limit headers; 11 tests green |
| 2 | 2026-08-31 | deepen `wj-auth-api` | 0 | 0 | `HttpMethod` test/lib split (same as webhook); string public port | hexagonal `domain/config.wj`, internal `HttpMethod` routing, `ServerResponse` helpers; **11/11 tests green** |
| 2 | 2026-08-31 | deepen `wj-notes-api` | 0 | 0 | HashMap i64 `.get` borrow; spurious `use Note;` same-file Serialize; `store.get(id)` → `get(&id)` at api boundary | hexagonal `domain/config.wj` + `note.wj`, `lookup_note`/`fetch_note`, internal `HttpMethod`, `ServerResponse` helpers; **24/24 tests green** |
| 2 | 2026-08-31 | deepen `wj-webhook` (regression) | 0 | 0 | spurious `use BusEventBody;` same-file Serialize (E0255) | split `domain/bus_event.wj`; **15/15 tests green** |
| 2 | 2026-08-30 | deepen `wj-webhook` | 0 | 0 | cross-crate owned forwarder for `wj-validate` literals | Dogfood `wj-cors` + `wj-headers` + `wj-validate`; CORS preflight + field limits; 12 tests green |
| 2 | 2026-08-30 | deepen `wj-migrate` | 0 | 0 | `DirEntry.name()` unwired; cross-module `Connection` emits `.clone()`; `pub mod` at end truncates `lib.rs` | `db_status::status_url` + shared discover; 18 tests green |
| 2 | 2026-08-30 | + `wj-migrate-cli` | 0 | 0 | cross-crate `Vec` borrow blocks `wj-cli-args`; local argv parsing | apply + status CLI; 12 tests green |
| 2 | 2026-08-31 | deepen `wj-migrate-cli` | 0 | 0 | — | `list` subcommand (filesystem-only, dogfoods `discover_migrations`); **17/17 tests green** |
| 2 | 2026-09-01 | deepen `wj-migrate-cli` | 0 | 0 | string var moved to `run_list`/`run_validate` then reused in `teardown_dir` (E0382) — literal path in teardown | `validate` subcommand (filesystem-only, dogfoods `validate_unique_versions`); **21/21 tests green** |
| 2 | 2026-08-31 | deepen `wj-retry` | 0 | 0 | `use std::async_runtime::sleep_ms_blocking` emits `as async` keyword alias (repro filed) | `pause_ms` via `std::time` spin-wait; **5/5 tests green** |

| 2 | 2026-09-03 | + `wj-cron` | 0 | 0 | — | Cron expression parsing + matching (5 fields, `*`, `*/N`, lists, ranges); **14/14 tests green** |
| 2 | 2026-09-03 | deepen `wj-cron` | 0 | 0 | — | `next_run` minute stepper + `CronDateTime` (cross-hour / weekday skip / max look-ahead); **19/19 tests green** |
| 2 | 2026-09-03 | deepen `wj-cron` | 0 | 0 | — | Named aliases (`@hourly`…`@annually`) + range/step (`1-10/2`); **28/28 tests green** |
| 2 | 2026-09-03 | + `wj-scheduler` | 0 | 0 | Vec helper after index → `&Vec` formal + `rest.clone()` E0308 (P3.224 repro; flag scan inlined) | Hexagonal CLI dogfoods `wj-cron`: `check` / `next` / `explain`; **22/22 tests green** |
| 2 | 2026-09-04 | deepen `wj-cron` | 0 | 0 | — | `CronExpr.source` keeps `parse_cron` owned formal; **30/30 tests green** |
| 2 | 2026-09-04 | deepen `wj-scheduler` | 0 | 0 | demoted `&str` ↔ owned `String` call-site (P3.225 / P3.233 repros; `"${expr}"` + direct `matches_cron`) | Crontab `list` / `due`; **37/37 tests green** |
| 2 | 2026-09-04 | deepen `wj-yaml` | 0 | 0 | — | Block scalars `|` (literal) + `>` (folded); **16/16 tests green** |
| 2 | 2026-09-04 | deepen `wj-validate` | 0 | 0 | — | `require_url` + `require_uuid`; **23/23 tests green** |
| 2 | 2026-09-11 | graduate `wj-mime` | 0 | 1 | `std::mime.from_*` charset ≠ APPLICATION_* (P3.243 repro) | Constants via `std::mime`; lookup stays WJ; **12/12 tests green** |
| 2 | 2026-09-11 | graduate `wj-path` | 0 | 0 | — | `join_path`/`basename` → `std::path`; normalize/dirname/extname sugar; **10/10 tests green** |
| 2 | 2026-09-11 | graduate `wj-yaml` | 0 | 1 | `std::yaml.to_json` accepts empty input (P3.244 repro); package pre-checks | `to_json` → `std::yaml`; get_* path sugar; **16/16 tests green** |
| 2 | 2026-09-11 | full graduate `wj-mime` | 0 | 0 | — | Full thin-wrap `std::mime` (lookup + predicates); P3.243 ✅; **12/12 tests green** |
| 2 | 2026-09-11 | full graduate `wj-yaml` | 0 | 0 | — | Drop empty pre-check; `to_json` → `std::yaml` only; P3.244 ✅; **16/16 tests green** |
| 2 | 2026-09-12 | deepen `wj-uuid` | 0 | 0 | tip `i64 & 0xff` → `255_u8` (P3.250 repro) | RFC 9562 **v7** + **NIL**/**MAX**; **20/20** on wj 0.50.0 |
| 2 | 2026-09-12 | deepen `wj-toml` | 0 | 0 | tip `get(&text)` owned formal (P3.254); `"${raw}"` interim (P3.251) | Dotted keys + inline tables; **17/17** on wj 0.50.0 |
| 2 | 2026-09-12 | deepen `wj-validate` | 0 | 0 | — | `require_uuid` accepts v1/v4/v5/v7 + RFC variant; **27/27** |
| 2 | 2026-09-12 | deepen `wj-auth-api` | 0 | 0 | multipass lit→demoted `&str` + `.to_string()` (P3.259); dual-runtime `HttpMethod` in tests | Register assigns **UUID v7** id; JWT `sub`=id; `handle_http` adapter port; **12/12** |
| 2 | 2026-09-12 | deepen `wj-config` | 0 | 0 | — | `from_toml` / `resolve_toml` via `wj-toml`; **7/7** |
| 2 | 2026-09-13 | deepen `wj-auth-api` config | 0 | 0 | HashMap get → demoted `&str` + `.clone()` (P3.261); interim `"${v}"` | `config_from_toml` dogfoods wj-config/wj-toml; **15/15** |
| 2 | 2026-09-13 | deepen `wj-auth-api` cookies + rate-limit | 0 | 0 | tip `wj` (2026-09-13) RED on owned cross-crate formals / notes-api; verified on cargo-bin `wj` 0.50.0 (2026-08-27); P3.262 metadata gate | Cookie session (`wj-cookie`) + `/logout` + fixed-window rate limit (`wj-rate-limit`); **20/20** |
| 2 | 2026-09-13 | deepen `wj-auth-api` security headers | 0 | 0 | — | Dogfood `wj-headers` on JSON / OPTIONS / 429; **23/23** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` validate | 0 | 0 | — | Dogfood `wj-validate` (nonempty/min/max) + config limits; **28/28** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` hash+jwt | 0 | 0 | — | Dogfood `wj-hash`/`wj-jwt`; configurable `tenant_slug` on `/me`; **30/30** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` router | 0 | 0 | — | Dogfood `wj-router` for dispatch + path normalize; **32/32** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` mime | 0 | 0 | — | Dogfood `wj-mime` `Content-Type` on `HttpReply` + adapter; **33/33** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` duration | 0 | 0 | — | Dogfood `wj-duration` for JWT TTL / window (`2h`, `30m`, `2m`); **36/36** on cargo-bin `wj` |
| 2 | 2026-09-13 | deepen `wj-auth-api` dotenv | 0 | 0 | — | Dogfood `wj-dotenv` `config_from_dotenv` + key map; **40/40** on cargo-bin `wj` |

## Weekly checklist

- [ ] No project-local `ffi/` / hand-written `.rs` / `extern fn` in apps
- [ ] CI green on pinned `wj`
- [ ] Changed seeds pass `wj test`
- [ ] [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) updated when production code gains std usage
- [ ] Compiler bugs get failing tests in `windjammer/tests/`
