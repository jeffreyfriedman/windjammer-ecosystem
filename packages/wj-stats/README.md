# wj-stats

Small descriptive statistics over `Vec<f64>`: mean, median, min, max, sorted, and population and sample standard deviation. Definitions follow Python's `statistics` module, so benchmark and report tools agree across languages.

## API

```windjammer
use wj_stats

let times = [0.31, 0.29, 0.30, 0.52]
println("median ${median(times)} stdev ${stdev(times)}")
```

- `sum`, `mean`, `min`, `max`, `sorted`
- `median` - mean of the two middle samples for even counts
- `pstdev` (population), `stdev` (sample; `0.0` below two samples)

## Layout

```
src/lib.wj
tests/stats_test.wj
```

## Build / test

```bash
cd packages/wj-stats
wj test --interpret
```

## License

MIT OR Apache-2.0
