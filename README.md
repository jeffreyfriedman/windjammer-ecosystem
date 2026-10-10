# Windjammer Ecosystem

Apps and packages that show what everyday Windjammer development looks like: CLIs, small APIs, config libraries, and similar projects built with Windjammer’s standard library plus a few Cargo crates via rust-interop.

## Goals

- Idiomatic Windjammer for application logic
- At most a handful of Cargo dependencies (for example `clap`, `ureq`/`reqwest`, `serde_json`, one DB or file crate)
- No project-local `ffi/` or hand-written Rust in app/package sources
- Clear, tested examples others can copy

## Status

Experimental. These are small, mostly synchronous building blocks (about 40 packages
and 14 demo apps). Capability levels per package are in [docs/PROGRESS.md](docs/PROGRESS.md);
most are wrappers over `std::*` or documented subsets of a format, not full implementations.
Test counts there are "present", and builds were not re-run when the docs were last edited.

Apps: `wj-hello`, `wj-fetch`, `wj-find`, `wj-first-hour`, `wj-form-parse`, `wj-notes-api`,
`wj-auth-api`, `wj-sitegen`, `wj-webhook`, `wj-proxy`, `wj-pipeline`, `wj-scheduler`,
`wj-todo-cli`, `wj-migrate-cli`.

## Packages by capability

| Level | Packages |
|---|---|
| Wrappers over `std::*` | wj-sha, wj-hash, wj-jwt, wj-regex, wj-yaml, wj-compress, wj-json-util, wj-http-client (sync only), wj-fs-walk, wj-base64 |
| Own parsers (documented subsets) | wj-toml, wj-csv, wj-cron, wj-url, wj-querystring, wj-semver, wj-timefmt, wj-duration, wj-glob, wj-multipart, wj-template, wj-dotenv, wj-cli-args, wj-cookie, wj-path, wj-mime |
| HTTP building blocks | wj-router, wj-cors, wj-headers, wj-rate-limit, wj-validate |
| Runtime / domain | wj-sync (channels, pool), wj-event, wj-retry, wj-log, wj-config, wj-migrate, wj-uuid, wj-inflect |

## Roadmap: the hard 20%

Not present, and what real services need next:

- Typed serde (derive-style encode/decode, not string-keyed JSON helpers)
- Async HTTP server/client with middleware
- Database access with typed rows
- Shared error types and error context
- Cache (TTL, LRU)
- WebSockets
- Email (SMTP, templates)
- i18n / message catalogs

## Known debt

[docs/COMPILER_WORKAROUNDS.md](docs/COMPILER_WORKAROUNDS.md) lists code shaped around compiler
limitations and cross-package `wj.toml` paths that point at generated `build/` directories.
[docs/MODULE_MAP.md](docs/MODULE_MAP.md) describes module responsibilities.

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
