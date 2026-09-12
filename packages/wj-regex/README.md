# wj-regex

Idiomatic regex helpers over `std::regex` (pattern-string APIs).

## API

```windjammer
use wj_regex

match is_match("\\d+", "abc123") {
    Ok(ok) => {},
    Err(e) => println(e),
}

match find_all("\\w+", "a b c") {
    Ok(parts) => {},
    Err(e) => println(e),
}

let lit = escape("a+b?")
```

| Function | Description |
|---|---|
| `is_match(pattern, text)` | Match anywhere |
| `find(pattern, text)` | First match or None |
| `find_all(pattern, text)` | All matches |
| `replace` / `replace_all` | Substitution |
| `split(pattern, text)` | Split on matches |
| `escape(text)` | Literal-escape metacharacters |

## Layout

```
src/lib.wj
tests/regex_test.wj
```

## License

MIT OR Apache-2.0
