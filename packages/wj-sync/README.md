# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Channel` / `Shared` / `Pending` / `Pool` / `task` / `parallel` / `.wait()`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status

| Area | Status |
|------|--------|
| Unbounded int channels, `clone_sender` | ✅ tested |
| Same-thread fan-out helpers (`channel_sum_range`) | ✅ tested |
| Bounded channels (`bounded_int` / `BoundedIntSender`) | ✅ tested |
| `SharedInt` / Counter, Once / Latch / Barrier | ✅ tested |
| `SharedMap` insert/len/get/has | ✅ tested |
| `parallel` / `Pending` | ✅ tested |
| Pool | ⏭ next |

| Cross-crate handle loop reassign (`tx = send_int(tx, …)`) | ⏸ **P3.290** — use package helpers until green |

## Usage

```windjammer
use wj_sync

let pair = unbounded_int()
let txs = clone_sender(pair.0)
send_int(txs.0, 1)
send_int(txs.1, 2)
let a = recv_int(pair.1)

// Same-thread dogfood (Shared + Channel):
let out = channel_sum_range(100)
```

## Graduation

Candidate for future **`std::sync`** / channel primitives once `parallel` and bounded channels compile. Stay ecosystem until OS-thread + `sync_channel` wiring is green.

## License

MIT OR Apache-2.0
