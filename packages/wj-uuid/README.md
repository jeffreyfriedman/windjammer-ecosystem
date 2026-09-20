# wj-uuid

RFC 4122 / RFC 9562 UUID helpers: **v1** (time-based), **v4** (random; thin-wraps `std::uuid.v4`), **v5** (name-based SHA-1), **v7** (Unix-ms time-ordered; thin-wraps `std::uuid.v7` / `v7_from_timestamp`), plus **nil** / **max** constants.

## API

```windjammer
use wj_uuid

let id = v4()
match v1() {
    Ok(id) => println(id),
    Err(e) => println(e),
}
match v5_dns("www.example.com") {
    Ok(id) => println(id),
    Err(e) => println(e),
}
match v7() {
    Ok(id) => println(id),
    Err(e) => println(e),
}
assert(is_valid(id))
assert(is_nil("00000000-0000-0000-0000-000000000000"))
assert(is_max("ffffffff-ffff-ffff-ffff-ffffffffffff"))
```

Deterministic builders for tests:

- `v1_from_timestamp(unix_millis, clock_seq, node_hex)`
- `v7_from_timestamp(unix_millis, random_hex)` — `random_hex` is 20 hex digits (10 bytes)

Constants: `NIL`, `MAX`, plus RFC namespaces (`NAMESPACE_DNS`, …).

## Layout

```
src/lib.wj
tests/uuid_test.wj
```

## Tip status

❌ tip RED (P3.298) — `append_bytes(mut out: Vec<u8>, …) -> Vec<u8>` demotes to `&Vec<u8>`. Idiomatic sources kept; gate in `windjammer/tests/`.

## License

MIT OR Apache-2.0
