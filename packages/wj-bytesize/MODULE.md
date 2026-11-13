# wj-bytesize

## Purpose
Format and parse byte sizes (IEC and SI units) and transfer rates with exact integer arithmetic.

## Inputs / outputs
- In: byte counts, or size text such as `1.5 MiB`.
- Out: display text, or a byte count / readable error.

## Layout
- `src/lib.wj`, `tests/bytesize_test.wj`, `tests/docs_test.wj`, `examples/upload_limits.wj`.

## Invariants
- No floats: formatting and parsing agree on every platform.
- `parse(format_places(x, base, 3))` is within display precision of `x`.
- Overflow is an `Err`, never a wrapped value.
