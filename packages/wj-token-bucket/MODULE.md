# wj-token-bucket

## Purpose
Token-bucket and sliding-window rate limiters driven by an injected millisecond clock.

## Inputs / outputs
- In: capacity/refill or limit/window settings, and `now` in milliseconds on every call.
- Out: `Decision { allowed, remaining, retry_after_ms }`, token and slot counts, a `Retry-After` header line.

## Layout
- `src/lib.wj` (`TokenBucket`, `SlidingWindow`, `Decision`), `tests/token_bucket_test.wj`.

## Invariants
- A denied request never changes the bucket or the window.
- Tokens never exceed capacity; a clock that moves backwards mints nothing.
- No clock, no I/O: `[capabilities]` is empty.
