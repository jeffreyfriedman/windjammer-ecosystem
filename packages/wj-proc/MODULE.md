# wj-proc

## Purpose
Process and git helpers for scripts: capture a program's result, time its CPU use, read machine load, list tracked files.

## Inputs / outputs
- In: program name and arguments, or a shell command line.
- Out: `Output` / `Timed` values; `Err(String)` only when the program cannot be started.

## Layout
- `src/lib.wj`: `capture`, `sh`, `sh_in`, `quote`, `timed`, `load_avg`, git helpers.
- `tests/proc_test.wj`: quoting, status and stream capture, timing, load parsing, git.

## Invariants
- Every spawn goes through `std::process::run_with_output`, so the capability gate sees the program name.
- `quote` makes any text one safe `sh` word.
- `timed` removes the `time -p` report lines from the child's stderr.
