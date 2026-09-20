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

## 3. Week-one CLI (copy path)

`apps/wj-todo-cli` — file CRUD + `std::json` export. Dogfoods validation via `wj-validate` (ecosystem schema DSL — fine for products).

```bash
cd apps/wj-todo-cli
$WJ test
$WJ build --release src
```

## 4. Week-one HTTP (copy path)

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
- [ ] Todo CLI + one HTTP app green without rust-interop
- [x] `wj-multipart` tip green (split_once + owned `parse_multipart` formals)
- [ ] Form + glob + config.resolve tip GREEN (or documented package path)
- [ ] `docs/STDLIB_GRADUATION.md` P0 rows all ✅
- [ ] Todo CLI + one HTTP app green without rust-interop
- [ ] Form + glob + config.resolve tip GREEN (or documented package path)
- [ ] `docs/STDLIB_GRADUATION.md` P0 rows all ✅

Cross-link: `STDLIB_GRADUATION.md`, `STDLIB_COVERAGE.md`, tip `tests/STDLIB_ADOPTION_QUEUE.md`.
