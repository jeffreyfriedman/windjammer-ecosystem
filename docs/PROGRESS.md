# Progress

Weekly build health for seed apps and packages.

| Week | Date | Apps / packages green | `extern fn` | Crates via interop | Compiler issues | Notes |
|------|------|----------------------|-------------|--------------------|-----------------|-------|
| 1 | 2026-08-15 | `wj-hello` | 0 | 0 | Multi-file CLI lib+bin (fixed upstream) | Initial skeleton |

## Weekly checklist

- [ ] No project-local `ffi/` / hand-written `.rs` / `extern fn` in apps
- [ ] CI green on pinned `wj`
- [ ] Changed seeds pass `wj test`
- [ ] [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) updated when production code gains std usage
- [ ] Compiler bugs get failing tests in `windjammer/tests/`
