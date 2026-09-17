# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Channel` / `Shared` / `Pending` / `Pool` / `task` / `parallel` / `.wait()`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status

| Area | Status |
|------|--------|
| **Generic** `Sender<T>` / `Receiver<T>` / `send` / `recv` / `close` | ❌ tip RED — P3.331 (`value: &T`) + P3.332 (`rx.clone()`) |
| Int aliases (`unbounded_int` / `send_int` / …) | ❌ tip RED (same gates; wrappers over generics) |
| `SharedInt` / Counter, Once / Latch / Barrier | ✅ previously green (blocked while channel lib RED) |
| `SharedMap` insert/len/get/has | ✅ previously green (blocked while channel lib RED) |
| `parallel` / `Pending` | ✅ previously green (blocked while channel lib RED) |
| Pool (`pool_run_double` / `pool_shared_inbox_sum`) | ✅ previously green (blocked while channel lib RED) |

**Design:** `docs/superpowers/specs/2026-09-16-wj-sync-generics-stdlib-design.md` (Option B — generics before done).  
**Do not** reshape packages to dodge P3.331/332 — fix is in the compiler.

## Usage

```windjammer
use wj_sync

let pair = unbounded_int()
let txs = clone_sender(pair.0)
send_int(txs.0, 1)
send_int(txs.1, 2)
let a = recv_int(pair.1)

let out = channel_sum_range(100)
let p = parallel_add(2, 3)
assert_eq(wait_int(p), 5)
```

## Benches

Soft wall-clock smoke tests in `tests/bench_smoke_test.wj` (10k channel/shared, 500 pool).

Rust baseline (`benches/rust_baseline`, `cargo run --release`, 2026-09-15 laptop):

| Bench | N | elapsed_ms |
|------|---|------------|
| channel_sum | 1_000_000 | ~20 |
| shared_incs | 1_000_000 | ~8 |
| pool_double (4 workers) | 20_000 | ~1–2 |

## Graduation

Candidate for future **`std::sync`** / channel primitives.

## License

MIT OR Apache-2.0
