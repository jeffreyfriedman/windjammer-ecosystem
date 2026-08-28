# wj-multipart

`multipart/form-data` parse / format helpers in pure Windjammer (text-oriented parts).

## API

```windjammer
use wj_multipart

let boundary = boundary_from_content_type(content_type)?
let parts = parse_multipart(boundary, body)?
match get_field(parts, "title") {
    Some(v) => {},
    None => {},
}
match get_file(parts, "avatar") {
    Some(file) => {},
    None => {},
}
let wire = format_multipart(boundary, parts)
let header = content_type_header(boundary)
```

Binary uploads are treated as opaque strings (no streaming). Prefer `std::http` multipart support when it lands.

## Layout

```
src/lib.wj
tests/multipart_test.wj
```

## License

MIT OR Apache-2.0
