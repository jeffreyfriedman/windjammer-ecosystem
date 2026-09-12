# wj-inflect

Case and slug helpers in pure Windjammer (`snake_case`, `camel_case`, `pascal_case`, `slugify`).

## API

```windjammer
use wj_inflect

assert_eq(snake_case("HelloWorld"), "hello_world")
assert_eq(camel_case("hello_world"), "helloWorld")
assert_eq(pascal_case("hello_world"), "HelloWorld")
assert_eq(slugify("Hello World!"), "hello-world")
```

## Layout

```
src/lib.wj
tests/inflect_test.wj
```

## License

MIT OR Apache-2.0
