# wj-ini

Read and write INI configuration files in pure Windjammer: `[section]` headers, `key = value` and `key: value`
pairs, `;` and `#` comments, quoted values with escapes, typed getters, and a writer whose output parses back to the
same document. Errors name the line they came from.

```windjammer
use wj_ini::get_bool
use wj_ini::get_int
use wj_ini::get_or
use wj_ini::parse
use wj_ini::set
use wj_ini::write

fn main() -> Result<(), string> {
    let text = "name = demo\n[server]\nport = 8080 ; default\nverbose = yes\ntitle = \"My App ; v2\"\n"
    let ini = parse(text)?

    println("port: ${get_int(ini, "server", "port")?}")
    println("verbose: ${get_bool(ini, "server", "verbose")?}")
    println("title: ${get_or(ini, "server", "title", "?")}")
    println("missing: ${get_or(ini, "server", "host", "localhost")}")

    let updated = set(ini, "server", "port", "9090")
    print(write(updated))
    Ok(())
}
```

Output:

```text
port: 8080
verbose: true
title: My App ; v2
missing: localhost
name = demo

[server]
port = 9090
verbose = yes
title = "My App ; v2"
```

## How it behaves

| Call | Effect |
|---|---|
| `parse(text)` | `Ok(Ini)` or `Err("line N: ...")` |
| `get`, `get_or`, `get_all` | lookups; a repeated key resolves to its last value, `get_all` keeps every one |
| `get_int`, `get_bool` | typed values with errors that name the section and key; integers go beyond 32 bits |
| `has`, `has_section`, `keys`, `section_names` | inspection |
| `set`, `remove` | return an updated copy |
| `write(ini)`, `quote(value)` | serialize; values are quoted only when they need it |

Keys before any header live in the section named `""`. A `;` or `#` starts an inline comment only after whitespace,
so `url = http://h/#frag` keeps its fragment. Booleans accept `true/false`, `yes/no`, `on/off`, `1/0`.

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/load_config.wj` line for line and that the
`Output` block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/ini_test.wj
tests/docs_test.wj
examples/load_config.wj
```

## License

MIT OR Apache-2.0
