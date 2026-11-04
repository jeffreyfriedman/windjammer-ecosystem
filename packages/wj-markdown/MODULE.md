# wj-markdown

## Purpose
Render a CommonMark subset to HTML with escaping on by default.

## Inputs / outputs
- In: Markdown text.
- Out: HTML text. Never fails; malformed constructs fall back to literal text.

## Layout
- `src/block.wj` (blocks), `src/inline.wj` (spans), `src/escape.wj` (escaping, URL safety), `src/lib.wj` (entry points).
- `tests/block_test.wj`, `tests/inline_test.wj`: markdown-it-py oracle. `tests/safety_test.wj`: injection cases.

## Invariants
- Pure: no I/O, no capabilities.
- No input character can produce a tag, attribute or script URL the renderer did not write itself.
