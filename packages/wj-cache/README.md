# wj-cache

An in-memory LRU cache with per-entry TTL, and the Redis wire protocol as a codec. Time is an argument, so expiry is deterministic and testable; the cache never reads a clock.

```windjammer
use wj_cache::lru::new_cache

let mut cache = new_cache(1000, 60000)      // 1000 entries, 60 s default TTL
cache.put("user:1", "ada", now_ms)
match cache.get("user:1", now_ms) {
    Some(v) => println(v),
    None => println("miss"),
}
```

## API

- `lru`: `new_cache`, `put`, `put_ttl`, `get`, `peek`, `contains`, `delete`, `purge_expired`, `incr`, `len`, `clear`, `keys_lru`, `stats`
- `resp`: `encode_command(args)`, `parse_reply(bytes)` (incremental: `Ok(None)` means more bytes), `show`, and `cmd_set`, `cmd_get`, `cmd_del`, `cmd_incr`, `cmd_set_ex`, `cmd_auth`

When the cache is full an expired entry is dropped before a live one is evicted. Behaviour is checked against a Python `OrderedDict` model over 400 random operations; the protocol against the vectors in the Redis protocol specification.

`std::net` has no raw TCP yet, so there is no connected Redis client: send the bytes from `encode_command` over your own socket and feed what comes back to `parse_reply`.

## Build / test

```bash
cd packages/wj-cache
wj test --interpret
wj test
```
