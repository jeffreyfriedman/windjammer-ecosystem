# wj-retry

Pure backoff + blocking pause helpers for retry loops.

## API

```windjammer
use wj_retry

let backoff = Backoff {
    initial_ms: 100,
    multiplier: 2,
    max_ms: 1000,
}

let mut attempt = 0
while should_retry(attempt, 5) {
    // do work…
    pause_ms(delay_ms(backoff, attempt))
    attempt = attempt + 1
}
```

| Function | Description |
|---|---|
| `delay_ms(backoff, attempt)` | Exponential delay capped at `max_ms` |
| `should_retry(attempt, max_attempts)` | Whether attempt index is still in range |
| `pause_ms(ms)` | Blocking wait (`std::time` spin until `std::async_runtime` import is green) |

## Layout

```
src/lib.wj
tests/retry_test.wj
```

## License

MIT OR Apache-2.0
