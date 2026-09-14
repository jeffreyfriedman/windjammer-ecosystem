# wj-sync + wj-pipeline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship pure-Windjammer `wj-sync` (channels, Shared, coordination, Pending/task/parallel, Pool) plus `wj-pipeline` dogfood app, with TDD and Rust-class benchmarks.

**Architecture:** Flat package modules under `packages/wj-sync/src/` with idiomatic Windjammer public names; `apps/wj-pipeline` hexagonal app consumes the library. Arc/Mutex are codegen details under `Shared`. No Tokio.

**Tech Stack:** Windjammer (local `wj` 0.50+), `std::sync` / `std::time` as available, Windjammer test framework, wall-clock benches.

**Spec:** `docs/superpowers/specs/2026-09-14-wj-sync-design.md`

## Global Constraints

- Pure Windjammer only — no `extern fn`, no `ffi/`, no rust-interop Tokio/rayon
- Public API: `Channel`/`Sender`/`Receiver`, `Shared`/`SharedMap`/`Counter`, `Barrier`/`Once`/`Latch`, `Pending`/`task`/`parallel`, `Pool` — never expose `Arc`/`Mutex`/`async`/`spawn`/`go`
- TDD: failing test → `$WJ test` red → implement → green; compiler bugs → `windjammer/tests/` repro, fix compiler, no app workarounds
- Build: `unset CARGO_TARGET_DIR && /Users/jeffreyfriedman/src/wj/windjammer/target/release/wj`
- Bench gate: warn if >2× Rust baseline; fail soft gate if >5×
- Commits only when user asks

---

## File map

| Path | Responsibility |
|------|----------------|
| `packages/wj-sync/wj.toml` | Package manifest |
| `packages/wj-sync/src/lib.wj` | Re-exports |
| `packages/wj-sync/src/channel.wj` | Channels |
| `packages/wj-sync/src/shared.wj` | Shared state |
| `packages/wj-sync/src/coord.wj` | Barrier / Once / Latch |
| `packages/wj-sync/src/pending.wj` | task / parallel / Pending |
| `packages/wj-sync/src/pool.wj` | Worker pool |
| `packages/wj-sync/tests/*_test.wj` | TDD + ownership stress |
| `packages/wj-sync/benches/*_bench.wj` | Throughput benches |
| `packages/wj-sync/benches/rust_baseline/` | Optional Rust comparison |
| `packages/wj-sync/README.md` | API + bench results |
| `apps/wj-pipeline/**` | Dogfood pipeline app |
| `docs/STDLIB_GRADUATION.md` | Graduation note (final task) |

---

### Task 1: Scaffold `wj-sync` + channel roundtrip (TDD)

**Files:**
- Create: `packages/wj-sync/wj.toml`
- Create: `packages/wj-sync/.gitignore`
- Create: `packages/wj-sync/src/lib.wj`
- Create: `packages/wj-sync/src/channel.wj`
- Create: `packages/wj-sync/tests/channel_test.wj`

**Interfaces:**
- Produces: `unbounded_int() -> (IntSender, IntReceiver)`, `send_int`, `recv_int` (start with `int` specialization if generics fail; promote to generic when green)

- [ ] **Step 1: Write failing channel test**

```windjammer
use crate::recv_int
use crate::send_int
use crate::unbounded_int

@test
fn test_unbounded_send_recv() {
    let pair = unbounded_int()
    send_int(pair.0, 42)
    assert_eq(recv_int(pair.1), 42)
}
```

- [ ] **Step 2: Run test — expect fail (missing symbols)**

```bash
cd packages/wj-sync && unset CARGO_TARGET_DIR && \
  /Users/jeffreyfriedman/src/wj/windjammer/target/release/wj test
```

Expected: compile/link failure or missing `unbounded_int`

- [ ] **Step 3: Minimal channel implementation**

Implement `unbounded_int` / `send_int` / `recv_int` in idiomatic WJ wrapping available sync channel primitives (no public Arc/Mutex names). Prefer generic `Channel` if compiler accepts; else int-specialized first slice then genericize in Task 1b.

- [ ] **Step 4: Tests green**

- [ ] **Step 5: Add `try_recv` empty + after-close tests; implement until green**

---

### Task 2: Shared counter (TDD)

**Files:**
- Create: `packages/wj-sync/src/shared.wj`
- Create: `packages/wj-sync/tests/shared_test.wj`
- Modify: `packages/wj-sync/src/lib.wj`

**Interfaces:**
- Produces: `SharedInt`, `shared_int(n)`, `shared_int_add`, `shared_int_get`

