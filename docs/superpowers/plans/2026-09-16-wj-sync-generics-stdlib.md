# wj-sync Generics + Stdlib Graduation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `wj-sync` a generics-first concurrency reference (`Channel<T>`, `Shared<T>`, `Pending<T>`, typed `Pool`), then queue/promote toward `std::sync` without Windjammer compiler source edits from this agent.

**Architecture:** Idiomatic Windjammer package under `packages/wj-sync`; dogfood via `apps/wj-pipeline`. Every tip RED stays as idiomatic `.wj` + failing `windjammer/tests/` repro + `COMPILER_REPRO_QUEUE.md`. No package reshapes to dodge bugs. No edits to `windjammer/src/`.

**Tech Stack:** Windjammer tip `wj` 0.50+, `std::sync::mpsc` / Arc+Mutex only under Shared internals, WJ test framework, `benches/rust_baseline` for ratios.

## Global Constraints

- Pure `.wj` in `apps/` / `packages/` — no `extern fn`, no `ffi/`, no hand `.rs`
- Public API: no `Arc`, `Mutex`, `spawn`, `async`, `Future`, `JoinHandle`
- Int-only APIs are migration aliases only — not “done”
- Sibling packages (toml/semver/timefmt/config) stay idiomatic; P3.325/326/329 remain compiler-owned
- Commits when user asks or at plan task boundaries if already authorized to proceed

---

### Task 1: Compiler gate — generic `send<T>(…, value: T)` must stay owned

**Files:**
- Create: `windjammer/tests/fixtures/generic_channel_send_owned.wj`
- Create: `windjammer/tests/bug_generic_channel_send_owned_param_must_not_demote_to_ref_test.rs`
- Modify: `windjammer/tests/COMPILER_REPRO_QUEUE.md` (P3.331)
- Modify: `windjammer/tests/run_red_repro_bundle.sh` (add filter)

**Interfaces:**
- Consumes: tip `wj` multipass `--library --module-file --no-cargo`
- Produces: RED gate proving `send` must emit `value: T` not `value: &T` / `.clone()`

- [ ] **Step 1: Write fixture**

```windjammer
use std::sync::mpsc

pub struct Sender<T> {
    tx: mpsc::Sender<T>,
}

pub fn send<T>(tx: Sender<T>, value: T) -> Sender<T> {
    match tx.tx.send(value) {
        Ok(_) => tx,
        Err(_) => tx,
    }
}
```

- [ ] **Step 2: Write failing Rust test** asserting generated `lib.rs` has `value: T` (not `value: &T`) and `tx.send(value)` (not `value.clone()`), and `cargo check` succeeds for a roundtrip helper.

- [ ] **Step 3: Run test — expect FAIL (RED)**

```bash
cd /Users/jeffreyfriedman/src/wj/windjammer
unset CARGO_TARGET_DIR
export CARGO_TARGET_DIR="$HOME/Library/Caches/windjammer/cargo-target/shared"
cargo test --release --test all -- generic_channel_send_owned_param_must_not_demote --nocapture
```

Expected: panic / assert fail showing `value: &T` in codegen.

- [ ] **Step 4: Queue entry P3.331 + RED_FILTERS** — do not fix compiler.

- [ ] **Step 5: Commit** (tests/queue only; never `windjammer/src/`)

---

### Task 2: Compiler gate — generic `recv` must not `rx.clone()` on non-Clone Receiver

**Files:**
- Create: `windjammer/tests/fixtures/generic_channel_recv_move.wj`
- Create: `windjammer/tests/bug_generic_channel_recv_must_move_receiver_not_clone_test.rs`
- Modify: `windjammer/tests/COMPILER_REPRO_QUEUE.md` (P3.332)
- Modify: `windjammer/tests/run_red_repro_bundle.sh`

**Interfaces:**
- Consumes: same multipass build
- Produces: RED gate — `recv` returns `(Receiver<T>, Option<T>)` by moving `rx`, never `rx.clone()`

- [ ] **Step 1: Fixture** with `Receiver<T>` + `recv` matching design
- [ ] **Step 2: Test** asserts no `rx.clone()` in generated recv body; cargo check green when fixed
- [ ] **Step 3: Run — expect RED**
- [ ] **Step 4: Queue P3.332**
- [ ] **Step 5: Commit tests only**

---

### Task 3: Idiomatic generic channel API in `wj-sync` (leave RED if tip blocks)

**Files:**
- Modify: `packages/wj-sync/src/channel.wj`
- Modify: `packages/wj-sync/src/lib.wj`
- Create: `packages/wj-sync/tests/channel_generic_test.wj`
- Keep int aliases temporarily: `unbounded_int` → thin wrappers calling generic with `int` **only if** tip cannot yet compile pure generic call sites; prefer pure generic tests first

**Interfaces:**
- Produces: `Sender<T>`, `Receiver<T>`, `unbounded`, `bounded`, `send`, `try_send`, `recv`, `try_recv`, `close`, `clone_sender`

