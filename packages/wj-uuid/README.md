# wj-uuid

RFC 4122 UUID helpers: **v1** (time-based), **v4** (random), and **v5** (name-based SHA-1).

## API

```windjammer
use wj_uuid

// Random v4
let id = v4()

// Time-based v1 (current UTC clock)
match v1() {
    Ok(id) => println(id),
    Err(e) => println(e),
}

// Deterministic v1 (tests / replay)
match v1_from_timestamp(1700000000000, 984, "123456789abc") {
    Ok(id) => println(id),
    Err(e) => println(e),
}

// Name-based v5
match v5_dns("www.example.com") {
    Ok(id) => println(id),
    Err(e) => println(e),
}

println(version("2ed6657d-e927-568b-95e1-2665a8aea6a2"))  // 5
assert(is_valid(id))
```

### Functions

| Function | Description |
|---|---|
| `v4()` | Random UUID v4 |
| `v1()` | UUID v1 from current UTC time + random clock/node |
| `v1_from_timestamp(ms, clock_seq, node_hex)` | Deterministic v1 |
| `v5(namespace, name)` | UUID v5 from namespace UUID + UTF-8 name |
| `v5_dns(name)` / `v5_url(name)` | v5 with RFC standard namespaces |
| `version(text)` | `Some(1\|4\|5)` when valid |
| `is_valid(text)` | Canonical v1 / v4 / v5 with RFC 4122 variant |
| `normalize(text)` | Lowercase hex |

### Standard namespaces

- `NAMESPACE_DNS`, `NAMESPACE_URL`, `NAMESPACE_OID`, `NAMESPACE_X500`

## Layout

```
src/lib.wj
tests/uuid_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd packages/wj-uuid
$WJ test
```

## Compiler dependencies (repros in `windjammer/tests/`)

| Blocker | Repro | Needed for |
|---|---|---|
| `std::random.range` → `random::int_range` | `bug_std_random_range_codegen_test.rs` | `v4()`, `v1()` node/clock |
| `std::crypto.sha1_bytes` | `bug_std_crypto_sha1_bytes_test.rs` | `v5()` |
| `time.utc_now().timestamp_millis()` | `bug_std_time_timestamp_millis_test.rs` | `v1()` |

## License

MIT OR Apache-2.0
