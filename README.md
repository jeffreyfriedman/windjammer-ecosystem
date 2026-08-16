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
| `.env` loader | [`packages/wj-dotenv`](packages/wj-dotenv) | Planned |
| HTTP CLI | `apps/wj-fetch` | Planned |
| CRUD API | `apps/wj-notes-api` | Planned |
| Static site generator | `apps/wj-sitegen` | Planned |
| Webhook worker | `apps/wj-webhook` | Planned |

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
