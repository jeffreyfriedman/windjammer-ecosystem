# Progress

Weekly build health for seed apps and packages.

| Week | Date | Apps / packages green | `extern fn` | Crates via interop | Compiler issues | Notes |
|------|------|----------------------|-------------|--------------------|-----------------|-------|
| 1 | 2026-08-15 | `wj-hello` | 0 | 0 | Multi-file CLI lib+bin (fixed upstream) | Initial skeleton |
| 2 | 2026-08-16 | `wj-hello`, `wj-dotenv` | 0 | 0 | Flat `lib.wj` + `wj test`, `strings::join` Vec, `fs` AsRef path (fixed upstream) | Idiomatic package seed |
| 2 | 2026-08-16 | + `wj-fetch` (domain + adapters) | 0 | 0 | `json.parse` borrow, HTTP status `u16`, `process::exit` path (fixed upstream) | Wave 1 CLI HTTP GET |

## Weekly checklist

- [ ] No project-local `ffi/` / hand-written `.rs` / `extern fn` in apps
- [ ] CI green on pinned `wj`
- [ ] Changed seeds pass `wj test`
- [ ] [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) updated when production code gains std usage
- [ ] Compiler bugs get failing tests in `windjammer/tests/`
