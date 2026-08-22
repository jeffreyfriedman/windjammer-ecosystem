# wj-fs-walk

Recursive directory walk helpers over `std::fs`.

## API

```windjammer
use wj_fs_walk

match walk("/var/www") {
    Ok(paths) => {
        for path in paths {
            println(path)
        }
    },
    Err(e) => println(e),
}
```

- `walk(root)` — depth-first files and directories (excluding `root`)
- `walk_files(root)` — file paths only
- `walk_dirs(root)` — directory paths only (excluding `root`)
- `walk_files_with_suffix(root, suffix)` — files whose path ends with `suffix` (e.g. `.md`)
- `is_hidden_name(name)` — true when `name` starts with `.`

## Layout

```
src/lib.wj
tests/fs_walk_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-fs-walk
$WJ test
```

## License

MIT OR Apache-2.0
