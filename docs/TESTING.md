# Testing

## How tests are discovered

- Every `apps/*` and `packages/*` directory with a `wj.toml` is a package. CI runs `wj build` then
  `wj test` inside it (`.github/workflows/ci.yml`, job `build-apps`). `wj test` finds `tests/**/*.wj`
  and `*_test.wj` / `test_*.wj` files; test functions are `fn test_*` / `@test`.
- A package with no test function would pass silently, so `scripts/assert-package-tests.sh` fails CI when a
  package has none (uses `wj test --list` when the compiler supports it, else greps).
- `scripts/check-orphan-tests.sh` (job `orphan-tests`, no builds) fails when a test file sits in a
  directory with no `wj.toml` (the loop skips it) or outside `apps/` and `packages/`
  (roots in `scripts/wj-test-roots.txt`). Exceptions: `scripts/orphan-allowlist.txt` (`path | reason`).

## Adding a test

1. Put `foo_test.wj` (or `tests/foo.wj`) in the package with at least one `fn test_...()`.
2. New package: create `apps/<name>/` or `packages/<name>/` with `wj.toml`; the CI loop picks it up.
3. Run locally: `cd packages/<name> && wj build src && wj test`.

## Compiler pin

`WJ_PIN.md` holds the full SHA; `ci.yml` `WINDJAMMER_REF` must match (job `pin-check`). The SHA must exist
on the compiler remote or the checkout fails. Bump pin and ref together.

## Unverified

CI has not been run. `wj test` exits 0 on zero tests in the current compiler (a fix is pending there), which
is why the assertion script exists; it stays useful afterward.
