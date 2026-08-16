# wj-hello

Minimal Windjammer CLI: prints a version string and exits 0.

No Cargo crates, no `extern fn`, no `ffi/`.

## Layout

```
src/
  domain/version.wj
  main.wj
tests/
  version_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-hello
$WJ test
$WJ build --release src
cd build && cargo run --release
```
