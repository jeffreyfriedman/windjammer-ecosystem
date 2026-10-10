# Compiler workarounds (input for the next cleanup phase)

Each entry is code shaped around a compiler limitation. None were rewritten in this pass
(no builds were run, so none can be verified). When the named compiler fix lands, replace
with the idiomatic form and delete the comment. AGENTS.md rule: fix the compiler, not the app.

| Location | Current shape | Idiomatic target |
|---|---|---|
| apps/wj-fetch/src/adapters/http_get.wj:7 | Timeout accepted at CLI but not applied (`std::net::Request` import codegen, P3.219) | Build a request with `timeout_secs` and call it |
| apps/wj-notes-api/src/domain/api.wj:19 | Import alias `qs_get` to avoid ownership metadata clash with `wj_url` (P3.283) | `use wj_querystring::get` under its natural name |
| apps/wj-scheduler/src/domain/commands.wj:43 | Inline `--at` flag scan loop (P3.224) | Call the shared flag helper from `wj-cli-args` |
| apps/wj-todo-cli/src/domain/config.wj:36 | Local `resolve_file_path` (cross-crate owned-formal move) | Use `wj-path` join |
| apps/wj-webhook/src/domain/webhook.wj:54 | SHA-256 of `secret.body` as demo signing | Real HMAC once `std` has it |
| packages/wj-base64/tests/base64_test.wj:61 | Typed `0u8` literals | Plain int literals into `Vec<u8>` |
| packages/wj-cookie/src/lib.wj:49 | Bound `Some(v) => { let _keep = v; true }` arm (P3.676) | `map.get(name).is_some()` or `Some(_) => true` |
| packages/wj-cors/src/lib.wj:36 | Inline loop instead of helper taking `&Vec` (emits spurious clone) | Call the existing helper |
| packages/wj-dotenv/src/lib.wj:55 | Full `match` with bound arm (P3.676) | `Some(_)` wildcard / `matches!` |
| packages/wj-querystring/src/lib.wj:4 | Notes tied to P3.463/526/535 wiring | Reword doc once stable |
| packages/wj-regex/src/lib.wj:41 | `.len()` return shape (P3.681) | `Ok(x.len())` directly |
| packages/wj-retry/src/lib.wj:263 | Spin-wait `pause_ms` on `time.utc_now()` | `std::async_runtime::sleep_ms_blocking` |
| packages/wj-sha/src/lib.wj:21 | `chars` loop (P3.679) | Indexed `while i < 64` with substring |
| packages/wj-sync/src/lib.wj:3, pending.wj:2, shared.wj:2 | No type aliases (P3.333/333b) | Type aliases for common instantiations |
| packages/wj-toml/src/lib.wj:285 | `"${raw}"` to force an owned copy (P3.251) | Pass `raw` directly |
| packages/wj-uuid/src/lib.wj:59 | Namespace literal instead of module `const` | `const NS_DNS` passed to `v5` |

That is 16 locations (18 sites counting wj-sync's three).

## Dependencies on transpiled output

Every cross-package dependency in the following manifests points at `.../build` (generated
output, not tracked in git), for example `wj_cors = { path = "../../packages/wj-cors/build" }`:

apps/wj-auth-api, wj-fetch, wj-find, wj-form-parse, wj-migrate-cli, wj-notes-api,
wj-pipeline, wj-proxy, wj-scheduler, wj-sitegen, wj-todo-cli, wj-webhook; and
packages/wj-sync/benches/wj_baseline (`path = "../../build"`).

Left unchanged: `resolver.rs` in the compiler is a stub and does not show whether `wj`
accepts a source-dir path dependency. Target: `path = "../../packages/wj-cors"`, so a
clean checkout builds without first building every package. Needs a compiler check.
