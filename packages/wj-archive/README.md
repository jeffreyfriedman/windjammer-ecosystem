# wj-archive

Tar and zip as byte vectors. Write an archive from a list of entries, or read one back, with no files, processes or temporary directories involved. Reading is safe by default: names that escape the extraction root are rejected, declared sizes are capped, and zip CRCs are verified.

```windjammer
use wj_archive::entry::file
use wj_archive::entry::dir
use wj_archive::zip
use wj_archive::bytes::utf8

let bytes = zip::write([dir("docs"), file("docs/a.txt", utf8("hi"))])?
let entries = zip::read(bytes)?
```

## API

- `entry`: `Entry`, `file`, `dir`, `is_safe_path`, `DEFAULT_MAX_TOTAL`
- `tar::write` (ustar, long names split into prefix), `tar::read` (ustar, GNU long names, PAX `path` and `size`)
- `zip::write` (stored), `zip::read` (stored and deflated, UTF-8 names, DOS timestamps)
- `inflate::inflate` (raw DEFLATE, written in Windjammer), `bytes::crc32`
- `read_limited(bytes, max_total)` on both formats

Output matches Python's `tarfile` and `zipfile` byte for byte; both readers read Python's ustar, PAX, GNU and deflated archives. Not supported: zip64, encryption, symlinks and devices in tar.

## Layout

```
src/{entry,bytes,inflate,tar,zip}.wj
tests/
```

## Build / test

```bash
cd packages/wj-archive
wj test --interpret
wj test
```
