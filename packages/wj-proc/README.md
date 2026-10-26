# wj-proc

Run programs and git from Windjammer scripts. Capture status, stdout and stderr; run a command line through `sh` in a directory; measure a child's CPU time; read the load average; list tracked files.

## API

```windjammer
use wj_proc

let out = capture("git", ["status", "--short"])?
println(out.stdout)

let t = timed("sh", ["-c", "sleep 1"])?
println("cpu ${t.user + t.sys}s")
```

- `capture(program, args)` - `Output { status, stdout, stderr }`; a non-zero exit is data, not an error
- `sh(command)`, `sh_in(dir, command)`, `quote(text)` - shell command lines with safe quoting
- `timed(program, args)` - `Timed { output, user, sys }` from `/usr/bin/time -p` (10 ms resolution)
- `load_avg()`, `parse_load(text)` - the 1-minute load average
- `git_toplevel(dir)`, `git_ls_files(dir, pathspecs)`, `git_changed(dir)`

Scripts that use it declare the programs they run, for example `process = ["git", "sh"]` under `[capabilities]` in `wj.toml`.

## Layout

```
src/lib.wj
tests/proc_test.wj
```

## Build / test

```bash
cd packages/wj-proc
wj test --interpret
```

## License

MIT OR Apache-2.0
