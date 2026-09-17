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

Measured **2026-09-17** (post pipeline hot-path opt — direct mpsc + local ints; WJ **debug** app; Rust `rustc -O` same-N / baseline):

| Bench | N | WJ ms | Rust ms | ≈ ratio |
|------|---|------:|--------:|--------:|
| channel_sum | 100_000 | 9 | 2 | ~4.5× |
| channel_sum | 1_000_000 | 88 | ~21 | ~4.2× |
| shared_incs | 100_000 | 3 | <1 | ~3×+ (Mutex; atomics blocked — see std adoption) |
| pool shared-inbox (4 workers) | 5_000 | 3 | <1 | ~3×+ |

Large-N Rust baseline (`benches/rust_baseline`, `cargo run --release`): channel 1e6 ~21ms, shared 1e6 ~8ms, pool 20k ~2ms.

Design target: within ~2× Rust (warn), soft-fail above 5×. Channel is now under 5×. Next: AtomicI64 `Counter` once `std::sync::atomic` wires (`bug_std_sync_atomic_i64_wiring_test`).

## Graduation

Candidate for **`std::sync`** / channels — see `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` (`bug_std_sync_channel_shared_wiring_test`).

## License

MIT OR Apache-2.0
