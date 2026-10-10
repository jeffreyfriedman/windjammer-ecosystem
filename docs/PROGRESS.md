# Capability table

Replaces the old weekly log. Levels:

- **Wrapper**: thin layer over a `std::*` module.
- **Subset**: own implementation of a documented subset of the format.
- **Domain**: app-level logic with its own rules.

Numbers are counted from the source (`pub fn` in `src/`, `fn test_*` in `*_test.wj`). Nothing was
built or run when this table was written, so "tests" means "tests present", not "passing".
Per-package details are in each package's README.

| Package | Level | pub fns | tests |
|---|---|---|---|
| wj-base64 | Wrapper + Basic-auth header helpers | 18 | 14 |
| wj-cli-args | Subset (flags, positionals) | 27 | 24 |
| wj-compress | Wrapper (Accept-Encoding negotiation, gzip) | 22 | 21 |
| wj-config | Subset (layered config) | 23 | 19 |
| wj-cookie | Subset (Cookie / Set-Cookie) | 23 | 20 |
| wj-cors | Subset | 17 | 17 |
| wj-cron | Subset (cron expressions) | 11 | 34 |
| wj-csv | Subset | 17 | 16 |
| wj-dotenv | Subset | 18 | 18 |
| wj-duration | Subset | 25 | 25 |
| wj-event | In-process event bus | 24 | 22 |
| wj-fs-walk | Wrapper | 22 | 18 |
| wj-glob | Subset | 19 | 28 |
| wj-hash | Wrapper (bcrypt) | 10 | 10 |
| wj-headers | Security-header presets | 21 | 20 |
| wj-http-client | Wrapper (sync, no middleware) | 24 | 13 |
| wj-inflect | Subset (case, slug) | 17 | 22 |
| wj-json-util | Wrapper | 18 | 23 |
| wj-jwt | Wrapper (HS256) | 15 | 12 |
| wj-log | Subset | 18 | 17 |
| wj-migrate | Domain (SQL migration planning) | 25 | 25 |
| wj-mime | Lookup table | 35 | 24 |
| wj-multipart | Subset | 22 | 21 |
| wj-path | Subset | 21 | 20 |
| wj-querystring | Wrapper + helpers | 27 | 29 |
| wj-rate-limit | Fixed-window limiter | 20 | 17 |
| wj-regex | Wrapper | 19 | 16 |
| wj-retry | Backoff policy | 30 | 26 |
| wj-router | Path matching | 12 | 13 |
| wj-semver | Subset | 20 | 17 |
| wj-sha | Wrapper (SHA-256 hex) | 15 | 11 |
| wj-sync | Channels, shared state, pool (generic) | 57 | 50 |
| wj-template | Subset (`{{var}}`) | 18 | 23 |
| wj-timefmt | Subset (RFC 3339) | 25 | 27 |
| wj-toml | Subset (flat keys, sections) | 22 | 30 |
| wj-url | Subset | 19 | 22 |
| wj-uuid | v1/v4/v5/v7 | 23 | 24 |
| wj-validate | Field validators | 21 | 36 |
| wj-yaml | Wrapper (YAML to JSON) | 17 | 26 |

Apps (`wj-auth-api`, `wj-fetch`, `wj-find`, `wj-first-hour`, `wj-form-parse`, `wj-hello`,
`wj-migrate-cli`, `wj-notes-api`, `wj-pipeline`, `wj-proxy`, `wj-scheduler`, `wj-sitegen`,
`wj-todo-cli`, `wj-webhook`) are demonstration apps; see each app's tests.
