# wj-cli-args

Argv helpers until clap rust-interop is the default for apps. Parse a `Vec<string>` (typically `env.args()`) without a schema: skip the program name, collect positionals, and read `--name` / `--name=value` flags.

## API

```windjammer
use wj_cli_args
use std::env

let args = env.args()
let url = require_first_positional(args, "usage: my-app <url>")?
let verbose = has_long_flag(args, "verbose")
```

- `read_argv()` — process argv including the program name
- `tail(args)` — tokens after `args[0]`
- `positionals(args)` — tokens that do not start with `--`
- `first_positional(args)` / `require_first_positional(args, usage)`
- `has_long_flag(args, name)` — true for `--name` or `--name=value`
- `long_flag_value(args, name)` — `--name=value` or the token after bare `--name`

Short flags (`-v`) and clap builders live in `std::cli` / future rust-interop; this package stays std-only.

## Layout

```
src/lib.wj                 # argv helpers
tests/cli_args_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-cli-args
$WJ test
```

## License

MIT OR Apache-2.0
