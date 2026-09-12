# wj-template

Simple `{{key}}` string templates backed by `HashMap<string, string>`.

## API

```windjammer
use wj_template
use std::collections::HashMap

let mut vars = HashMap::new()
vars.insert("title", "Notes")
vars.insert("body", "Hello")

println(render("<h1>{{title}}</h1><p>{{body}}</p>", vars))
```

- `render(template, vars)` — substitute known keys; leave unknown `{{key}}` tokens unchanged
- `render_with_defaults(template, vars, defaults)` — fill from defaults; `vars` override
- `render_html(template, vars)` — like `render`, but HTML-escapes values
- `escape_html(text)` — escape `&`, `<`, `>`, `"` for HTML text
- `render_strict(template, vars)` — `Ok` rendered text or `Err` on missing keys
- `missing_keys(template, vars)` — keys referenced in the template but not in `vars`
- `placeholder_names(template)` — ordered list of every `{{key}}` in the template (names trimmed)

Placeholder names ignore surrounding whitespace: `{{ name }}` uses key `name`.

## Layout

```
src/lib.wj
tests/template_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-template
$WJ test
```

## License

MIT OR Apache-2.0
