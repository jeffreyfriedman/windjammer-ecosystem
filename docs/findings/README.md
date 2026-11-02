# Compiler findings from the tip-health sweep

Found by running every package and app test suite interpreted (`wj test --interpret`) and compiled
(`wj test`) with `scripts/health_sweep.wj`. Each open finding has a generic repro under `repros/`; the
packages keep their idiomatic shape and carry no shim for them.

Current sweep: `wj` 0.50.0, commit `22d3fe8b2a00` (the pinned integration head). An earlier sweep at
`4b18e8c2e` found eight issues; five are fixed at the pinned head (a relative `wj build src` next to a
`wj.toml`, the resolved-engine `Option` match returning `()`, the cross-crate `&Vec` argument, the
`sleep_ms_blocking(int)` width, and the nested-loop move of an `Option` struct field), so their repros were retired.

## Open

| # | Finding | Repro | Effect in the ecosystem |
|---|---|---|---|
| 1 | `std::http` has no request builder: `http.get(url)` returns a `Result`, so a timeout cannot be set. | [`http_request_timeout.wj`](repros/http_request_timeout.wj) | `wj-fetch --timeout` is accepted but not applied. |
| 2 | `out.push(i + 1)` where `i` is bounded by `lines.len()` and `out` is returned as `Vec<int>`: the index type (`usize`) leaks into the `i64` vector (E0308). | [`usize_index_pushed_into_int_vec`](repros/usize_index_pushed_into_int_vec) | `wj-grep` red compiled. |
| 3 | `wj check` run at a package root reports WJ0006 "unresolved import" for `pub use shapes::area` when `pub mod shapes` lives in `src/shapes.wj`; `wj check src`, `wj test` and `wj build` accept the same package. | [`check_pub_mod_unresolved_import`](repros/check_pub_mod_unresolved_import) | `wj check` at the root of `wj-sync` reports 74 errors; the package builds and passes its tests. |
| 4 | `(result ?? Vec::new())` on a `Result<Vec<int>, string>` compiles to `unwrap_or_else(\|\| ..)`, which a `Result` expects with one argument (E0593). Interpreted it works. | [`coalesce_result_vec_closure`](repros/coalesce_result_vec_closure) | Tests use a `match` instead. |
| 5 | A path dependency on a package source directory (`path = "../wj-glob"`) builds the dependency but cargo then reports `no matching package named wj_glob`; only the `.../build` form works (checked at `4b18e8c2e`). | `packages/wj-gitignore/wj.toml` with `wj-glob/build` replaced by `wj-glob` | dependencies stay on `/build` paths. |

Also seen once at `4b18e8c2e`: `mine[0].find("two") ?? 0` on an indexed `Vec<string>` element compiled to
`unwrap_or(0_i32)` against a `usize` (E0308); a standalone repro did not reproduce it, so it is not listed.

Sweep method: `~/src/wj-wt/bin/slot wj run --allow process=sh scripts/health_sweep.wj <wj> <logdir> both packages/* apps/*`,
then `scripts/health_table.wj` renders the table. Running two sweeps at once against one cargo target directory
can fail a build with `package collision in the lockfile`; rerun that target alone.

## Derived output is not tracked

`gen/` (generated Rust) was checked in for five packages and had gone stale: it held `len()` against `int`
(WJ0003) and pointed at an older embedded runtime. It is derived, so it is untracked and ignored
(`AGENTS.md`: generated output is never source); the build regenerates it.
