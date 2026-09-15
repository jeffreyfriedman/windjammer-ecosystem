# wj-pipeline

Hexagonal dogfood app for `wj-sync`: channel fan-in + Shared counters + pure stage transforms.

Same-thread runner via `wj_sync::channel_sum_range` until compiler **P3.286** (Pool) and **P3.290** (cross-crate handle loops) unlock full adapter ownership stress.

```bash
unset CARGO_TARGET_DIR
wj build --release --library ../../packages/wj-sync/src -o ../../packages/wj-sync/build
wj test   # from this app
```

## License

MIT OR Apache-2.0
