# Windjammer compiler pin

The ecosystem apps and packages are built and tested in CI with the Windjammer compiler (`wj`) at the
commit below. CI never uses a floating branch.

```
WJ_REPO=https://github.com/jeffreyfriedman/windjammer.git
WJ_COMMIT=6ed79d644d3f4d03114b61f36540a8165782502d
```

Status: VERIFIED for the ecosystem loop. Sweep at this commit (interpreted `wj test --interpret` and compiled `wj test`, per
package and app, 2026-11-05): 70 of 70 targets are green in both modes (`docs/STDLIB_GRADUATION.md`). The commit must exist on
`WJ_REPO` before CI can fetch it.

## Pin policy

- `WJ_COMMIT` is a full 40-char SHA, never a branch or `tip`.
- `.github/workflows/ci.yml` (`WINDJAMMER_REF`) must equal `WJ_COMMIT`; the `pin-check` job fails if not.
- Bump it only in a commit that shows the ecosystem CI loop green with the new compiler. One bump per commit.
- Same format as the platform repo's `WJ_PIN.md` (`grep '^WJ_COMMIT='` parses it).
