# wj-json-doc

JSON documents that remember the order you wrote them in. Parse a file, change a member, print it back, and the keys stay where they were; print with Python-compatible layout (`json.dumps` with `indent` and `sort_keys`) so tools in either language produce identical files.

## API

```windjammer
use wj_json_doc

let doc = parse("{\"total\": 3, \"modules\": {}}")?
let doc = with(doc, "scope", from_str("cli"))
println(dumps(doc, 2, false))
```

- `parse(text)` - ordered parse; a repeated key keeps its first position and takes the last value
- `dumps(doc, indent, sort_keys)` - Python's `json.dumps` layout (`indent == 0` is the one-line form); non-ASCII is escaped as `\uXXXX`
- `dumps_unicode(doc, indent, sort_keys)` - the same with characters kept as they are
- `float_repr(x)` - Python's `repr(float)` (`100.0`, `1e+16`, `1.5e-05`)
- build: `object()`, `array()`, `member(k, v)`, `with(doc, key, value)`, `push(doc, value)`, `from_str`, `from_int`, `from_float`
- read: `get(doc, key)`, `at(doc, i)`, `keys(doc)`, `members(doc)`, `items(doc)`, `as_str`, `as_i64`, `as_f64`, `as_bool`, `is_null`

## Layout

```
src/lib.wj                  # Json enum, builders, parser, writer
tests/json_doc_test.wj      # expectations taken from Python's json module
```

## Build / test

```bash
cd packages/wj-json-doc
wj test --interpret
```

## License

MIT OR Apache-2.0
