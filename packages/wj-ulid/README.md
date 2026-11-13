# wj-ulid

ULIDs for Windjammer: 128-bit identifiers that sort by creation time, written as 26 Crockford base32 characters
(48 bits of millisecond timestamp, 80 bits of randomness). They make good database keys because new rows land at the
end of the index, and they read well in logs and URLs.

The core functions take the clock and the random bytes as arguments, so tests and replays are deterministic.
`generate()` supplies both from the runtime when you just want an ID.

```windjammer
use wj_ulid::encode
use wj_ulid::generate
use wj_ulid::is_valid
use wj_ulid::monotonic
use wj_ulid::timestamp_ms

fn main() {
    // deterministic: the clock and the random bytes are arguments
    let first = encode(1469922850259, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]).unwrap()
    let second = monotonic(first, 1469922850259, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap()
    println(first)
    println(second)
    println("same millisecond, still ordered: ${first < second}")
    println("created at ${timestamp_ms(first).unwrap()} ms")

    // convenient: system clock and random source
    println("generated id is valid: ${is_valid(generate())}")
}
```

Output:

```text
01ARZ3NDEK18A1WA1J7H350PK4
01ARZ3NDEK18A1WA1J7H350PK5
same millisecond, still ordered: true
created at 1469922850259 ms
generated id is valid: true
```

## How it behaves

| Call | Effect |
|---|---|
| `encode(timestamp_ms, random_bytes)` | 26-character ULID; validates the range and exactly 10 bytes of 0..=255 |
| `decode(text)` | the 16 bytes, or an error naming the bad character or position |
| `is_valid`, `normalize` | validation (either letter case) and the canonical upper-case form |
| `timestamp_ms`, `random_bytes` | take an ID apart |
| `monotonic(previous, now_ms, fresh_random)` | within one millisecond the random part counts up, so IDs from one process always sort in creation order |
| `compare(a, b)` | -1, 0 or 1; the text order is the time order |
| `generate()` | an ID from the system clock and random source |

The first character is at most `7`, because 26 characters hold 130 bits and a ULID uses 128. A clock that moves
backwards never produces an ID that sorts before the previous one.

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/order_ids.wj` line for line and that the
`Output` block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/ulid_test.wj
tests/docs_test.wj
examples/order_ids.wj
```

## License

MIT OR Apache-2.0
