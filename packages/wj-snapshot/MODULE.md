# wj-snapshot

## Purpose
Golden-file assertions with a unified diff and an explicit update switch.

## Inputs / outputs
- In: a snapshot name and the actual text; `WJ_UPDATE_SNAPSHOTS` from the environment.
- Out: an `Outcome`, or a failed assertion carrying the diff.

## Layout
- `src/diff.wj` (line LCS and unified diff), `src/store.wj` (files, normalization, update mode); `tests/snapshot_test.wj` (difflib oracle, file flow in a scratch directory).

## Invariants
- A missing snapshot never passes and never gets written unless updating.
- Only `tests/snapshots/**` is touched by `assert_snapshot`; `check` writes where the caller points it.
