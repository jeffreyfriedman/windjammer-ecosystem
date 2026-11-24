# wj-snapshot

Snapshot tests: keep a program's output in a file next to the tests, fail with a readable unified diff when it changes, and accept the new output with one environment variable.

```windjammer
use wj_snapshot::store::assert_snapshot

@test
fn test_report_layout() {
    assert_snapshot("report", render_report())
}
```

The first run fails with `no snapshot ...; run with WJ_UPDATE_SNAPSHOTS=1 to create it`, so CI never writes files by itself. After `WJ_UPDATE_SNAPSHOTS=1 wj test` the file `tests/snapshots/report.snap` exists and later runs compare against it.

## API

- `store`: `assert_snapshot(name, actual)`, `check(dir, name, actual, update)` returning an `Outcome` (`matched`, `created`, `updated`, `changed`, `missing`), `normalize`, `redact`, `updating`
- `diff`: `unified(old, new, old_name, new_name, context)`, `lines_of`

Output is normalized before comparing (`\r\n` to `\n`, trailing blanks removed, one final newline). `redact` swaps volatile values (ids, timestamps, temp paths) for labels. The diff format matches Python's `difflib.unified_diff`.

Capabilities: `env.read WJ_UPDATE_SNAPSHOTS`, `fs.read` and `fs.write` under `./tests/snapshots/**`.

## Build / test

```bash
cd packages/wj-snapshot
wj test --interpret
wj test
```
