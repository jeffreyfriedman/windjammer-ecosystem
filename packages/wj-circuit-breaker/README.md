# wj-circuit-breaker

Stop hammering a dependency that is down. A circuit breaker in pure Windjammer with three states: **closed** (calls
flow and outcomes are recorded), **open** (calls are refused during a cool-down) and **half-open** (a limited number of
probe calls decide whether to close again). It trips on a failure *rate* over a rolling window with a minimum call
count, so one unlucky call never opens it. Time is an argument in milliseconds.

```windjammer
use wj_circuit_breaker::new_breaker
use wj_circuit_breaker::breaker_config

fn main() {
    // trip at 50% failures over at least 10 calls in 60 s; stay open 30 s; 3 probes
    let mut b = new_breaker(breaker_config(50, 10, 60000, 30000, 3))
    if b.allow(now) {
        match call_backend() {
            Ok(_) => b.record_success(now),
            Err(_) => b.record_failure(now),
        }
    }
}
```

## Composing with wj-retry

`wj-retry` decides how long a retry policy would wait; the breaker knows when the next call is permitted. Combine them
with `retry_in`:

```windjammer
let policy_ms = wj_retry::delay_ms(backoff, attempt)
let wait = b.retry_in(now, policy_ms)   // the longer of the two
```

## How it behaves

| Call | Effect |
|---|---|
| `allow(now)` | may a call start? (half-open admits `probe_limit` probes) |
| `record_success(now)`, `record_failure(now)` | report the outcome |
| `state(now)`, `state_name(now)` | `Closed`, `Open`, `HalfOpen` |
| `wait_ms(now)` | time until calls are allowed again |
| `retry_in(now, retry_delay_ms)` | the retry delay, raised to the cool-down if longer |
| `failure_rate_pct(now)`, `trips()`, `reset()` | inspection and operator control |

All probes must succeed to close; one failed probe re-opens the breaker with a fresh cool-down.

## Layout

```
src/lib.wj
tests/breaker_test.wj
```

## License

MIT OR Apache-2.0
