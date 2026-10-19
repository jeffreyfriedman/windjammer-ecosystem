# Windjammer compiler pin

The ecosystem apps and packages are built and tested in CI with the Windjammer compiler (`wj`) at the
commit below. CI never uses a floating branch.

```
WJ_REPO=https://github.com/jeffreyfriedman/windjammer.git
WJ_COMMIT=034e0b4e0a28c7d0ae860a955f399552671f625b
```

Status: CANDIDATE. Sweep at this commit (compiled `wj build --release src` + `wj test`, per package and app):
all but five packages and nine apps are green; the rest fail on compiler issues tracked in the compiler repo
(`docs/known-issues/`). Bump to VERIFIED only when the whole loop is green. The commit must exist on `WJ_REPO`
before CI can fetch it.

## Pin policy

- `WJ_COMMIT` is a full 40-char SHA, never a branch or `tip`.
- `.github/workflows/ci.yml` (`WINDJAMMER_REF`) must equal `WJ_COMMIT`; the `pin-check` job fails if not.
- Bump it only in a commit that shows the ecosystem CI loop green with the new compiler. One bump per commit.
- Same format as the platform repo's `WJ_PIN.md` (`grep '^WJ_COMMIT='` parses it).
