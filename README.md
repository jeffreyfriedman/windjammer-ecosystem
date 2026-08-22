# Windjammer Ecosystem

Apps and packages that show what everyday Windjammer development looks like: CLIs, small APIs, config libraries, and similar projects built with Windjammer’s standard library plus a few Cargo crates via rust-interop.

## Goals

- Idiomatic Windjammer for application logic
- At most a handful of Cargo dependencies (for example `clap`, `ureq`/`reqwest`, `serde_json`, one DB or file crate)
- No project-local `ffi/` or hand-written Rust in app/package sources
- Clear, tested examples others can copy

## Example projects

| Kind | Project | Status |
|---|---|---|
| Std-only CLI | [`apps/wj-hello`](apps/wj-hello) | Available |
| `.env` loader | [`packages/wj-dotenv`](packages/wj-dotenv) | Available |
| Layered config | [`packages/wj-config`](packages/wj-config) | Available |
| Logging helpers | [`packages/wj-log`](packages/wj-log) | Available |
| Argv helpers | [`packages/wj-cli-args`](packages/wj-cli-args) | Available |
| HTTP CLI | [`apps/wj-fetch`](apps/wj-fetch) | Available |
| CRUD API | [`apps/wj-notes-api`](apps/wj-notes-api) | Available (in-memory REST) |
| Static site generator | [`apps/wj-sitegen`](apps/wj-sitegen) | Available |
| Webhook worker | [`apps/wj-webhook`](apps/wj-webhook) | Available |
| HTTP client helpers | [`packages/wj-http-client`](packages/wj-http-client) | Available |
| JSON utilities | [`packages/wj-json-util`](packages/wj-json-util) | Available |
| Directory walk | [`packages/wj-fs-walk`](packages/wj-fs-walk) | Available |
| String templates | [`packages/wj-template`](packages/wj-template) | Available |
| UUID v1/v4/v5 | [`packages/wj-uuid`](packages/wj-uuid) | Blocked on stdlib wiring (see package README) |
| Semver | [`packages/wj-semver`](packages/wj-semver) | Available |

See [docs/ROADMAP.md](docs/ROADMAP.md) for the full package catalog.

## Layout

```
apps/        # Binaries
packages/    # Libraries
docs/        # Roadmap, stdlib coverage, progress
```

## Building

```bash
unset CARGO_TARGET_DIR
cd /path/to/windjammer && cargo build --release

export WJ=/path/to/windjammer/target/release/wj
cd apps/wj-hello
$WJ test
$WJ build --release src
cd build && cargo run --release
```

Contributor guidelines: [AGENTS.md](AGENTS.md).  
Stdlib usage across seeds: [docs/STDLIB_COVERAGE.md](docs/STDLIB_COVERAGE.md).

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
