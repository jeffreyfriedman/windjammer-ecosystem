# wj-retry

Pure backoff helpers for retry loops (no sleep — callers decide how to wait).

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
    let wait = delay_ms(backoff, attempt)
    // sleep(wait) in an adapter
    attempt = attempt + 1
}
```

| Function | Description |
|---|---|
| `delay_ms(backoff, attempt)` | Exponential delay capped at `max_ms` |
| `should_retry(attempt, max_attempts)` | Whether attempt index is still in range |

## Layout

```
src/lib.wj
tests/retry_test.wj
```

## License

MIT OR Apache-2.0
