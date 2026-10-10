# Contributor guidelines

## Scope

This repository holds sample apps and reusable packages for typical Windjammer application development. Game engines, full database engines, and browser/GPU stacks live in other repositories.

## Rules

1. No project-local `ffi/` or hand-written `.rs` in `apps/` / `packages/` — only generated Rust and rust-interop when available.
2. Do not work around compiler bugs in application code. Reproduce with a failing test in `windjammer/tests/`, fix the compiler, then resume.
3. Compiler ownership/coercion fixes must use signatures / IR / solver — not hardcoded method-name lists.
4. Build with a local `wj`: `unset CARGO_TARGET_DIR &&` use `windjammer/target/release/wj` (or `cargo install --path . --force`).
5. Prefer TDD, DRY, SOLID, and hexagonal layout: domain in pure Windjammer; CLI/HTTP/DB/FS in adapters; wiring in `main.wj`.
6. Prefer Windjammer over Rust: at most five Cargo crates via rust-interop; no `extern fn` in this repo.
7. Tests live in `tests/*_test.wj` (or `src/*_test.wj`) using the Windjammer test framework. `wj test` does not discover `@test` only in `main.wj`.
8. Use standard library modules that the app genuinely needs. Track them in `docs/STDLIB_COVERAGE.md`.

## When the compiler disagrees with idiomatic code

**Stop. Do not reshape the app to please a broken compiler.**

Learned the hard way on `wj-dotenv`:

| Instinct (wrong) | Correct response |
|---|---|
| Nest `src/mod.wj` + `dotenv.wj` because `use crate::parse` failed from a flat `lib.wj` | Keep the flat `src/lib.wj` API; add a failing repro in `windjammer/tests/` |
| Switch `HashMap` import style / avoid `.get` because of `missing boundary signature for map::get` | Minimal package-shaped repro; fix signature/boundary codegen |
| Avoid `strings.join(vec, …)` because codegen emits `&[String]` vs `Vec` mismatch | Repro for owned-`Vec` → stdlib join; fix ownership inference / signature |
| Add `extern fn` / hand `.rs` “just for this package” | Never in this repo |

Idiomatic shapes that **must** work (if they fail, the bug is in `windjammer/`, not here):

- **Small package:** `src/lib.wj` with `pub fn` at the crate root; tests use `use crate::fn_name`
- **App with domain:** `src/domain/…` + `src/main.wj`; build with `$WJ build --release src` (multi-file CLI emits `[lib]` + `[[bin]]`)
- **Tests:** `tests/*_test.wj` only — not `@test` buried in `main.wj`

Module nesting (`mod.wj` trees) is for real domain boundaries, not for dodging import or ownership bugs.

## TDD workflow

1. Write a failing `tests/<name>_test.wj`.
2. Run `unset CARGO_TARGET_DIR && $WJ test` and confirm failure.
3. Implement the minimum **idiomatic** `.wj` to pass (the API you would publish).
4. Re-run `$WJ test` until green.
5. If step 3’s idiomatic code cannot compile: leave the package at that shape, write `windjammer/tests/<bug>_test.rs`, fix the compiler, then resume. Do not “make it compile” by uglier structure.

## CI and `wj` pinning

CI checks out a pinned `windjammer` commit or release, builds that `wj`, then builds and tests every app and package. Ban-list checks reject `extern fn` and `ffi/` under `apps/` and `packages/`.

## Architecture

**Apps** (hexagonal):

```
apps/<name>/src/
  domain/       # business rules
  adapters/     # CLI, HTTP, DB, FS
  main.wj       # composition root
```

**Packages** (keep flat until the domain needs modules):

```
packages/<name>/
  src/lib.wj
  tests/*_test.wj
  wj.toml
```

## Ban-list check

```bash
rg -n 'extern fn' apps packages --glob '*.wj'
find apps packages -type d -name ffi
```

Both should be empty for application sources.

## Docs and licensing

- Public docs: no private agent meta-narrative or internal “do not claim…” language.
- Dual license files match the compiler repo: `LICENSE-MIT` and `LICENSE-APACHE`.



## Dogfood rules (apply to every Windjammer project repo)

1. **Compiler-first.** If `wj` generates wrong or non-compiling Rust, file a minimal, generic repro in the compiler repo's `tests/` and fix it there. Never add shims (`+ ""`, spurious `.clone()`, `.as_str()`), never hand-edit generated Rust, never restructure idiomatic code to dodge a codegen bug.
2. **No compiler bug tracking here.** Status/"tip RED/GREEN" commits, repro queues and compiler incident docs live in the compiler repo only.
3. **Pin the compiler.** Record the known-good `wj` commit in `WJ_PIN.md`; bump deliberately, in its own commit.
4. **Generated output is never source.** Don't commit `build/`, `gen/` or transpiled `.rs` unless this repo's section below says otherwise; never edit them by hand.
5. **No status-report files.** No `*_COMPLETE.md`, `SESSION_SUMMARY*`, `*_STATUS.md` in the repo; record decisions in `docs/` ADRs and progress in commit messages.
6. **Real tests only.** Each feature needs a behavior test; no padding with trivial predicate/getter functions or placeholder `-> true` stubs.
7. **Races.** Other agents may be working: run `git status` before committing, stage only your own paths, never `git add -A` blindly, never force-push.
8. **Disk.** Use the shared target dir (`export CARGO_TARGET_DIR="$(wj cache path)"`); run `wj cache prune` when free disk is low; tests must write only to temp dirs.
9. **Claims need evidence.** Don't write "working/complete" without a command output or screenshot that proves it.

## Ecosystem rules
- A package earns its place by covering a real Pareto use case (HTTP, serde, DB, auth, logging, config...). Each addition needs a capability and an app-level test.
- **No padding**: no `require_*`/`status_is_*`/`is_*` predicate wrappers over existing getters; delete existing ones when touched.
- Dependencies between packages point at source (`path = "../wj-x"`), not at `build/` output.
- All packages stay on one released version series; bump with a CHANGELOG entry.
- Missing capability needing compiler/runtime work (typed serde, async HTTP, rust-interop) → file in the compiler repo, don't emulate it.
