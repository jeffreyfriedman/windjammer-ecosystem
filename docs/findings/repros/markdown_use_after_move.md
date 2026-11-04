# Use after move through by-value helpers (not yet reduced)

Standalone attempts (a helper called in an `&&` chain, in an `if` condition then an `else` branch, in a
`while` loop over `lines[i]`) all compile. The failing shape is in `packages/wj-markdown`:

1. In `packages/wj-markdown/src/block.wj`, remove the `.clone()` the package adds when it passes `line`, `l`,
   `after` or `last` to `indent_of`, `is_blank`, `drop_indent`, `atx`, `is_thematic`, `fence_open`,
   `is_fence_close`, `marker_of`, `is_quote`, `strip_quote`, `setext_level` and `interrupts_paragraph`
   (for example `interrupts_paragraph(line.clone())` becomes `interrupts_paragraph(line)`).
2. `cd packages/wj-markdown && wj test`: rustc stops with E0382 "borrow of moved value: `line`" (and `l`).

The generated Rust moves the value into the first helper call and puts the `.clone()` on a later use, for example
`!interrupts_paragraph(line) && !marker_of(line.clone()).ok` followed by `content.push(line)`. The helpers take
`String` by value even though they only read it, so the earlier call consumes the value.
