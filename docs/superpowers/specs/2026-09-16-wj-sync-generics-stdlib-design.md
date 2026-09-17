# wj-sync generics + stdlib graduation

**Date:** 2026-09-16  
**Status:** Approved  
**Supersedes / extends:** `2026-09-14-wj-sync-design.md` (int-specialized MVP → generics-first completion)  
**Package:** `packages/wj-sync`  
**Dogfood:** `apps/wj-pipeline`  
**Graduation target:** `std::sync` (+ channel primitives)

## Decision

**Option B:** Do not call `wj-sync` “done” until the public API is **generic** (`Channel<T>`, `Shared<T>`, `Pending<T>`, typed `Pool`). Int-only helpers may exist only as temporary aliases during migration; they are not the finished surface.

**Other packages** (`wj-toml`, `wj-semver`, `wj-timefmt`, `wj-config`, …): remain **idiomatic**. No reshapes to dodge tip RED. Open gates (P3.325 / P3.326 / P3.329 and successors) stay in `windjammer/tests/` + `COMPILER_REPRO_QUEUE.md` for the compiler agent.

**Windjammer source:** This ecosystem agent does **not** edit compiler/`std` implementation. Promotion work here = idiomatic package + failing std adoption tests + queue/docs. The compiler agent owns greening gates and landing `std::sync`.

## Goals

1. Idiomatic generic concurrency reference in pure Windjammer  
2. Maximal ownership/lifetime dogfood for the compiler  
3. Real benches (WJ vs Rust baseline) within ~2× soft goal / 5× soft fail  
4. Clear path: package green → thin `std::sync` → `wj-sync` re-exports/sugar  

## Non-goals

- Tokio / rayon / rust-interop in `wj-sync` or `wj-pipeline`  
- Public `Arc` / `Mutex` / `spawn` / `async` / `Future` / `JoinHandle`  
- Workarounds in ecosystem packages for open compiler bugs  
- Editing `windjammer/src/` or shipping `std::sync` from this agent  

## Public API (target)

### Channel (`channel.wj`)

- `unbounded<T>() -> (Sender<T>, Receiver<T>)`
- `bounded<T>(capacity: int) -> (Sender<T>, Receiver<T>)`
- `send` / `try_send` / `recv` / `try_recv` / `close`
- Cloneable senders; ownership **moves** with messages

### Shared (`shared.wj`)

- `Shared<T>` — new / get / set / with (names stay Windjammer; Arc/Mutex are codegen)
- `SharedMap<K, V>` — insert / get / has / len
- `Counter` — preferred hot counter path

### Coord (`coord.wj`)

- `Barrier`, `Once`, `Latch` — keep; harden tests under generic callers

### Pending (`pending.wj`)

- `task(work) -> Pending<T>`
- `parallel(work) -> Pending<T>`
- `wait(pending) -> T` (method `.wait()` when language sugar is solid)

### Pool (`pool.wj`)

- Fixed worker count; typed job inbox + result outbox  
- Shared-inbox workers; **real shutdown** (drain or reject new jobs; tested)  
- No Arc/Mutex in the **public** API  

## Migration from int MVP

| Today (int) | Target |
|-------------|--------|
| `unbounded_int` / `send_int` / … | `unbounded` / `send` / … over `T` |
| `SharedInt` / `shared_int_*` | `Shared<int>` (+ keep Counter) |
| `parallel_add` / `PendingInt` | `parallel` / `Pending<T>` |
| `pool_run_double` / `pool_shared_inbox_sum` | generic pool run + shared-inbox |

TDD order: failing generic tests first → leave idiomatic RED → file compiler repro → green when tip allows → delete int-primary exports.

## Compiler contract

1. Prefer idiomatic generic `.wj` even when tip RED  
2. Minimal repro in `windjammer/tests/` + queue entry for every blocker  
3. Resume package work only after the gate is tip GREEN (or a new gate is filed)  
4. Known related: P3.290 (owned handle loop) tip GREEN — use real loops; drop obsolete comments/helpers when generics land  

## Benchmarks

- WJ wall-clock benches under `benches/` or `tests/*_bench.wj` if discovery limited  
- Compare to `benches/rust_baseline` on the same host  
- Metrics: channel msgs/sec, Shared/Counter incs/sec, Pool jobs/sec, pipeline e2e  
- Document ratios in `packages/wj-sync/README.md`  

## Stdlib promotion

After generics + tests + benches:

| Step | Owner |
|------|--------|
| Failing `std::sync` / channel adoption tests | Ecosystem agent (tests + `STDLIB_ADOPTION_QUEUE.md`) |
| Implement `std::sync` + wire channels | Compiler agent |
| Thin-wrap `wj-sync` → std; update `STDLIB_GRADUATION.md` | Ecosystem after tip green |

`wj-sync` remains the **CSP / sync reference** until std is tip-green; then package becomes sugar/re-export, not a permanent duplicate.

## Success criteria

- [ ] Public API matches generic table above (no int-only as primary)  
- [ ] Package + `wj-pipeline` tests green on tip  
- [ ] Bench ratios recorded; no silent >5× regression vs Rust baseline  
- [ ] Std adoption tests + graduation row ready for compiler agent  
- [ ] Sibling packages still idiomatic; RED only via filed gates  

## Spec self-review

- No placeholders for API names beyond acknowledged `.wait()` sugar  
- No contradiction with AGENTS.md (no workarounds; no hand Rust in packages)  
- Scope is one package + graduation path; toml/semver/timefmt explicitly out of reshape scope  
