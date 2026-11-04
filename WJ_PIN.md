# Windjammer compiler pin

The ecosystem apps and packages are built and tested in CI with the Windjammer compiler (`wj`) at the
commit below. CI never uses a floating branch.

```
WJ_REPO=https://github.com/jeffreyfriedman/windjammer.git
WJ_COMMIT=b77ea8a6365cbc60cddc9eb1d641e02ddd9c0860
```

Status: CANDIDATE. Sweep at this commit (interpreted `wj test --interpret` and compiled `wj test`, per package and app,
2026-11-04): 64 of 66 targets are green in both modes. `wj-find` and `wj-grep` are red compiled on one compiler issue
(`docs/findings/README.md`, item 5; they passed at the previous pin). Bump to VERIFIED only when the whole loop is green.
The commit must exist on `WJ_REPO` before CI can fetch it.

## Pin policy

- `WJ_COMMIT` is a full 40-char SHA, never a branch or `tip`.
- `.github/workflows/ci.yml` (`WINDJAMMER_REF`) must equal `WJ_COMMIT`; the `pin-check` job fails if not.
- Bump it only in a commit that shows the ecosystem CI loop green with the new compiler. One bump per commit.
- Same format as the platform repo's `WJ_PIN.md` (`grep '^WJ_COMMIT='` parses it).
