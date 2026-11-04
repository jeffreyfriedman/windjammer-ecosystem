# wj-metrics

Counters, gauges and histograms with labels, rendered in the Prometheus text exposition format. Series are keyed by name plus label set, label order never splits a series, and output is sorted so scrapes and tests are stable.

```windjammer
use wj_metrics::new_registry
use wj_metrics::Kind

let mut r = new_registry()
r.register("http_requests_total", "Requests served.", Kind::Counter, Vec::new())?
r.counter_inc("http_requests_total", [("method", "GET"), ("code", "200")])?
r.histogram_observe("latency_seconds", Vec::new(), 0.042)?
println(r.render())
```

## API

- `register(name, help, kind, buckets)` (optional; metrics also register on first use)
- `counter_inc`, `counter_add`, `gauge_set`, `gauge_add`, `histogram_observe`, `value_of`
- `render()`: Prometheus text format 0.0.4 with label and help escaping, `+Inf` bucket, `_sum` and `_count`
- `snapshot()` and `snapshot_json` / `parse_snapshot`: the same data as typed `@json` structs
- Names and labels are validated; a counter cannot decrease; a name keeps one kind.

## Layout

```
src/lib.wj
tests/metrics_test.wj
```

## Build / test

```bash
cd packages/wj-metrics
wj test --interpret
wj test
```
