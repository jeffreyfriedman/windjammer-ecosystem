# wj-notes-api

In-memory notes CRUD — domain-first seed for Wave 1 REST API.

No Cargo crates, no `extern fn`, no `ffi/`. HTTP adapter comes next.

## Layout

```
src/
  domain/store.wj   # Note + NoteStore (HashMap-backed)
  adapters/         # HTTP (planned)
  main.wj           # composition root (stub)
tests/
  store_test.wj     # CRUD domain tests
```

## Build / test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-notes-api
$WJ test
```

## License

MIT OR Apache-2.0
