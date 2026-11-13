# wj-bytesize

Human-readable byte sizes in pure Windjammer, with no floating point: write `1536` as `1.5 KiB` (or `1.5 kB` in SI
units), read `"2 GiB"` or `"1.5MB"` back into an exact byte count, and format transfer rates. Rounding is integer
arithmetic, so results are identical on every platform and in every build mode.

```windjammer
use wj_bytesize::format_iec
use wj_bytesize::format_rate
use wj_bytesize::format_si
use wj_bytesize::parse

fn main() {
    // a size limit from a config file or a CLI flag
    let limit = parse("1.5 MiB").unwrap()
    println("limit: ${limit} bytes")

    let upload = 1900000
    if upload > limit {
        println("rejected: ${format_iec(upload)} is over the ${format_iec(limit)} limit")
    }

    println("disk label: ${format_si(upload)}")
    println("speed: ${format_rate(upload, 250)}")
    println("bad input: ${parse("12 parsecs").unwrap_err()}")
}
```

Output:

```text
limit: 1572864 bytes
rejected: 1.8 MiB is over the 1.5 MiB limit
disk label: 1.9 MB
speed: 7.2 MiB/s
bad input: unknown unit 'parsecs'
```

## How it behaves

| Call | Effect |
|---|---|
| `format_iec(bytes)` | binary units (`KiB`, `MiB`, ...), one decimal, trailing zeros dropped |
| `format_si(bytes)` | decimal units (`kB`, `MB`, ...) |
| `format_places(bytes, base, places)` | 0 to 3 decimals; rounds half up; a value that rounds up to the next unit is shown there (`1048575` is `1 MiB`) |
| `format_rate(bytes, elapsed_ms)` | `7.2 MiB/s` |
| `parse(text)` | `"512"`, `"2 GiB"`, `"1.5MB"`, `"64k"`; exact for up to 3 decimals; errors on overflow and unknown units |
| `parse_or(text, fallback)` | `parse` with a default |

`KiB`/`MiB`/... and bare `K`/`M`/`G` are powers of 1024; `kB`/`MB`/`GB` are powers of 1000. A capital `M` with a
lowercase `b` (`Mb`) means bits, so it is rejected with a hint rather than guessed.

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/upload_limits.wj` line for line and that the
`Output` block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/bytesize_test.wj
tests/docs_test.wj
examples/upload_limits.wj
```

## License

MIT OR Apache-2.0
