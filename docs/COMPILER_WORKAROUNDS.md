# Compiler workarounds

Burn-down of code that was shaped around a compiler limitation. Each entry was retried in its
idiomatic form against the current tip `wj` (see [findings](findings/README.md) for the compiler version
and the sweep method) and kept only when the idiomatic form still fails.

## Burned down

13 of the original 16 entries now use the idiomatic form, with interpreted and compiled tests green for the
package or app that owns the code.

| Entry | Idiomatic form now in place |
|---|---|
| `wj-scheduler` `--at` flag scan | `wj_cli_args::long_flag_value(rest, "at")` |
| `wj-todo-cli` `resolve_file_path` | delegates to `wj_path::join_path` |
| `wj-webhook` demo SHA-256 signing | HMAC-SHA256 via `std::crypto.hmac_sha256`, `verify_hmac_sha256` and `constant_time_eq` (needs the compiler change on `wt/std-hmac`; see below) |
| `wj-base64` typed `0u8` literals in tests | plain integer literals into `Vec<u8>` |
| `wj-cookie` bound `Some(v)` arm | `Some(_) => true` |
| `wj-cors` inline origin loop | `allow_origin` calls `is_origin_allowed` |
| `wj-dotenv` bound `Some(v)` arm | `map.get(key).is_some()` |
| `wj-querystring` P3.x wiring notes | doc reworded: `parse` thin-wraps `std::encoding.form_parse`, which strips the `?` |
| `wj-regex` loop counting matches | `Ok(hits.len())` |
| `wj-sha` `chars` loop | the `for ch in strings.chars(text)` loop is the idiomatic form and passes; note removed |
| `wj-sync` "no type aliases" | `SharedInt`, `IntReceiver`, ... are `pub type` aliases already; stale comments removed |
| `wj-toml` `"${raw}"` owned copy | `normalize_value(raw)` |
| `wj-uuid` namespace literal | `v5(NAMESPACE_DNS, name)` using the existing `pub const` |

`wj-cookie`, `wj-dotenv` and `wj-toml` still have interpreted failures in their `get`-style tests; those are
the engine finding below, present before and after these edits (same failing tests).

## Kept, with a minimal repro

| Location | Current shape | Idiomatic target | Why it stays | Repro |
|---|---|---|---|---|
| `apps/wj-fetch/src/adapters/http_get.wj` | `--timeout` accepted but not applied | build a request with a timeout | `std::http.get` returns a `Result`; there is no request builder, so `.timeout_secs(n)` has nothing to attach to (interpreter: `a Result has no method timeout_secs`; compiled: build fails) | [`http_request_timeout.wj`](findings/repros/http_request_timeout.wj) |
| `apps/wj-notes-api/src/domain/api.wj` | `use wj_querystring::get as qs_get` | `use wj_querystring::get` | interpreted tests pass with the natural name; the compiled build of this app is red for an unrelated reason (the `wj_cors` finding), so the change cannot be verified compiled | [`wj_cors_allow_origin_borrowed_vec`](findings/repros/wj_cors_allow_origin_borrowed_vec) |
| `packages/wj-retry/src/lib.wj` | spin-wait `pause_ms` on `time.utc_now()` | `std::async::sleep_ms_blocking` | interpreter accepts it; compiled build fails: runtime takes `u64`, the wrapper passes Windjammer `int` (`i64`) | [`sleep_ms_blocking_int_arg`](findings/repros/sleep_ms_blocking_int_arg) |

(The old entry text named `std::async_runtime`; the module is `std::async`.)

## Dependencies on transpiled output

Every cross-package dependency still points at `.../build`. Trying the source-directory form
(`wj_glob = { path = "../wj-glob" }`) with the current compiler builds the dependency but then cargo
reports `no matching package named wj_glob`, so the `/build` form stays until the compiler resolves
source-directory path dependencies. This is recorded in [findings](findings/README.md).
