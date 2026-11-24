# wj-grep

Recursive text search and replace in the spirit of `ripgrep`, built on ecosystem packages:

| Concern | Package |
|---|---|
| regex matching, escaping, replace | `wj-regex` |
| `.gitignore` rules | `wj-gitignore` |
| directory walk | `wj-fs-walk` |
| argv flags | `wj-cli-args` |

Domain (`src/domain/`) is pure: option handling, line selection, context rendering, replacement, file selection.
The adapters (`src/adapters/`) parse argv and touch the file system. Default tests need no network or files.

## Usage

```text
wj-grep [--ignore-case] [--word] [--fixed] [--invert] [--count] [--files-with-matches]
        [--no-line-number] [--no-ignore] [--hidden] [--context=N] [--before=N] [--after=N]
        [--max-count=N] [--replace=TEXT [--write]] PATTERN [PATH...]
```

- Output is `path:LINE:text`; context lines use `-` (`path-LINE-text`) and groups are separated by `--`.
- Dot-files and dot-directories are skipped unless `--hidden`; the root `.gitignore` of each searched directory applies unless `--no-ignore`.
- `--replace=TEXT` previews (`path: N replacement(s)`); add `--write` to rewrite files. `$1`-style captures work.
- Exit status: 0 when something matched, 1 when nothing did, 2 on a usage or I/O error.
- Flags take the `--name=value` form (short flags are not supported); nested `.gitignore` files are not read.

## Build and test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj
for p in wj-regex wj-gitignore wj-fs-walk wj-cli-args wj-glob; do (cd packages/$p && $WJ build --release src); done
cd apps/wj-grep
$WJ test
$WJ build --release src
```
