# wj-form-parse

Minimal multipart dogfood: parse `multipart/form-data` into field pairs using only
`wj-multipart` (no notes-api dep graph). Week-one upload success path.

See `docs/BETA_SUCCESS_PATH.md`.

## Layout

```
src/
  domain/form.wj
  domain/mod.wj
  main.wj
tests/
  form_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj

cd packages/wj-multipart && $WJ build --release src && cd -
cd apps/wj-form-parse
$WJ test
$WJ build --release src
cd build && cargo run --release
```
