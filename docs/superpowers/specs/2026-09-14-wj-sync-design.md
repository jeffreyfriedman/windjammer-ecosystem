# wj-sync — Concurrency Reference Library Design

**Date:** 2026-09-14  
**Status:** Approved for implementation (TDD)  
**Package:** `packages/wj-sync`  
**Dogfood app:** `apps/wj-pipeline`

## Goal

A pure-Windjammer concurrency library that is both:

1. **Useful** — Go-style `sync`-class primitives + CSP channels + worker pools that apps can depend on  
2. **Compiler dogfood** — maximal ownership/lifetime stress across tasks, shared state, and message passing  
3. **Reference-quality** — the idiomatic guide for high-class Windjammer concurrency  
4. **Fast** — stay close to optimized Rust (`std::sync` / `mpsc` / OS workers under the hood; no Tokio in this package)

## Non-goals

- No rust-interop crates, no Tokio, no rayon crates in `wj-sync` / `wj-pipeline`  
- No Rust leakage in the **public** API (`Arc`, `Mutex`, `Future`, `JoinHandle`, `spawn`, `async`, `go`)  
- Not a permanent duplicate of std once graduated — see graduation checklist below

## Architecture

```
packages/wj-sync/
  src/lib.wj           # re-exports
  src/channel.wj       # message-passing
  src/shared.wj        # Shared / SharedMap / Counter
  src/coord.wj         # Barrier / Once / Latch
  src/pending.wj       # Pending + task / parallel
  src/pool.wj          # worker Pool
  tests/*_test.wj
  benches/*_bench.wj   # throughput / latency vs Rust-class targets

apps/wj-pipeline/
  src/domain/…         # pipeline stages (pure WJ)
  src/adapters/…       # CLI / timing
  src/main.wj
  tests/*_test.wj
```

**Rules (AGENTS.md):** idiomatic `.wj` only; if the compiler cannot handle a shape, leave it idiomatic, file `windjammer/tests/` repro, fix compiler, resume. No structural workarounds.

## Terminology (Windjammer vocabulary)

| Concept | Windjammer term | Reject |
|--------|-----------------|--------|
| Default call | *(no prefix)* | — |
| Cooperative concurrent work | `task` | `async`, `go` |
| Parallel worker | `parallel` | `spawn`, `thread` (as API name) |
| Unfinished result | `Pending<T>` | `Future`, `JoinHandle` |
| Wait for result | `.wait()` | `.await`, `.join()` |
| Message conduit | `Channel` / `Sender` / `Receiver` | — |
| Shared mutable | `Shared<T>` | exposing `Arc` / `Mutex` |
| Fan-out workers | `Pool` | — |
| Coordination | `Barrier`, `Once`, `Latch` | — |