- [ ] **Step 1: Failing tests for create/add/get**
- [ ] **Step 2: Confirm red**
- [ ] **Step 3: Implement `Shared`/`Counter` for int without exposing Mutex in API**
- [ ] **Step 4: Green; add multi-add stress test (same-thread first, then parallel when Task 4 exists)**

---

### Task 3: Coordination — Once, Latch, Barrier (TDD)

**Files:**
- Create: `packages/wj-sync/src/coord.wj`
- Create: `packages/wj-sync/tests/coord_test.wj`

- [ ] **Step 1: Failing tests — Once runs body once; Latch counts down to open; Barrier of 1 trips**
- [ ] **Step 2: Red → implement → green**
- [ ] **Step 3: Multi-party Barrier test (same-thread simulation OK if threads blocked; real multi-task in Task 4)**

---

### Task 4: Pending — `task` / `parallel` / `.wait()` (TDD)

**Files:**
- Create: `packages/wj-sync/src/pending.wj`
- Create: `packages/wj-sync/tests/pending_test.wj`

**Interfaces:**
- Produces: `PendingInt`, `parallel_int(fn-or-value-pattern)`, `wait_int`

Note: If closures are unavailable, use named functions + int job ids (same pattern as `wj-event` queue model) while keeping public names `task`/`parallel`/`Pending`/`wait`.

- [ ] **Step 1: Failing test — `parallel` compute then `wait` returns value**
- [ ] **Step 2: Red → implement via language/OS worker without public `spawn`/`thread` names**
- [ ] **Step 3: Shared counter incremented from two `parallel` works — ownership dogfood**
- [ ] **Step 4: If compiler fails idiomatic shape → stop, file windjammer repro, fix, resume**

---

### Task 5: Pool (TDD)

**Files:**
- Create: `packages/wj-sync/src/pool.wj`
- Create: `packages/wj-sync/tests/pool_test.wj`

- [ ] **Step 1: Failing test — pool of N workers processes M int jobs, all results received**
- [ ] **Step 2: Implement Pool with Channel + parallel workers + shutdown**
- [ ] **Step 3: Green; add shutdown-rejects-new-jobs test**

---

### Task 6: Benchmarks

**Files:**
- Create: `packages/wj-sync/benches/channel_bench.wj` (or `tests/channel_bench.wj` if needed)
- Create: `packages/wj-sync/benches/shared_bench.wj`
- Create: `packages/wj-sync/benches/pool_bench.wj`
- Create: `packages/wj-sync/benches/rust_baseline/Cargo.toml` + `src/main.rs` (std mpsc / Arc Mutex / threads)
- Modify: `packages/wj-sync/README.md` with numbers

- [ ] **Step 1: WJ wall-clock benches (msgs/sec, incs/sec, jobs/sec)**
- [ ] **Step 2: Rust baseline microbenches**
- [ ] **Step 3: Record ratios; warn if >2×, soft-fail note if >5×**
- [ ] **Step 4: Document in README**

---

### Task 7: `wj-pipeline` dogfood app

**Files:**
- Create: `apps/wj-pipeline/wj.toml`, `.gitignore`, `README.md`
- Create: `apps/wj-pipeline/src/main.wj`, `src/domain/*.wj`, `src/adapters/*.wj`
- Create: `apps/wj-pipeline/tests/*_test.wj`
- Modify: `apps/wj-pipeline/wj.toml` dependency on `wj-sync` build path

- [ ] **Step 1: Domain test — stage transform pure function**
- [ ] **Step 2: Wire Channel + Pool pipeline; CLI prints count + elapsed ms**
- [ ] **Step 3: Integration test with small N jobs**
- [ ] **Step 4: `wj build` / `wj test` green for app**

---

### Task 8: Docs + graduation note

**Files:**
- Modify: `docs/STDLIB_GRADUATION.md`
- Modify: `docs/STDLIB_COVERAGE.md` if sync modules used
- Modify: `docs/ROADMAP.md` / `PROGRESS.md` briefly
- Modify: `packages/wj-sync/README.md`

- [ ] **Step 1: Add wj-sync row — candidate for `std::sync` / channels**
- [ ] **Step 2: Note terminology feedback for WJ-CONC-01**
- [ ] **Step 3: Final full `wj test` on package + app**

---

## Self-review

1. **Spec coverage:** Channels, Shared, coord, Pending, Pool, pipeline app, benches, graduation — all have tasks.  
2. **Placeholders:** None intentional; int-specialization allowed only as temporary if generics blocked, then genericize before Task 7.  
3. **Types:** `Pending` + `wait`; Channel Sender/Receiver; Shared — consistent across tasks.
