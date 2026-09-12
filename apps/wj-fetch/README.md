# wj-fetch

CLI HTTP GET: fetch a URL, pretty-print JSON when the body is JSON, otherwise print the raw body.

No Cargo crates, no `extern fn`, no `ffi/`.

## Usage

```bash
wj-fetch https://example.com/api
wj-fetch --fail https://example.com/api          # exit 1 on non-2xx
wj-fetch --retries=2 https://example.com/api     # retry 429/502/503/504
wj-fetch --timeout=10 https://example.com/api    # timeout (adapter pending P3.219)
wj-fetch --output=response.json https://example.com/api
wj-fetch -o body.txt https://example.com/api
wj-fetch -s https://example.com/api              # body only (no HTTP status line)
wj-fetch -s -o body.txt https://example.com/api  # write file, no stdout
```

Environment:

- `FETCH_FAIL_ON_ERROR=1` — same as `--fail`
- `FETCH_MAX_RETRIES=N` — default retry budget (overridden by `--retries=N`)
- `FETCH_TIMEOUT_SECS=N` — default timeout in seconds (overridden by `--timeout=N`; adapter applies once P3.219 greens)

Stdout:

```
HTTP 200
{ ... pretty JSON or raw body ... }
```

## Layout

```
src/
  domain/url_arg.wj      # argv → FetchRequest (--fail, --retries, --timeout, --output|-o)
  domain/config.wj       # env defaults (FETCH_*)
  domain/parse.wj        # parse_nonneg_int via strings.parse_i32
  domain/retry_policy.wj # transient status + wj-retry should_retry
  domain/run.wj          # retry loop with backoff pause
  domain/format.wj       # JSON pretty-print, resolve_fetch_output (--output, --silent)
  adapters/cli_args.wj   # std::env.args
  adapters/http_get.wj   # std::http GET
  adapters/pause.wj      # delegates to wj-retry::pause_ms
  adapters/write_out.wj  # fs.write for --output
  main.wj                # composition root
tests/
  url_arg_test.wj
  format_test.wj
  parse_test.wj
  retry_test.wj
  fetch_test.wj
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
