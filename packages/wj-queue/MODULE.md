# wj-queue

## Purpose
In-memory job queue with priorities, delays, leases, exponential-backoff retries and dead letters.

## Inputs / outputs
- In: payload strings, priorities, delays, and `now` in milliseconds on every call that depends on time.
- Out: `Job` values from `pop`, booleans from `ack`/`nack`/`extend`, `Stats`, the dead-letter list.

## Layout
- `src/lib.wj` (`Queue`, `Job`, `Config`, `backoff_ms`), `tests/queue_test.wj`.

## Invariants
- A job is handed to one worker at a time; its `attempts` count rises on every hand-out.
- An expired lease is treated as a failure at the moment the lease ended, not when it was noticed.
- Among equal priorities the oldest job wins.
- No clock, no I/O: `[capabilities]` is empty.
