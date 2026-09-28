# Beta success path — first hour

Goal: a new developer compiles something useful in **under an hour** using tip-green `std::*` only (no Cargo crates, no `extern fn`).

## 0. Install local `wj`

```bash
cd /path/to/windjammer
unset CARGO_TARGET_DIR
cargo build --release
# or: cargo install --path . --force
export WJ="$(pwd)/target/release/wj"   # or tip binary
$WJ --version   # expect 0.50.0+
```

## 1. Smoke the seed binary (2 minutes)

```bash
cd windjammer-ecosystem/apps/wj-hello
unset CARGO_TARGET_DIR
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Expect: `wj-hello 0.1.0` and exit 0.

## 2. First-hour std card (15 minutes)

```bash
cd windjammer-ecosystem/apps/wj-first-hour
unset CARGO_TARGET_DIR
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Prints a JSON-ish session card built only from tip-green std:

| Concern | Module |
|---|---|
| Identity | `std::uuid.v4` |
| Clock | `std::time.utc_now` + `to_rfc3339` |
| Paths | `std::path.join` |
| Config | `std::config.parse_flat` (toml text) |
| URL component | `std::encoding.url_encode` |

## 2b. Multipart form parse (10 minutes)

```bash
cd packages/wj-multipart && $WJ build --release src && cd -
cd windjammer-ecosystem/apps/wj-form-parse
unset CARGO_TARGET_DIR
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Expect lines like `title=Hello` / `body=World`. Dogfoods `wj-multipart` without the notes-api dep graph.

## 3. Week-one find (walk + glob)

```bash
cd packages/wj-fs-walk && $WJ build --release src && cd -
cd packages/wj-glob && $WJ build --release src && cd -
cd windjammer-ecosystem/apps/wj-find
unset CARGO_TARGET_DIR
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Expect a `*.wj` path (demo walks `../wj-hello/src`). Uses package `wj-glob` until tip `path.glob_match` greens.

## 4. Week-one CLI (copy path)

`apps/wj-todo-cli` — file CRUD + `std::json` export. Dogfoods validation via `wj-validate` (ecosystem schema DSL — fine for products).

```bash
cd packages/wj-validate && $WJ build --release src && cd -
cd apps/wj-todo-cli
$WJ test
$WJ build --release src
```

## 5. Week-one HTTP (copy path)

| App | What you learn |
|---|---|
| `wj-fetch` | `std::http` client GET |
| `wj-notes-api` | small REST surface |
| `wj-auth-api` | JWT + bcrypt via std wrappers |

## Tip-owned holes (do not work around in apps)

File / wait on tip RED gates — see handoffs:

| Hole | Gate / handoff |
|---|---|
| Config merge/resolve ownership | `STDLIB_CONFIG_HANDOFF.md` |
| Form-urlencoded std | `STDLIB_FORM_HANDOFF.md` → `encoding.form_*` |
| Path glob match | same → `path.glob_match` |

Until form greens: use `wj-querystring` (pure WJ). Until glob greens: use `wj-glob`.
Until `std::url` greens: use `wj-url` (query helpers form-decode today).

## Definition of “comfortable beta”

- [x] Hello + first-hour card green on pinned tip `wj`
- [x] `wj-form-parse` multipart dogfood green (3 tests)
- [x] `wj-multipart` tip green (split_once + owned `parse_multipart` formals)
- [x] `wj-find` walk+glob dogfood green (4 tests); `wj-glob` `$WJ test` 14/14 on tip p3515 after P3.516 (`is_match` thin-wraps `path.glob_match`)
- [x] Todo CLI green without rust-interop (60 tests)
- [x] One HTTP app green without rust-interop (`wj-fetch` 31 tests; `wj-notes-api` P3.520 isolate GREEN — product `handle_method(self)` P3.522, `&mut query` P3.518, adapter `find_char(&String)` P3.524)
- [x] Form + glob + config.resolve tip GREEN (`form_parse`/`form_stringify`, `path.glob_match`, `url.parse`/`join`, `config.resolve` cargo-check on tip p3515)
- [ ] `docs/STDLIB_GRADUATION.md` P0 rows all ✅ (DB apply path P3.527/P3.528; querystring thin-wrap blocked on P3.526)

Cross-link: `STDLIB_GRADUATION.md`, `STDLIB_COVERAGE.md`, tip `tests/STDLIB_ADOPTION_QUEUE.md`.
