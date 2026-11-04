# Compiler findings from the tip-health sweep

Found by running every package and app test suite interpreted (`wj test --interpret`) and compiled
(`wj test`) with `scripts/health_sweep.wj`. Each open finding has a generic repro under `repros/`; the
packages keep their idiomatic shape and carry no shim for them.

Current sweep: `wj` 0.50.0, commit `22d3fe8b2a00` (the pinned integration head). An earlier sweep at
`4b18e8c2e` found eight issues; five are fixed at the pinned head (a relative `wj build src` next to a
`wj.toml`, the resolved-engine `Option` match returning `()`, the cross-crate `&Vec` argument, the
`sleep_ms_blocking(int)` width, and the nested-loop move of an `Option` struct field), so their repros were retired.

## Open

Items 1 to 4 were found by the sweep on the existing packages; 5 to 15 by writing the five new packages (`wj-decimal`,
`wj-ws`, `wj-metrics`, `wj-markdown`, `wj-archive`) and running them interpreted and compiled. Each repro is a package
directory (`wj test`) or a single file. "Interpreted vs compiled" means the interpreter accepts the code and the Rust
backend does not. Fixed at `b77ea8a63` since the previous sweep: the `usize` index pushed into a `Vec<int>`, and
`(result ?? Vec::new())` (the `parse` form below still fails).

| # | Finding | Repro | Effect |
|---|---|---|---|
| 1 | `std::http` has no request builder: `http.get(url)` returns a `Result`, so a timeout cannot be set. | [`http_request_timeout.wj`](repros/http_request_timeout.wj) | `wj-fetch --timeout` is accepted but not applied. |
| 2 | `wj check` run at a package root reports WJ0006 "unresolved import" for `pub use shapes::area` when `pub mod shapes` lives in `src/shapes.wj`; `wj check src`, `wj test` and `wj build` accept the same package. | [`check_pub_mod_unresolved_import`](repros/check_pub_mod_unresolved_import) | `wj check` at the root of `wj-sync` reports 74 errors. |
| 3 | `text.parse::<int>() ?? -1` on a `Result` compiles to `unwrap_or_else(\|\| ..)`, which a `Result` expects with one argument (E0593). Interpreted it works. | [`coalesce_result_parse`](repros/coalesce_result_parse) | code uses `.unwrap_or(..)`. |
| 4 | A path dependency on a package source directory (`path = "../wj-glob"`) builds the dependency but cargo then reports `no matching package named wj_glob`; only the `.../build` form works (checked at `4b18e8c2e`). | `packages/wj-gitignore/wj.toml` with `wj-glob/build` replaced by `wj-glob` | dependencies stay on `/build` paths. |
| 5 | A Windjammer package used as a path dependency is checked like a Rust crate (ADR-0026) and `wj build src` stops with WJ0050 "function `is_match` is not declared in the signature surface" for a plain `pub fn is_match(pattern: string, ..)` in a package that also calls it with an interpolated argument. The same code built at `22d3fe8b2a00`. | [`wj_dependency_treated_as_rust_crate`](repros/wj_dependency_treated_as_rust_crate) | `wj-find` and `wj-grep` (through `wj-gitignore`) red compiled. |
| 6 | `std::encoding.hex_encode_string` / `hex_decode_string` and `std::crypto.base64_encode_bytes` / `base64_decode_bytes` are declared in `std/*.wj` and implemented by the interpreter, but the compiled runtime has no such functions (E0425). | [`std_declared_missing_in_runtime`](repros/std_declared_missing_in_runtime) | the new packages use `base64_encode_string` / `base64_decode` instead. |
| 7 | A list literal as the tail expression of a function returning `Vec<float>` or `Vec<int>` stays an array (E0308 "expected `Vec<f64>`, found `[f64; 11]`"). | [`list_literal_tail_vec`](repros/list_literal_tail_vec) | tables are bound first (`let t: Vec<int> = [..]` then `t`). |
| 8 | `strings.substring(s, cut, n)` with `int` variables: `std/strings.wj` declares `usize` bounds and the compiled call is rejected (E0308) although literals work. | [`substring_int_variable_bound`](repros/substring_int_variable_bound) | `wj-markdown` wraps it once in an `int`-bounds helper using `as usize`. |
| 9 | A counter that is only incremented and later passed to `substring` is inferred `usize`, and a tuple return declared `(int, string)` then fails (E0308). | [`counter_in_tuple_return`](repros/counter_in_tuple_return) | the counter is annotated `: int`. |
| 10 | `strings.chars(x).collect()` works interpreted (an iterator) but `strings.chars` returns a `Vec<char>` compiled (E0599). | [`chars_collect`](repros/chars_collect) | a helper pushes the characters into a `Vec<char>`. |
| 11 | `for t in ["a", "b"] { f(t.clone()) }` iterates `&str` compiled (E0308) and `match s.clone().as_str()` needs an unstable feature (E0658). | [`for_over_str_literals`](repros/for_over_str_literals) | tests bind a typed `Vec<string>` first. |
| 12 | A float expression passed to a `float` parameter is cast `as f32` (E0308): `is_huge(1.0e300 * 10.0)`, `format_value(0.0 - inf)`. | [`float_exponent_literal_f32`](repros/float_exponent_literal_f32) | the expression is bound to a `let` first. |
| 13 | A tuple literal in a `match` arm, `Err(_) => (0, "")`, is typed `(i32, &str)` and clashes with the other arm's `(i64, String)` (E0308). | [`tuple_literal_match_arm`](repros/tuple_literal_match_arm) | tests read one field through a helper. |
| 14 | A `string` passed to a by-value helper that only reads it, in a condition, and used again later: the automatic `.clone()` lands on the later use, so the earlier call moves the value (E0382). Seen in `&&` chains and across an `if` / `else`. Standalone attempts compile; the steps that reproduce it in `wj-markdown` are written down. | [`markdown_use_after_move.md`](repros/markdown_use_after_move.md) | `wj-markdown` clones explicitly at the reused sites (W0005 warnings). |
| 15 | `wj check` accepts a call to a function that does not exist in `std::strings` (`strings.index_of`); the interpreter then fails with "not implemented" and the message repeats `std::std::strings`. | [`check_accepts_unknown_std_function.wj`](repros/check_accepts_unknown_std_function.wj) | none; caught only by running. |

Also seen once at `4b18e8c2e`: `mine[0].find("two") ?? 0` on an indexed `Vec<string>` element compiled to
`unwrap_or(0_i32)` against a `usize` (E0308); a standalone repro did not reproduce it, so it is not listed.

Sweep method: `~/src/wj-wt/bin/slot wj run --allow process=sh scripts/health_sweep.wj <wj> <logdir> both packages/* apps/*`,
then `scripts/health_table.wj` renders the table. Running two sweeps at once against one cargo target directory
can fail a build with `package collision in the lockfile`; rerun that target alone.

## Derived output is not tracked

`gen/` (generated Rust) was checked in for five packages and had gone stale: it held `len()` against `int`
(WJ0003) and pointed at an older embedded runtime. It is derived, so it is untracked and ignored
(`AGENTS.md`: generated output is never source); the build regenerates it.
