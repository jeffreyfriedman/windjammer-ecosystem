# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Sender`/`Receiver`/`Shared`/`Pending`/`Pool` / `task` / `parallel` / `wait`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status (tip 2026-09-17)

| Area | Status |
|------|--------|
| Generic `Sender<T>` / `Receiver<T>` / `send` / `recv` / `close` | ✅ lib + package tests |
| Generic `Shared<T>` / `SharedMap<K,V>` / `shared_new` / `shared_set` | ✅ + `SharedInt` / `SharedMapSI` aliases (P3.333b) |
| Generic `Pending<T>` / `wait` / `parallel_int` / `task_int` | ✅ + `PendingInt` / `PendingShared` aliases |
| Pool + `pool_is_alive` / `pool_shutdown` / shared-inbox | ✅ |
| Int helpers (`unbounded_int` / `send_int` / …) | ✅ |

**Open compiler gates:** AtomicI64 runtime re-export (`bug_std_sync_atomic_i64_wiring_test`). P3.350 (`wj build --release` → cargo `--release`) is tip GREEN — rebuild/install tip `wj` for dogfood.

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

Measured **2026-09-17** (WJ `benches/wj_baseline` + Rust `benches/rust_baseline`, both `cargo build --release` + LTO on WJ):

| Bench | N | WJ ms | Rust ms | ≈ ratio |
|------|---|------:|--------:|--------:|
| channel_sum | 1_000_000 | 21–23 | 21–22 | **~1.0–1.05×** |
| shared_incs (Mutex) | 1_000_000 | 9–10 | 8–9 | **~1.0–1.15×** |
| pool shared-inbox (4 workers) | 20_000 | 2–3 | 2–3 | **~1.0×** |

**Target:** ≤1.2× Rust on release. Prior ~4× reports were debug WJ vs release Rust (fixed by P3.350 + `run_release.sh`). AtomicI64 `Counter` still blocked on runtime re-export (would beat Mutex baseline once wired).

## Graduation

Candidate for **`std::sync`** / channels — see `windjammer/tests/STDLIB_ADOPTION_QUEUE.md` (`bug_std_sync_channel_shared_wiring_test`).

## License

MIT OR Apache-2.0