Package name **`wj-sync`** mirrors Go’s [`sync`](https://pkg.go.dev/sync). Channels live in this package until language/`std::channel` exists (Go puts channels in the language).

## Arc vs Mutex (compiler responsibility)

`Shared<T>` is the Windjammer concept. `Arc` / `Mutex` / `RwLock` / atomics are **codegen choices**:

| Usage | Likely emit |
|-------|-------------|
| Single owner, no share | owned `T` |
| Immutable, multi-task | `Arc<T>` |
| Mutated across tasks | `Arc<Mutex<T>>` (or `RwLock` / atomics when appropriate) |

The package never asks authors to choose Arc vs Mutex. Wrong codegen → compiler bug + repro.

Until inference is complete, implementation may use Windjammer/`std` sync helpers that already map to Rust `std::sync`, but **public** names stay `Shared` / `Channel` / `Pending`.

## Public API

### Message-passing (`channel.wj`)

- `unbounded<T>() -> (Sender<T>, Receiver<T>)`
- `bounded<T>(capacity: int) -> (Sender<T>, Receiver<T>)`
- `Sender.send` / `try_send` / `close` (cloneable senders)
- `Receiver.recv` / `try_recv`
- Ownership **moves** with messages (dogfood)

### Shared-state (`shared.wj`)

- `Shared::new(T)` with `.with(...)` / read / replace helpers
- `SharedMap` (key/value concurrent map — full impl, not std stub)
- `Counter` (increment / get; preferred path for hot counters)

### Coordination (`coord.wj`)

- `Barrier` (N parties)
- `Once` (run-once)
- `Latch` (countdown)

### Execution (`pending.wj`)

- `task(work) -> Pending<T>` — cooperative / multiplexed concurrent unit  
- `parallel(work) -> Pending<T>` — separate worker  
- `Pending.wait() -> T`  
- Until call-site prefixes exist in the language, these are library functions; API names match the future language vocabulary.

### Pool (`pool.wj`)

- Fixed worker count, job channel in, result channel out, clean shutdown

## Dogfood app: `wj-pipeline`

Multi-stage fan-out / fan-in pipeline:

1. Ingest jobs on a channel  
2. `Pool` of workers transforms jobs  
3. Fan-in results via channel / Shared counters  
4. CLI reports counts + wall time  

Proves end-to-end ownership across stages.

## Testing (TDD)

1. Write failing `tests/<area>_test.wj`  
2. `unset CARGO_TARGET_DIR && $WJ test` — confirm fail  
3. Minimal idiomatic impl  
4. Green  
5. Compiler stuck → repro in `windjammer/tests/`, fix, resume  

Coverage must include: close races, multi-sender, Shared across `parallel`, pool shutdown, ownership move on send.

## Benchmarks (performance gate)

Stay close to optimized Rust. Benches live under `packages/wj-sync/benches/` (or `tests/*_bench.wj` if bench discovery is limited) and measure:

| Bench | Metric | Target |
|-------|--------|--------|
| Unbounded channel ping-pong | msgs/sec | within ~2× of Rust `mpsc` microbench on same machine |
| Bounded channel fill/drain | msgs/sec | within ~2× of Rust `sync_channel` |
| Shared counter increments (N workers) | incs/sec | within ~2× of Rust `Arc<Mutex<u64>>` or better via `Counter` |
| Pool throughput (CPU-light jobs) | jobs/sec | within ~2× of hand-rolled Rust thread pool |
| Pipeline app end-to-end | jobs/sec + p50/p99 latency | regress gate: record baseline in README |

Procedure:

1. Implement WJ bench harness using `std::time` (wall clock)  
2. Optional: sibling Rust microbench in docs or `benches/rust_baseline/` for apples-to-apples  
3. Fail CI / local gate if WJ is worse than **5×** Rust on the same host (soft), warn at **2×** (hard goal)  
4. Document results in `packages/wj-sync/README.md`

No Tokio runtime in benches — pure `std` sync/threads baselines.

## Stdlib graduation (post-green)

After tests + benches are green, decide and update `docs/STDLIB_GRADUATION.md`:

| Outcome | When |
|---------|------|
| Graduate core into `std::sync` (+ channels) | API stable, benches near Rust, language ready |
| Keep package, thin-wrap std later | Std grows thin tokio wrappers instead — package remains reference + sugar |
| Feed WJ-CONC-01 | Terminology (`task` / `parallel` / `Pending` / `.wait()`) should update the RFC |

`std::async` thin tokio wrappers remain for I/O; **`wj-sync` is the synchronization/CSP reference**, not an async runtime.

## Error handling

- Channel closed → `Result` / explicit error strings (idiomatic WJ, no panics in library paths)  
- Lock poison → treat as fatal error string or recover policy documented on `Shared`  
- Pool shutdown → drain or reject new jobs; tests cover both

## Constraints

- Pure Windjammer sources under `apps/` / `packages/` (ban-list: no `extern fn`, no `ffi/`)  
- Local `wj`: `unset CARGO_TARGET_DIR && windjammer/target/release/wj`  
- Hexagonal app layout for `wj-pipeline`  
- TDD mandatory; commits only when user asks  
