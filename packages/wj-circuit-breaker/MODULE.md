# wj-circuit-breaker

## Purpose
Closed / open / half-open circuit breaker with a failure-rate window and probe limits, on an injected clock.

## Inputs / outputs
- In: `Config` (rate, minimum calls, window, cool-down, probe limit), call outcomes, `now` in milliseconds.
- Out: `allow` decisions, state, wait time, failure rate, trip count.

## Layout
- `src/lib.wj` (`Breaker`, `Config`, `State`), `tests/breaker_test.wj`.

## Invariants
- Open to half-open happens only when `now >= opened_at + open_ms`.
- Half-open admits at most `probe_limit` calls; any failed probe re-opens, all successes close.
- Outcomes recorded while open are ignored; tripping clears the window.
- No clock, no I/O: `[capabilities]` is empty.
