# wj-pipeline

Hexagonal dogfood app for `wj-sync`: channel fan-in + Shared counters + Pool + timed CLI.

```bash
unset CARGO_TARGET_DIR
wj build --release --library ../../packages/wj-sync/src -o ../../packages/wj-sync/build
wj test
wj build --release src
```

- `run_int_pipeline` / `time_int_pipeline` — same-thread channels + domain transform
- `run_pool_pipeline` / `time_pool_pipeline` — `pool_sum_double`

CLI prints count, sum, and `elapsed_ms` for both modes.

## License

MIT OR Apache-2.0
