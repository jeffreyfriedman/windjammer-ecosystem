# wj-fuzzy

Fuzzy text matching in pure Windjammer: edit distance, "did you mean" suggestions for CLIs and config keys, and a
subsequence scorer for command palettes and file pickers. Comparison is by character, so accented text counts
correctly, and everything is integer arithmetic.

```windjammer
use wj_fuzzy::fuzzy_score
use wj_fuzzy::hint
use wj_fuzzy::rank
use wj_fuzzy::suggest

fn main() {
    let commands = ["build", "test", "check", "clean", "publish"]

    // a mistyped CLI command
    println("unknown command 'biuld'; ${hint("biuld", commands)}")
    println("nothing close: '${hint("deploy-everything", commands)}'")
    println("candidates: ${suggest("cleen", commands, 2).join(", ")}")

    // a file picker
    let files = ["README.md", "tests/main_test.wj", "src/main.wj", "Cargo.toml"]
    println("rank 'main': ${rank("main", files).join(", ")}")
    println("no match score: ${fuzzy_score("xyz", "src/main.wj")}")
}
```

Output:

```text
unknown command 'biuld'; did you mean 'build'?
nothing close: ''
candidates: clean
rank 'main': src/main.wj, tests/main_test.wj
no match score: -1
```

## How it behaves

| Call | Effect |
|---|---|
| `levenshtein(a, b)` | insertions, deletions and substitutions |
| `damerau(a, b)` | the same, with a neighbouring swap costing 1 (`teh` and `the`) |
| `similarity_percent(a, b)` | 0 to 100 relative to the longer text |
| `suggest(input, candidates, max_distance)` | near matches, nearest first, case-insensitive, original spelling kept |
| `did_you_mean(input, candidates)` | the best match within `default_tolerance` (a third of the length, at least 1) |
| `hint(input, candidates)` | `did you mean 'x'?` or an empty string |
| `fuzzy_score(pattern, text)` | -1 when the letters are not an ordered subsequence; otherwise higher is better (runs, word starts, short texts) |
| `rank(pattern, items)` | matching items, best first, ties in input order |

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/command_hints.wj` line for line and that the
`Output` block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/fuzzy_test.wj
tests/docs_test.wj
examples/command_hints.wj
```

## License

MIT OR Apache-2.0
