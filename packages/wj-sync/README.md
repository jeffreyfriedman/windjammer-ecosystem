# wj-sync

Idiomatic Windjammer concurrency — Go-style `sync` + CSP channels.

**Public vocabulary:** `Channel` / `Shared` / `Pending` / `Pool` / `task` / `parallel` / `.wait()`  
**Not in the public API:** `Arc`, `Mutex`, `async`, `spawn`, `go`, `Future`, `JoinHandle`

## Status

| Area | Status |
|------|--------|
| Unbounded int channels, `clone_sender` | ✅ tested |
| Bounded channels (`sync_channel`) | ⏸ blocked on compiler **P3.287** |
| `SharedInt`, Once / Latch / Barrier | ✅ tested |
| `parallel` / `Pending` / Pool | ⏸ blocked on compiler **P3.286** (`std::thread::spawn` emits `&(move \|\| …)`) |

Idiomatic pending sources: `src/pending.wj.blocked` + `tests/pending_test.wj.blocked`.

## Usage

```windjammer
use wj_sync

let pair = unbounded_int()
let txs = clone_sender(pair.0)
send_int(txs.0, 1)
send_int(txs.1, 2)
let a = recv_int(pair.1)
```

## License

MIT OR Apache-2.0
