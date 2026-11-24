# wj-archive

## Purpose
Read and write tar and zip archives in memory.

## Inputs / outputs
- In: `Vec<Entry>` to write, `Vec<u8>` to read.
- Out: `Vec<u8>` or `Vec<Entry>`; every failure is `Err(message)`, never a panic.

## Layout
- `src/tar.wj`, `src/zip.wj`, `src/inflate.wj`, `src/bytes.wj`, `src/entry.wj`.
- `tests/oracle_test.wj` (Python tarfile, zipfile, zlib), `tests/safety_test.wj`.

## Invariants
- Pure: no I/O, no capabilities. Names are validated on write and on read.
- Total uncompressed size is capped (default 256 MiB) before any output is produced.
