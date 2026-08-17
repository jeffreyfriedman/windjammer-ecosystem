# wj-fetch

CLI HTTP GET: fetch a URL, pretty-print JSON when the body is JSON, otherwise print the raw body.

No Cargo crates, no `extern fn`, no `ffi/`.

## Usage

```bash
wj-fetch https://example.com/api
```

Stdout:

```
HTTP 200
{ ... pretty JSON or raw body ... }
```

## Layout

```
src/
  domain/url_arg.wj    # argv → URL
  domain/format.wj     # JSON pretty-print / fallback
  adapters/cli_args.wj # std::env.args
  adapters/http_get.wj # std::http GET
  main.wj              # composition root
tests/
  url_arg_test.wj
  format_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-fetch
$WJ test
$WJ build --release src
cd build && cargo run --release -- https://example.com
```

## License

MIT OR Apache-2.0
