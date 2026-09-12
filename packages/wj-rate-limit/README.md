# wj-rate-limit

Fixed-window rate limiting in pure Windjammer: per-key buckets, decisions, and standard HTTP headers.

## API

```windjammer
use wj_rate_limit

let lim = fixed_window(100, 60000)
let (bucket, result) = check_fixed_window(lim, empty_bucket(), now_ms)
if !result.allowed {
    let headers = rate_limit_headers(result)
}
```

## Layout

```
src/lib.wj
tests/rate_limit_test.wj
```

## License

MIT OR Apache-2.0
