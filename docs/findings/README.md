# Compiler findings from the tip-health sweep

Found by running every package and app test suite interpreted (`wj test --interpret`) and compiled
(`wj test`) with `scripts/health_sweep.wj`. Each finding has a generic repro under `repros/` and is
a compiler issue: the packages keep their idiomatic shape and no in-package shim is added.

Compiler: `next/v0.50.0` at `4b18e8c2e` (0.50.0).

| # | Finding | Repro | Effect in the ecosystem |
|---|---|---|---|
| 1 | `wj build src` (relative `src` next to a `wj.toml`) stops with `Error: Path does not exist: `. `check::effects::manifest_path("src")` returns the relative `wj.toml`, whose parent is the empty path. Treating an empty parent as `.` fixes it (one line); `wj build ./src` and `wj build .` work. | [`build_src_relative_path`](repros/build_src_relative_path) | Every compiled dependency build (`wj test` builds path dependencies with `wj build src`) fails, so all 17 apps and `wj-gitignore` are red compiled on the unpatched binary. The sweep below uses the same binary with that one-line fix applied. |
| 2 | Resolved engine: `match map.get(k) { Some(v) => Some(v), None => None }` evaluates to `()` on a miss (the tree-walker engine is right). | [`resolved_engine_option_match_none.wj`](repros/resolved_engine_option_match_none.wj) | `wj-config`, `wj-cookie`, `wj-dotenv`, `wj-toml` fail their `get`-style tests interpreted (2, 3, 2, 4 tests). |
| 3 | Cross-crate call to `wj_cors::allow_origin` from a `Vec<string>` parameter passes the Vec by value although the callee takes `&Vec<String>` (E0308). Shrunk from an app with `wj reduce` to 4 lines; not yet narrowed to a standalone library, which does not reproduce. | [`wj_cors_allow_origin_borrowed_vec`](repros/wj_cors_allow_origin_borrowed_vec) | `wj-auth-api`, `wj-notes-api`, `wj-webhook` red compiled. |
| 4 | `std::async::sleep_ms_blocking(ms)` with a Windjammer `int` fails to compile: runtime takes `u64`, generated code passes `i64` (E0308). | [`sleep_ms_blocking_int_arg`](repros/sleep_ms_blocking_int_arg) | blocks `wj-retry` `pause_ms` target form. |
| 5 | `std::http` has no request builder: `http.get(url)` returns a `Result`, so a timeout cannot be set. | [`http_request_timeout.wj`](repros/http_request_timeout.wj) | `wj-fetch --timeout` is accepted but not applied. |
| 6 | `out.push(i + 1)` where `i` is bounded by `lines.len()` and `out` is returned as `Vec<int>`: the index type (`usize`) leaks into the `i64` vector (E0308). | [`usize_index_pushed_into_int_vec`](repros/usize_index_pushed_into_int_vec) | `wj-grep` red compiled. |
| 7 | A path dependency on a package source directory (`path = "../wj-glob"`) builds the dependency but cargo then reports `no matching package named wj_glob`; only the `.../build` form works. | `packages/wj-gitignore/wj.toml` with `wj-glob/build` replaced by `wj-glob` | dependencies stay on `/build` paths. |
| 8 | A struct parameter used by value in a `for` loop (`lines_of(root, cmd)`), then an `Option<string>` field matched inside a nested loop (`match cmd.replace { Some(replacement) => ... }`), reports E0382 "value moved here, in previous iteration of loop". | [`option_field_match_moved_in_loop`](repros/option_field_match_moved_in_loop) | `wj-grep` red compiled. |

Sweep method: `wj run --allow process=sh scripts/health_sweep.wj <wj> <logdir> both packages/* apps/*`, then
`scripts/health_table.wj` renders the table. Findings 3 and 8 were shrunk from the failing apps with `wj reduce`.
Running two sweeps (or a sweep and a reduction) at once against one cargo target directory can fail a build with
`package collision in the lockfile`; rerun that target alone.
