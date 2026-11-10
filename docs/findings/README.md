# Compiler findings from the tip-health sweep

Found by running every package and app test suite interpreted (`wj test --interpret`) and compiled
(`wj test`) with `scripts/health_sweep.wj`. Each open finding has a generic repro under `repros/`; the
packages keep their idiomatic shape and carry no shim for them.

Current sweep: `wj` 0.50.0, commit `88d4cea796fd` (the pinned integration head; overflow checks on, raw TCP/UDP in `std::net`).

Fixed since the previous sweep (repros retired): `strings.substring` with a computed `int` local, the counter inferred
`usize` in a tuple return, and `strings.split_lines` with a trailing CRLF. `wj check` now reports compiled-width problems
(WJ0003) before a build, which caught three of this round's fixes in `wj-sqlite`.

## Open

Each repro is a package directory (`wj test`) or a note. "Interpreted vs compiled" means the interpreter accepts the code and
the Rust backend does not.

| # | Finding | Repro | Effect |
|---|---|---|---|
| 1 | `text.parse::<int>() ?? -1` on a `Result` compiles to `unwrap_or_else(\|\| ..)`, which a `Result` expects with one argument (E0593). Interpreted it works. | [`coalesce_result_parse`](repros/coalesce_result_parse) | code uses `.unwrap_or(..)`. |
| 2 | A float expression passed to a `float` parameter is cast `as f32` (E0308): `is_huge(1.0e300 * 10.0)`, `format_value(0.0 - inf)`. | [`float_exponent_literal_f32`](repros/float_exponent_literal_f32) | the expression is bound to a `let` first. |
| 3 | A tuple literal in a `match` arm, `Err(_) => (0, "")`, is typed `(i32, &str)` and clashes with the other arm's `(i64, String)` (E0308). | [`tuple_literal_match_arm`](repros/tuple_literal_match_arm) | tests read one field through a helper. |
| 4 | A list of tuple literals or of string literals in a struct field, a call argument or a typed `let` (`Vec<(string, string)>`) keeps `&str` elements (E0308). Building the vector with `push` works. | [`literal_list_to_vec_param`](repros/literal_list_to_vec_param) | tests build such vectors with `push`. |
| 5 | `for (k, e) in self.map { best = k }`: the key is borrowed and assigning it to a `string` local needs a copy the compiler does not insert (E0308). | [`map_loop_key_assigned`](repros/map_loop_key_assigned) | `wj-cache` copies with `"${k}"`. |
| 6 | `buf[(after + len + 1) as usize]` loses its parentheses: the cast lands on `len` alone (E0308 / E0277). | [`parenthesised_sum_cast`](repros/parenthesised_sum_cast) | `wj-cache` computes the index into a `let` first. |
| 7 | A field of a struct parameter is consumed (`t.refresh.unwrap_or("none")`) and the whole struct is used afterwards: the compiled code mixes `as_ref()` with an owned default (E0308), or reports E0382. | [`partial_move_then_whole_use`](repros/partial_move_then_whole_use) | tests call the whole-struct function first. |
| 8 | A clamp written as chained `if` expressions mixes `i64` and `usize` branches (E0308). Not reduced: it needs the surrounding function. | [`chained_if_clamp_not_reduced.md`](repros/chained_if_clamp_not_reduced.md) | `wj-cli-ui` uses a `clamp` helper. |
| 9 | `picked.contains(&i)` with a local `i` loses the `&` in the generated call (E0308 "expected `&i64`, found `i64`"). | [`vec_contains_ref_dropped`](repros/vec_contains_ref_dropped) | loops compare by hand. |
| 10 | `env.temp_dir()` is declared in `std/env` and works interpreted; the compiled runtime's `env` module has no `temp_dir` (E0425). | [`env_temp_dir_missing_in_runtime`](repros/env_temp_dir_missing_in_runtime) | tests use a fixed `/tmp/...` directory. |
| 11 | A call to a function or method that has a function-typed parameter is not adapted like other calls: integer literal arguments are typed `i32` (E0308 expected `i64`), string literals are passed as `String` where `&str` is expected, and a local passed where `&mut` is needed is passed by value. | [`fn_param_call_adaptation`](repros/fn_param_call_adaptation) | `wj-prop` tests use typed constants; `wj-cli-ui` validates answers with a `Rule` enum, not a closure. |
| 12 | `db.connect("sqlite:<path>?mode=rwc")` under an explicit `[capabilities]` grant: the check reads the URL as the relative path `./sqlite:/private/var/...` and refuses a database in the test scratch directory that the plain path opens (exit 126). | [`db_sqlite_url_capability`](repros/db_sqlite_url_capability) | tests connect with a plain path. |
| 13 | The runtime feature groups are decided from the library's generated Rust only: a test file that is the sole user of `std::db` gets `use windjammer_runtime::db;` with the `db` feature left out (E0432 "configured out"). | [`runtime_feature_from_tests_only`](repros/runtime_feature_from_tests_only) | `wj-sqlite` uses `std::db` in its library (`verify`). |
| 14 | `items(n)?.len() as int` as a struct field: the interpreter reports `a Result has no method __opt_chain`. | [`try_then_method_in_struct_literal`](repros/try_then_method_in_struct_literal) | bind the `?` result first. |

Sweep method: `~/src/wj-wt/bin/slot wj run --allow process=sh scripts/health_sweep.wj <wj> <logdir> both packages/* apps/*`,
then `scripts/health_table.wj` renders the table. The compiler's own `scripts/ecosystem_width_gate.wj` runs the compiled
sweep over the targets the usize-class merge once broke. Running two sweeps (or a sweep and a reduction) at once against one
cargo target directory can fail a build with `package collision in the lockfile`; rerun that target alone.

## Derived output is not tracked

`gen/` (generated Rust) is derived, so it is untracked and ignored (`AGENTS.md`: generated output is never source); the build
regenerates it.
