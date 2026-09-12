# wj-glob

Shell-style path glob matching (`*`, `?`, `**`) in pure Windjammer.

- `*` / `?` match within a single path segment (do not cross `/`)
- `**` matches zero or more path segments

## API

```windjammer
use wj_glob

assert(is_match("*.md", "readme.md"))
assert(is_match("src/*", "src/main.wj"))
assert(is_match("src/**/*.wj", "src/deep/main.wj"))
assert(is_match("**/main.wj", "a/b/main.wj"))

let mut paths = Vec::new()
paths.push("a.md")
paths.push("b.txt")
let hits = filter("*.md", paths)
```

| Function | Description |
|---|---|
| `is_match(pattern, path)` | Segment-aware glob match (`*`, `?`, `**`) |
| `filter(pattern, paths)` | Matching subset |

## Layout

```
src/lib.wj
tests/glob_test.wj
```

## License

MIT OR Apache-2.0
