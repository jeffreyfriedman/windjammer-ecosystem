# Compiler workarounds

Burn-down of code that was shaped around a compiler limitation. Each entry was retried in its
idiomatic form against the current tip `wj` (see [findings](findings/README.md) for the compiler version
and the sweep method) and kept only when the idiomatic form still fails.

## Burned down

All 16 of the original entries now use the idiomatic form, with interpreted and compiled tests green for the
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
| `wj-fetch` `--timeout` | `http.request("GET", url).timeout_secs(n).send()` |
| `wj-uuid` namespace literal | `v5(NAMESPACE_DNS, name)` using the existing `pub const` |
| `wj-notes-api` `qs_get` alias | `use wj_querystring::get` under its natural name |
| `wj-retry` spin-wait `pause_ms` | `std::async::sleep_ms_blocking` (the old entry named `std::async_runtime`); a test asserts the wait |

## Kept

None. The last entry, `wj-fetch --timeout`, now builds its request with `http.request("GET", url).timeout_secs(n).send()`.

Newer packages carry small forms chosen to avoid compiler gaps that are recorded as findings
([findings](findings/README.md)); those are findings, not workaround entries.

## Dependencies on transpiled output

Every cross-package dependency still points at `.../build`. The source-directory form
(`wj_glob = { path = "../wj-glob" }`) builds the dependency but cargo then reports
`no matching package named wj_glob`, so the `/build` form stays until the compiler resolves
source-directory path dependencies ([findings](findings/README.md), item 5).
