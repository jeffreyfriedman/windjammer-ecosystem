# wj-metrics

## Purpose
In-process metrics and their Prometheus text rendering.

## Inputs / outputs
- In: metric names, label pairs, numbers.
- Out: exposition text, or a typed `Snapshot` (`@json`).

## Layout
- `src/lib.wj`: registry, rendering, snapshot. `tests/metrics_test.wj`: Prometheus documentation examples, bucket counts checked against Python.

## Invariants
- Pure: no I/O, no capabilities. Rendering is deterministic (families by name, series by label text).
- Histogram buckets are cumulative, strictly ascending, and always end with `+Inf`.
