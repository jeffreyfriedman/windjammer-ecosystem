# wj-first-hour

Std-only first-hour showcase: builds a small session card from tip-green `std::*`
(uuid, time, path, config.parse_flat, encoding.url_encode). No Cargo crates, no `extern fn`.

See `docs/BETA_SUCCESS_PATH.md`.

## Layout

```
src/
  domain/card.wj   # build_card
  domain/mod.wj
  main.wj
tests/
  card_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj

cd apps/wj-first-hour
$WJ test
$WJ build --release src
cd build && cargo run --release
```
