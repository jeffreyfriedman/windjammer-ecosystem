# wj-path

POSIX-style path helpers. `join_path` and `basename` thin-wrap `std::path`; `normalize`, `dirname`, and `extname` stay package sugar (not yet in std).

Named `join_path` (not `join`) to avoid clashing with `strings.join` in codegen.

## API

```windjammer
use wj_path

assert_eq(join_path("/tmp", "a"), "/tmp/a")
assert_eq(normalize("/a/b/../c"), "/a/c")
assert_eq(basename("/tmp/foo.txt"), "foo.txt")
assert_eq(dirname("/tmp/foo.txt"), "/tmp")
assert_eq(extname("foo.txt"), ".txt")
```

## Layout

```
src/lib.wj
tests/path_test.wj
```

## License

MIT OR Apache-2.0