- [ ] **Step 1: Failing package test**

```windjammer
use crate::unbounded
use crate::send
use crate::recv

@test
fn test_generic_int_roundtrip() {
    let pair = unbounded()
    let tx = send(pair.0, 7)
    let got = recv(pair.1)
    assert_eq(got.1, Some(7))
}
```

- [ ] **Step 2: `unset CARGO_TARGET_DIR && $WJ test`** — confirm fail
- [ ] **Step 3: Implement idiomatic generic `channel.wj`** (no workarounds)
- [ ] **Step 4: Re-run** — if tip RED, leave shape; ensure P3.331/332 cover the failure; keep existing int tests compiling via aliases **only if** they still typecheck without reshaping generics away
- [ ] **Step 5: Commit package**

---

### Task 4: Generic `Shared<T>` + `SharedMap<K,V>`

**Files:**
- Modify: `packages/wj-sync/src/shared.wj`
- Create: `packages/wj-sync/tests/shared_generic_test.wj`
- File new windjammer repros if tip demotes / Clone-forces generic Shared

**Interfaces:**
- Produces: `Shared<T>`, `shared_new`, `shared_get`, `shared_set` (or add), `SharedMap<K,V>` ops
- Counter stays int-specialized hot path OK

- [ ] **Step 1: Failing tests for Shared int + Shared string**
- [ ] **Step 2: Implement idiomatic Shared\<T\>**
- [ ] **Step 3: Repro if RED; queue next P3**
- [ ] **Step 4: Commit**

---

### Task 5: Generic `Pending<T>` + `task` / `parallel`

**Files:**
- Modify: `packages/wj-sync/src/pending.wj`
- Create: `packages/wj-sync/tests/pending_generic_test.wj`
- Repro gates as needed (closures / thread spawn generics)

**Interfaces:**
- Produces: `Pending<T>`, `parallel`, `task`, `wait`

- [ ] **Step 1: Failing `parallel` returning `Pending<int>` wait = 5**
- [ ] **Step 2: Implement; file repros if tip RED**
- [ ] **Step 3: Commit**

---

### Task 6: Typed Pool + real shutdown

**Files:**
- Modify: `packages/wj-sync/src/pool.wj`
- Modify: `packages/wj-sync/tests/pool_test.wj`
- Create: `packages/wj-sync/tests/pool_shutdown_test.wj`

**Interfaces:**
- Produces: `pool_new`, shared-inbox run over ints first then generic jobs when gates allow; `pool_shutdown` that rejects/drains (tested)

- [ ] **Step 1: Failing shutdown test**
- [ ] **Step 2: Implement idiomatic shutdown**
- [ ] **Step 3: Commit**

---

### Task 7: Update `wj-pipeline` + drop obsolete P3.290 comments

**Files:**
- Modify: `apps/wj-pipeline/src/**/*.wj` as needed for generic APIs
- Modify: `packages/wj-sync/src/lib.wj` (remove stale P3.290 note)
- Modify: `packages/wj-sync/src/pipeline.wj` if still needed as sugar

- [ ] **Step 1: App tests still green or idiomatic RED with filed gate**
- [ ] **Step 2: Commit**

---

### Task 8: Real benches + README ratios

**Files:**
- Create: `packages/wj-sync/tests/channel_bench_test.wj` (or `benches/` if discovered)
- Modify: `packages/wj-sync/benches/rust_baseline/src/main.rs` if needed
- Modify: `packages/wj-sync/README.md`

**Interfaces:**
- Produces: documented msgs/sec, incs/sec, jobs/sec vs Rust on same host

- [ ] **Step 1: WJ wall-clock benches (larger N than smoke)**
- [ ] **Step 2: Run rust_baseline release**
- [ ] **Step 3: Record ratios in README**
- [ ] **Step 4: Commit**

---

### Task 9: Stdlib adoption queue for `std::sync`

**Files:**
- Create: `windjammer/tests/bug_std_sync_channel_unbounded_wiring_test.rs` (and/or `.wj` fixture)
- Modify: `windjammer/tests/STDLIB_ADOPTION_QUEUE.md`
- Modify: `docs/STDLIB_GRADUATION.md` (ecosystem)

**Interfaces:**
- Produces: failing std adoption gate describing target `std::sync` / channel API for compiler agent

- [ ] **Step 1: Write failing std wiring test**
- [ ] **Step 2: Queue row — 🆕 RED**
- [ ] **Step 3: Commit tests + docs only**

---

## Self-review

1. **Spec coverage:** Generics channel/shared/pending/pool, benches, stdlib promotion, idiomatic siblings — Tasks 1–9.  
2. **Placeholders:** None — concrete paths and P3.331/332 assigned.  
3. **Types:** `Sender<T>` / `Receiver<T>` / `Pending<T>` / `Shared<T>` consistent across tasks.
