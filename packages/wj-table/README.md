# wj-table

Tables for terminals and markdown. Add rows, pick which columns are right-aligned, and render aligned plain text or a GitHub-flavoured markdown table. Fixed-point number formatting is included, matching Python's `f"{x:.3f}"`.

## API

```windjammer
use wj_table

let t = right_align(new(["kernel", "ms"]), [1])
let t = add_row(add_row(t, ["sieve", "1.5"]), ["matmul", "22.25"])
println(markdown(t))
println(plain(t, "  "))
```

- `new(headers)`, `right_align(t, columns)`, `add_row(t, cells)`
- `markdown(t)` - `| a | b |` with a `|---|---:|` rule
- `plain(t, gap)` - padded columns, trailing spaces trimmed
- `pad_left(text, width)`, `pad_right(text, width)`
- `fixed(x, decimals)`, `signed(x, decimals)`

## Layout

```
src/lib.wj
tests/table_test.wj
```

## Build / test

```bash
cd packages/wj-table
wj test --interpret
```

## License

MIT OR Apache-2.0
