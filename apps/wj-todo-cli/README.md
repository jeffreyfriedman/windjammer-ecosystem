# wj-todo-cli

Local todo list CLI (hexagonal): domain store + command parsing + file adapter.

```
wj-todo-cli add buy milk
wj-todo-cli list
wj-todo-cli list --pending
wj-todo-cli list --done
wj-todo-cli list --json
wj-todo-cli list --done --json
wj-todo-cli done 1
wj-todo-cli edit 1 buy oat milk
wj-todo-cli rm 1
wj-todo-cli clear
wj-todo-cli clear --done
wj-todo-cli export
wj-todo-cli export --pending
wj-todo-cli export --out backup.json
wj-todo-cli import backup.json
wj-todo-cli import --merge backup.json
wj-todo-cli stats
wj-todo-cli stats --json
```

Persists to a tab-separated file (default `todos.txt` in the working directory).

## Packages

| Layer | Module |
|---|---|
| **wj-validate** | title nonempty + max length on `add` and `edit` |
| **std::json** | `list --json` output; `export` / `import` JSON array codec |
| **std::path** (local join) | `TODO_FILE` resolution until `wj-path` cross-crate forwarder repro is green |

## Layout

```
src/domain/     # TodoStore, parse_command, query, codec, config
src/adapters/   # file I/O, argv, run_args
src/main.wj
tests/
```

## Environment

| Variable | Default | Purpose |
|---|---|---|
| `TODO_FILE` | `todos.txt` | Storage file (absolute or relative to cwd) |
| `TODO_MAX_TITLE_LEN` | `200` | Max title length on `add` |

## Build / test

Path dependency must point at `packages/wj-validate/build`. Pre-build once:

```bash
unset CARGO_TARGET_DIR
export WJ=~/.cargo/bin/wj

cd packages/wj-validate && $WJ build src

cd apps/wj-todo-cli
$WJ test
$WJ build --release src
```

`wj test` — 60 unit tests (store, commands, query, codec, export, import, stats, edit, config, run).

## License

MIT OR Apache-2.0
