# wj-static-server

A read-only static file server core. `handle` turns a request into a response over the `std::http` types, with no socket, so it is tested without a network. Folders are mounted under URL prefixes; a request never reaches a file outside its mount.

- Mount table: N folders under URL prefixes (`/`, `/data`, ...); the longest matching prefix wins.
- Confined paths: rejects `..` segments (plain or percent-encoded), relative paths, backslashes and NUL bytes, then resolves the file with symlinks evaluated and re-checks it is still inside the mount. A symlink that leaves the folder is a 404.
- No directory listing. Only the root of a mount maps to its `index.html`.
- GET and HEAD serve files; any other method is a 405 with `Allow: GET, HEAD`; anything that does not resolve is a 404.
- A path ending in `/index.html` is answered with a 301 to `./`, so the canonical URL is the folder.
- Content-Type from an explicit extension table (`.js` is `text/javascript`); unknown extensions are sniffed as text or binary from the first 512 bytes.
- `Cache-Control` (default `no-store`), `X-Content-Type-Options: nosniff` and a `Content-Security-Policy` on every file response.
- `serve` binds 127.0.0.1 only; `serve_on` refuses any other host.

## API

```windjammer
pub struct Mount { prefix: string, folder: string }
pub struct Roots { mounts: Vec<Mount> }
pub struct StaticServer { roots: Roots, csp: string, cache_control: string }

pub fn mount(prefix: string, folder: string) -> Mount
StaticServer::new(mounts: Vec<Mount>) -> Result<StaticServer, string>
pub fn with_csp(server: StaticServer, csp: string) -> StaticServer
pub fn with_cache_control(server: StaticServer, value: string) -> StaticServer
pub fn handle(server: StaticServer, req: ServerRequest) -> ServerResponse
pub fn resolve(server: StaticServer, url_path: string) -> Option<string>
pub fn content_type(file: string, bytes: Vec<u8>) -> string
pub fn strict_local() -> string
pub fn strict_local_with_connect(origins: Vec<string>) -> string
pub fn serve(server: StaticServer, port: int) -> Result<(), string>
pub fn serve_on(server: StaticServer, host: string, port: int) -> Result<(), string>
```

- `StaticServer::new` resolves every folder to an absolute path. Errors: `no mounts`, `<folder> is not a folder`, `duplicate mount prefix <p>`, and a prefix that does not start with `/`, ends with `/` (other than `/` itself) or holds `.` or `..` segments.
- `strict_local()` is `default-src 'self'; img-src 'self' data:; media-src 'self'; object-src 'none'`. `strict_local_with_connect(["https://api.example.test"])` appends `; connect-src 'self' https://api.example.test`.
- A response for a file is `ServerResponse::binary`, so read the bytes from `binary_body`.
- Query strings are not part of the path `handle` sees; pass `req.path` without one.

## Example

Serve a site at `/` and a data folder at `/data`:

```windjammer
use std::cli
use wj_static_server::{StaticServer, mount, serve, with_csp, strict_local_with_connect}

fn main() {
    let server = match StaticServer::new([mount("/", "site"), mount("/data", "data")]) {
        Ok(s) => with_csp(s, strict_local_with_connect(["https://api.example.test"])),
        Err(e) => {
            eprintln("serve: ${e}")
            std::process::exit(1)
        },
    }
    println("serving http://127.0.0.1:8080/")
    match serve(server, 8080) {
        Ok(_) => {},
        Err(e) => eprintln("serve: ${e}"),
    }
}
```

A compiled project imports the package as `wj_static_server`.

## Tests

Handler-level tests (no socket) run against the checked-in tree `tests/fixtures/tree`: `site/` (mounted at `/`), `data/` (mounted at `/data`), `secret.txt` outside both, and symlinks in `site/` pointing at it.

```bash
wj test --interpret
```

`wj test` has no network grant yet, so a socket-level check is not part of this package's tests; `handle` is pure over the `std::http` types, which is what the tests exercise.

## Limits

- Whole files are read into memory; there is no range request, conditional request (ETag, 304) or compression.
- Not for the public internet: loopback only, no TLS, no listing, no write methods.

## Layout

```
src/lib.wj
tests/static_server_test.wj
tests/fixtures/tree/
```

## License

MIT OR Apache-2.0
