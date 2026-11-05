# Compiler findings from the tip-health sweep

Found by running every package and app test suite interpreted (`wj test --interpret`) and compiled
(`wj test`) with `scripts/health_sweep.wj`. Each open finding has a generic repro under `repros/`; the
packages keep their idiomatic shape and carry no shim for them.

Current sweep: `wj` 0.50.0, commit `6ed79d644d3f` (the pinned integration head).

Fixed since the first sweeps (their repros are retired): a relative `wj build src` next to a `wj.toml`, the resolved-engine
`Option` match, the cross-crate `&Vec` argument, `sleep_ms_blocking(int)`, the nested-loop move, a Windjammer dependency treated
as a Rust crate (WJ0050), declared-but-missing std functions in the compiled runtime (`hex_encode_string` and friends), list
literals returned as `Vec<T>`, `strings.chars(x).collect()`, iterating a list of string literals, unknown std functions
accepted by `wj check`, `wj check` at a package root with `pub mod`, the `std::http` request builder, and the
use-after-move that needed `.clone()` calls in `wj-markdown`.

## Open

Each repro is a package directory (`wj test`) or a note. "Interpreted vs compiled" means the interpreter accepts the code and
the Rust backend does not.

| # | Finding | Repro | Effect |
|---|---|---|---|
| 1 | `text.parse::<int>() ?? -1` on a `Result` compiles to `unwrap_or_else(\|\| ..)`, which a `Result` expects with one argument (E0593). Interpreted it works. | [`coalesce_result_parse`](repros/coalesce_result_parse) | code uses `.unwrap_or(..)`. |
| 2 | `strings.substring(s, cut, n)` with an `int` local computed from other ints: `std/strings.wj` declares `usize` bounds and the compiled call is rejected (E0308) although literals and `int` parameters work. | [`substring_int_variable_bound`](repros/substring_int_variable_bound) | helpers take the bounds as parameters. |
| 3 | A counter that is only incremented and later passed to `substring` is inferred `usize`, and a tuple return declared `(int, string)` then fails (E0308). | [`counter_in_tuple_return`](repros/counter_in_tuple_return) | the counter is annotated `: int`. |
| 4 | A float expression passed to a `float` parameter is cast `as f32` (E0308): `is_huge(1.0e300 * 10.0)`, `format_value(0.0 - inf)`. | [`float_exponent_literal_f32`](repros/float_exponent_literal_f32) | the expression is bound to a `let` first. |
| 5 | A tuple literal in a `match` arm, `Err(_) => (0, "")`, is typed `(i32, &str)` and clashes with the other arm's `(i64, String)` (E0308). | [`tuple_literal_match_arm`](repros/tuple_literal_match_arm) | tests read one field through a helper. |
| 6 | A list of tuple literals or a list of string literals in a struct field, a call argument or a typed `let` (`Vec<(string, string)>`, `Vec<string>`) keeps `&str` elements (E0308). Building the vector with `push` works. | [`literal_list_to_vec_param`](repros/literal_list_to_vec_param) | tests build such vectors with `push`. |
| 7 | `for (k, e) in self.map { best = k }`: the key is borrowed and assigning it to a `string` local needs a copy the compiler does not insert (E0308). | [`map_loop_key_assigned`](repros/map_loop_key_assigned) | `wj-cache` copies with `"${k}"`. |
| 8 | `buf[(after + len + 1) as usize]` loses its parentheses: the cast lands on `len` alone (E0308 / E0277). | [`parenthesised_sum_cast`](repros/parenthesised_sum_cast) | `wj-cache` computes the index into a `let` first. |
| 9 | A field of a struct parameter is consumed (`t.refresh.unwrap_or("none")`) and the whole struct is used afterwards: the compiled code mixes `as_ref()` with an owned default (E0308), or reports E0382 "partially moved". | [`partial_move_then_whole_use`](repros/partial_move_then_whole_use) | tests call the whole-struct function first. |
| 10 | `strings.split_lines("250 OK\r\n")` has one element interpreted and a trailing empty element compiled. | [`split_lines_trailing_empty`](repros/split_lines_trailing_empty) | `wj-smtp` skips empty lines. |
| 11 | A clamp written as chained `if` expressions mixes `i64` and `usize` branches (E0308). Not reduced: it needs the surrounding function. | [`chained_if_clamp_not_reduced.md`](repros/chained_if_clamp_not_reduced.md) | `wj-cli-ui` uses a `clamp` helper. |

Sweep method: `~/src/wj-wt/bin/slot wj run --allow process=sh scripts/health_sweep.wj <wj> <logdir> both packages/* apps/*`,
then `scripts/health_table.wj` renders the table. Running two sweeps (or a sweep and a reduction) at once against one cargo
target directory can fail a build with `package collision in the lockfile`; rerun that target alone.

## Derived output is not tracked

`gen/` (generated Rust) is derived, so it is untracked and ignored (`AGENTS.md`: generated output is never source); the build
regenerates it.
