# wj-pipeline

Hexagonal dogfood app for `wj-sync`: channel fan-in + Shared counters + Pool + pure stage transforms.

```bash
unset CARGO_TARGET_DIR
wj build --release --library ../../packages/wj-sync/src -o ../../packages/wj-sync/build
wj test
```

- `run_int_pipeline` — same-thread channels + domain transform
- `run_pool_pipeline` — `pool_sum_double` (parallel workers)

## License

MIT OR Apache-2.0
