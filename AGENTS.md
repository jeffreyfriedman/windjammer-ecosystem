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
