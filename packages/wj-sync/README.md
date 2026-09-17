# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Sender`/`Receiver`/`Shared`/`Pending`/`Pool` / `task` / `parallel` / `wait`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status (tip 2026-09-17)

| Area | Status |
|------|--------|
| Generic `Sender<T>` / `Receiver<T>` / `send` / `recv` / `close` | ✅ lib + 44 package tests |
| Generic `Shared<T>` / `SharedMap<K,V>` / `shared_new` / `shared_set` | ✅ + `SharedInt` / `SharedMapSI` aliases (P3.333b) |
| Generic `Pending<T>` / `wait` / `parallel_int` / `task_int` | ✅ + `PendingInt` / `PendingShared` aliases |
| Pool + `pool_is_alive` / `pool_shutdown` / shared-inbox | ✅ |
| Int helpers (`unbounded_int` / `send_int` / …) | ✅ |

**Open compiler gate:** P3.342 (inject unbound `Sender<T>` on named assign from `clone_sender`/`send`). Tests avoid those binds.

**Design:** `docs/superpowers/specs/2026-09-16-wj-sync-generics-stdlib-design.md`


## Usage

```windjammer
use wj_sync

let pair = unbounded_int()
let txs = clone_sender(pair.0)
send_int(txs.0, 1)
send_int(txs.1, 2)
let a = recv_int(pair.1)

let out = channel_sum_range(100)
assert_eq(wait_int(parallel_add(2, 3)), 5)
```

## Benches

Soft gates in `tests/bench_smoke_test.wj` + `tests/bench_throughput_test.wj` (100k channel/shared, 5k pool inbox).

Measured **2026-09-17** on this laptop (WJ app **debug** build via tip `wj` 0.50.0; Rust `rustc -O` same-N):

| Bench | N | WJ ms | Rust ms | ≈ ratio |
|------|---|------:|--------:|--------:|
| channel_sum | 100_000 | 14 | 2 | ~7× (soft >5×) |
| shared_incs | 100_000 | 3 | <1 | ~3×+ |
| pool shared-inbox (4 workers) | 5_000 | 3 | <1 | ~3×+ |

Large-N Rust baseline (`benches/rust_baseline`, `cargo run --release`):

| Bench | N | elapsed_ms |
|------|---|------------|
| channel_sum | 1_000_000 | ~21 |
| shared_incs | 1_000_000 | ~8 |
| pool_double (4 workers) | 20_000 | ~2 |

Design target: within ~2× Rust (warn), soft-fail note above 5×. Channel is the current outlier; re-measure with WJ **release** once tip `wj` builds cleanly.

## Graduation

Candidate for **`std::sync`** / channels — see `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` (`bug_std_sync_channel_shared_wiring_test`).

## License

MIT OR Apache-2.0
