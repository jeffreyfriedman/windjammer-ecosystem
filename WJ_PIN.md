# Windjammer compiler pin

The ecosystem apps and packages are built and tested in CI with the Windjammer compiler (`wj`) at the
commit below. CI never uses a floating branch.

```
WJ_REPO=https://github.com/jeffreyfriedman/windjammer.git
WJ_COMMIT=923b4c679a747ec93fe64ccefc1467cd8960c63f
```

Status: CANDIDATE. This commit has not been verified to pass `wj build` + `wj test` across all apps and
packages here. It must exist on `WJ_REPO` before CI can fetch it: push it to the compiler remote first,
or the checkout step fails.

## Pin policy

- `WJ_COMMIT` is a full 40-char SHA, never a branch or `tip`.
- `.github/workflows/ci.yml` (`WINDJAMMER_REF`) must equal `WJ_COMMIT`; the `pin-check` job fails if not.
- Bump it only in a commit that shows the ecosystem CI loop green with the new compiler. One bump per commit.
- Same format as the platform repo's `WJ_PIN.md` (`grep '^WJ_COMMIT='` parses it).
