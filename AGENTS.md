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

## TDD workflow

1. Write a failing `tests/<name>_test.wj`.
2. Run `unset CARGO_TARGET_DIR && $WJ test` and confirm failure.
3. Implement the minimum idiomatic `.wj` to pass.
4. Re-run `$WJ test` until green.
5. On a compiler bug: failing repro in `windjammer/tests/` → fix → resume.

## CI and `wj` pinning

CI checks out a pinned `windjammer` commit or release, builds that `wj`, then builds and tests every app and package. Ban-list checks reject `extern fn` and `ffi/` under `apps/` and `packages/`.

## Architecture per app

```
apps/<name>/src/
  domain/       # business rules
  adapters/     # CLI, HTTP, DB, FS
  main.wj       # composition root
```

## Ban-list check

```bash
rg -n 'extern fn' apps packages --glob '*.wj'
find apps packages -type d -name ffi
```

Both should be empty for application sources.
