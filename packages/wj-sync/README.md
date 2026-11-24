# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Sender`/`Receiver`/`Shared`/`Pending`/`Pool` / `task` / `parallel` / `wait`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status (tip 2026-09-18)

| Area | Status |
|------|--------|
| Generic `Sender<T>` / `Receiver<T>` / `send` / `recv` / `close` | ✅ lib + package tests |
| Generic `Shared<T>` / `SharedMap<K,V>` / `shared_new` / `shared_set` | ✅ + `SharedInt` / `SharedMapSI` aliases (P3.333b) |
| Generic `Pending<T>` / `wait` / `parallel_int` / `task_int` | ✅ + `PendingInt` / `PendingShared` aliases |
| Pool + `pool_is_alive` / `pool_shutdown` / shared-inbox | ✅ |
| `Counter` (`AtomicI64`) | ✅ tip + package tests (P3.370/P3.380 void i64 peers) |
| Int helpers (`unbounded_int` / `send_int` / …) | ✅ |

**Compiler:** P3.350 release/LTO + AtomicI64 wiring + void `AtomicI64::new`/`fetch_add` i64 peers (P3.380) tip GREEN — rebuild/install tip `wj` for dogfood.

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

**Fair comparison = release vs release.** Soft smoke gates in `tests/bench_*_test.wj` run under `wj test` (debug) and are not ratio evidence.

```bash
packages/wj-sync/benches/run_release.sh
```

Measured **2026-09-19** (tip wj, quiet median of 7 runs, release+LTO):

| Bench | N | WJ ms | Rust ms | ≈ ratio |
|------|---|------:|--------:|--------:|
| channel_sum | 1_000_000 | ~22 | ~21 | **~1.05×** |
| shared_incs (Mutex) | 1_000_000 | ~9 | ~8 | **~1.12×** |
| counter_incs (AtomicI64) | 10_000_000 | ~22 | ~22 | **~1.0×** |
| pool shared-inbox round-robin (4 workers) | 100_000 | ~10 | ~10 | **~1.0×** |

**Target:** ≤1.2× Rust on release — **met** for channel / Shared / Counter / pool under quiet load. Prior Counter ~1.5× at N=1M was ms-timer noise; N=10M shows parity. Prior ~4× reports were debug WJ vs release Rust (P3.350).

## Graduation

**Package readiness:** green for idiomatic channel / Shared (Mutex) / Pending / Pool / AtomicI64 `Counter` (49 package tests).

**`std::sync` wrap:** tip GREEN for `unbounded`/`send`/`recv` + `shared`/`shared_add`/`shared_get` (`bug_std_sync_channel_shared_wiring_test`) and AtomicI64 (`bug_std_sync_atomic_i64_wiring_test`). `std/sync.wj` documents the vocabulary; `wj-sync` `unbounded`/`bounded` thin-wrap `std::sync`; Pending/Pool/generics stay package sugar.

## License

MIT OR Apache-2.0
