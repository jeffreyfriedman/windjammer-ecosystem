# wj-token-bucket

Rate limiters for services that want smooth traffic shaping, in pure Windjammer: a **token bucket** (bursts up to a
capacity, steady refill) and a **sliding window** (at most N requests in any rolling span, with no boundary burst).
Time is an argument in milliseconds, so every decision is deterministic and easy to test. It complements
`wj-rate-limit`, which provides the fixed window and the `RateLimit-*` header helpers.

```windjammer
use wj_token_bucket::token_bucket
use wj_token_bucket::sliding_window
use wj_token_bucket::retry_after_header

fn main() {
    let mut bucket = token_bucket(10, 5, 1000)   // burst of 10, refill 5 tokens per second
    let d = bucket.try_acquire(now)
    if !d.allowed {
        println(retry_after_header(d))           // Retry-After: 1
    }

    let mut window = sliding_window(100, 60000)  // 100 requests per rolling minute
    let ok = window.try_acquire(now).allowed
}
```

## How it behaves

| Call | Effect |
|---|---|
| `token_bucket(capacity, refill_tokens, interval_ms)` | a full bucket |
| `try_acquire(now)`, `try_take(now, n)` | take tokens or learn exactly how long to wait (`retry_after_ms`) |
| `available(now)` | whole tokens ready |
| `sliding_window(limit, window_ms)` | rolling-window limiter |
| `try_acquire(now)`, `remaining(now)`, `used(now)` | admit, or report when the oldest request leaves the window |
| `retry_after_header(decision)` | `Retry-After: <seconds>` for a denied decision |

Refill is exact: tokens are counted in units of 1/interval, so a bucket that gains one token every 3 ms never drifts.
A `retry_after_ms` of -1 means the request can never succeed (larger than the capacity, or a bucket with no refill).

## Layout

```
src/lib.wj
tests/token_bucket_test.wj
```

## License

MIT OR Apache-2.0
