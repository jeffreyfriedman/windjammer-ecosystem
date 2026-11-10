# wj-cli-ui

## Purpose
Colour, text measurement, progress rendering and prompts for command-line tools.

## Inputs / outputs
- In: strings, numbers, a `Theme`; for prompts a list of answer lines.
- Out: strings to print (the prompt functions print their questions).

## Layout
- `src/style.wj`, `src/ansi.wj`, `src/text.wj`, `src/layout.wj`, `src/progress.wj`, `src/prompt.wj`; `tests/ui_test.wj`, `tests/polish_test.wj` (Python oracle values).

## Invariants
- Capabilities: `env.read` for `NO_COLOR` and `TERM`, `fs.read` for `/dev/stdin`. Everything else is pure.
- With colour off every styling function returns its input unchanged.
