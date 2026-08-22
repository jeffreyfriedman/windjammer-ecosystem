# wj-notes-api

In-memory notes CRUD over HTTP — Wave 1 REST seed.

No Cargo crates, no `extern fn`, no `ffi/`. Domain routing and JSON live in Windjammer; the HTTP adapter binds `std::http`.

## Routes

| Method | Path | Status |
|---|---|---|
| `GET` | `/notes` | 200 JSON array |
| `POST` | `/notes` | 201 created / 400 invalid JSON |
| `GET` | `/notes/:id` | 200 / 404 |
| `PUT` | `/notes/:id` | 200 / 400 / 404 |
| `DELETE` | `/notes/:id` | 204 / 404 |

Bind address is `0.0.0.0`. Port comes from `PORT` (default `8080`).

## Layout

```
src/
  domain/           # NoteStore + request routing / JSON (no sockets)
  adapters/http_server.wj  # std::http + Arc/Mutex
  main.wj           # composition root
tests/
  store_test.wj
  api_test.wj
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-notes-api
$WJ test
$WJ build --release src
```

## License

MIT OR Apache-2.0
