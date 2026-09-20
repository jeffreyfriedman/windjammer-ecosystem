# wj-find

Week-one path find: walk a tree and filter with shell globs (`wj-fs-walk` + `wj-glob`).

See `docs/BETA_SUCCESS_PATH.md`.

## Layout

```
src/
  domain/find.wj
  domain/mod.wj
  main.wj
tests/
  find_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj

cd packages/wj-fs-walk && $WJ build --release src && cd -
cd packages/wj-glob && $WJ build --release src && cd -

cd apps/wj-find
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Demo walks `../wj-hello/src` for `**/*.wj` (or prints baked sample hits if the path is missing).
