# Roadmap

Catalog of seed apps and packages for common Windjammer application work.

## Design principles

- Idiomatic Windjammer for business logic
- Standard library first; rust-interop for a small set of Cargo crates when needed
- No project-local `ffi/` or hand-edited generated Rust
- Hexagonal layout in apps (domain / adapters / `main.wj`)
- Tests in Windjammer (`tests/*_test.wj`)

## Wave 1

| Name | Kind | Use case | Dependencies |
|---|---|---|---|
| `wj-hello` | app | Build-path smoke test | std only |
| `wj-first-hour` | app | Std-only first-hour card (uuid/time/path/config/encoding) | tip-green std |
| `wj-form-parse` | app | Multipart field parse dogfood | `wj-multipart` |
| `wj-dotenv` | package | `.env` / `KEY=VALUE` loading | `std::fs`, `std::strings` |
| `wj-fetch` | app | CLI HTTP GET + JSON | `std::http` / `std::cli` / `std::json`, or clap + ureq later |
| `wj-notes-api` | app | Small CRUD REST API | HTTP std + in-memory; optional `std::db` |
| `wj-config` | package | Layered config (file + env + defaults) | builds on `wj-dotenv` |
| `wj-log` | package | Logging helpers over `std::log` | std |
| `wj-cli-args` | package | Argv helpers until clap interop | std / later clap |

## Wave 2

| Name | Kind | Use case |
|---|---|---|
| `wj-sitegen` | app | Static site / docs generator |
| `wj-webhook` | app | Bot / webhook worker |
| `wj-http-client` | package | Idiomatic GET/POST JSON client |
| `wj-json-util` | package | Pretty-print, path get, merge |
| `wj-fs-walk` | package | Recursive directory walk |
| `wj-template` | package | Simple string templates |
| `wj-uuid` | package | UUID v1/v4/v5/v7 + nil/max |
| `wj-semver` | package | Parse / compare semver |
| `wj-url` | package | Parse / join URLs |
| `wj-base64` | package | Encode / decode |
| `wj-sha` | package | SHA-256 hex digest |
| `wj-retry` | package | Retry with backoff |
| `wj-timefmt` | package | RFC3339 format / parse |
| `wj-migrate` | package | Ordered SQL migrations |
| `wj-migrate-cli` | app | Apply SQL migrations CLI ✅ |
| `wj-router` | package | HTTP path match + params |
| `wj-cors` | package | CORS header helpers |
| `wj-todo-cli` | app | Local file CRUD CLI ✅ |
| `wj-proxy` | app | Small reverse proxy / request logger ✅ |
| `wj-auth-api` | app | JWT auth API dogfood (CORS, gzip, templates) ✅ |
| `wj-scheduler` | app | Cron check / next / explain CLI (dogfoods `wj-cron`) ✅ |

## After Wave 2: prefer native Windjammer over Wave 3 wrappers

**Default: yes — implement remaining ecosystem packages in pure Windjammer (and strengthen `std::*`) before wrapping Cargo crates.**

| Prefer pure WJ / std | Prefer rust-interop (Wave 3) only when… |
|---|---|
| Parsing, strings, HTTP helpers, CORS, routing, templates, retry, semver, URL, validation | Need mature C/Rust FFI (SQLite via `rusqlite`, OS TLS stacks, `tokio` runtime) |
| Crypto/encoding once std wiring is complete (`sha*`, base64, uuid, jwt) | Wrapping would only duplicate work we will replace anyway |
| Path / glob / mime / cookies / duration / rate-limit | Performance-critical codecs where a battle-tested crate is the product |

Rationale: wrappers create dual maintenance, leak Rust types into app code, and fight the “maximize Windjammer” directive. Dogfood std gaps with failing `windjammer/tests/` repros instead. Keep Wave 3 as a **thin allowlist** for irreducible dependencies, not a default path.

## Cross-ecosystem Pareto (not just NPM)

NPM alone under-weights gaps that dominate other ecosystems. Synthesis across **NPM, PyPI, crates.io, Go modules, Deno std, Maven/NuGet, RubyGems**:

| Concern | NPM | PyPI | Go | Deno | Java/.NET | Ruby | Ours |
|---|---|---|---|---|---|---|---|
| HTTP client | axios/fetch | requests/urllib3 | net/http | fetch | HttpClient | faraday | ✅ `wj-http-client` / `std::http` |
| JSON | lodash.get / json | pydantic json | encoding/json | JSON | Newtonsoft / System.Text.Json | oj | ✅ `wj-json-util` / `std::json` |
| Env/config | dotenv | python-dotenv | envconfig | dotenv | Microsoft.Extensions.Configuration | dotenv | ✅ `wj-dotenv` / `wj-config` |
| CLI | commander/yargs | click/argparse | cobra/flag | cliffy | System.CommandLine | thor/optparse | ✅ `wj-cli-args` |
| Logging | debug/pino | logging | zap/logrus | log | Serilog / SLF4J | logger | ✅ `wj-log` |
| Retry/backoff | async-retry | tenacity | cenkalti/backoff | — | Polly | retryable | ✅ `wj-retry` |
| Semver | semver | packaging | Masterminds/semver | semver | NuGet.Versioning | — | ✅ `wj-semver` |
| URL | url / qs | urllib.parse | net/url | URL | Uri | addressable | ✅ `wj-url` |
| Path | path | pathlib/os.path | path/filepath | std/path | System.IO.Path | Pathname | ✅ `wj-path` |
| Router | express/path-to-regexp | — | mux/chi | oak | ASP.NET routing | rack | ✅ `wj-router` |
| CORS | cors | flask-cors | rs/cors | — | — | rack-cors | ✅ `wj-cors` |
| Hashing | crypto | hashlib | crypto | std/crypto | SHA256 | Digest | ✅ `wj-sha` / `wj-uuid` / `wj-base64` |
| **Validation** | zod/joi | **pydantic** | go-playground/validator | zod | **FluentValidation** | ActiveModel | ✅ `wj-validate` |
| **Cookies** | cookie | http.cookies | net/http Cookie | — | CookieContainer | rack | ✅ `wj-cookie` |
| **Duration** | ms | timedelta / dateutil | time.Duration | — | TimeSpan | ActiveSupport::Duration | ✅ `wj-duration` |
| **YAML** | js-yaml | **PyYAML** (top-20) | yaml.v3 | yaml | SnakeYAML | Psych | ✅ `wj-yaml` (subset) |
| **TOML** | @iarna/toml | tomli | BurntSushi/toml | toml | Tomlyn | toml-rb | ✅ `wj-toml` (config subset) |
| CSV | csv-parse | csv | encoding/csv | csv | CsvHelper | csv | ✅ `wj-csv` (thin-wrap `std::csv`) |
| Date/time parse | dayjs/luxon | **python-dateutil** | time | datetime | NodaTime | Time | ✅ `wj-timefmt` (offsets + epoch) |
| JWT | jsonwebtoken | PyJWT | golang-jwt | djwt | IdentityModel | jwt | ✅ `wj-jwt` / `std::jwt` |
| Glob | minimatch | pathlib/fnmatch | filepath.Match | — | — | File.fnmatch | ✅ `wj-glob` |
| Hex/base64 | buffer | base64 | encoding/* | std/encoding | Convert | Base64 | ✅ `wj-base64` |
| DB migrate | knex/migrate | alembic | goose/migrate | — | FluentMigrator | ActiveRecord | ✅ `wj-migrate` (domain + apply) |
| Rate limit | express-rate-limit | limits | tollbooth | — | AspNetCore.RateLimiting | rack-attack | ✅ `wj-rate-limit` |
| Case/slug | lodash | inflection | — | — | Humanizer | ActiveSupport | ✅ `wj-inflect` |

**Verdict:** coverage is strong for HTTP/config/CLI/logging/retry/URL/path/router/glob/time/mime/yaml/rate-limit/headers/migrate/inflect, but **not yet Pareto-complete**. Highest remaining leverage: **jwt** (stdlib), unblock **uuid/base64**, Wave 2 apps. Skip cloud SDKs (boto3), numerics (numpy), and frameworks (Spring/Rails) — wrong repo.

Full top-100 distillation, scorecard, and priority queue: [CROSS_ECOSYSTEM_TOP100.md](CROSS_ECOSYSTEM_TOP100.md).

**Stdlib vs packages:** many seeds are temporary workarounds for missing/unwired `std::*`. Graduation plan: [STDLIB_GRADUATION.md](STDLIB_GRADUATION.md). Compiler agent queue: `windjammer/tests/STDLIB_ADOPTION_QUEUE.md`.

## Pareto packages (multi-ecosystem)

| Priority | Package | Cross-ecosystem analogues | Notes |
|---|---|---|---|
| P0 | `wj-validate` ✅ | zod / pydantic / FluentValidation | Landed: nonempty / len / email / int range / one_of / all_ok |
| P0 | `wj-path` ✅ | path / pathlib / filepath | Landed |
| P0 | `wj-cookie` ✅ | cookie / http.cookies / CookieContainer | Landed: Cookie + Set-Cookie parse/format |
| P0 | `wj-yaml` ✅ | js-yaml / PyYAML / yaml.v3 / Psych | Landed: subset parser + path getters; 12 tests green |
| P1 | `wj-duration` ✅ | ms / timedelta / time.Duration / TimeSpan | Landed: parse/format `1h30m` ↔ ms |
| P1 | `wj-glob` ✅ | minimatch / fnmatch / filepath.Match | Landed: `*`/`?`/`**` recursive segments |
| P1 | `wj-timefmt` ✅ | dayjs / dateutil / time / NodaTime | Landed: RFC3339 offsets + epoch + compare |
| P1 | `wj-mime` ✅ | mime / mimetypes | Landed: full thin-wrap `std::mime` |
| P1 | `wj-toml` ✅ | tomli / BurntSushi/toml / toml-rb | Landed: config subset → flat map; compose with `wj-config` |
| P1 | `wj-csv` ✅ | csv-parse / encoding/csv | Landed: thin `std::csv` parse/write |
| P1 | `wj-headers` ✅ | helmet | Secure default response headers |
| P1 | `wj-rate-limit` ✅ | express-rate-limit / Polly / rack-attack | Fixed window + standard headers |
| P1 | `wj-migrate` ✅ | knex / alembic / goose | Domain helpers + `db_apply::apply_url` + Postgres smoke (`scripts/smoke_postgres.sh`) |
| P2 | `wj-jwt` ✅ | jsonwebtoken / PyJWT / golang-jwt | Thin HS256 wrapper over `std::jwt` |
| P2 | `wj-multipart` ✅ | multer / multipart | Text-oriented form-data parse/format |
| P2 | `wj-querystring` ✅ | qs / urllib.parse | Deepened `wj-url` (`query_has` / `query_remove` / `with_query`) |
| P2 | `wj-event` ✅ | EventEmitter | Queue + pattern listeners (no closures) |
| P2 | `wj-inflect` ✅ | inflection / ActiveSupport / Humanizer | snake/camel/pascal/slug |
| P2 | `wj-hash` ✅ | bcrypt / passlib | Thin bcrypt over `std::crypto` |
| P2 | `wj-compress` ✅ | compression / gzip-middleware | Accept-Encoding + Base64 gzip via `std::compress` |
| P2 | `wj-regex` ✅ | re / regexp / java.util.regex | Thin wrappers over `std::regex` |
| P2 | `wj-cron` ✅ | node-cron / croniter / robfig/cron | Five-field parse/match + aliases + range/step + `next_run` |
| Skip here | React, Spring, Rails, boto3, numpy, webpack, eslint | — | Other repos / cloud / science / tooling |

## Wave 3 (rust-interop allowlist)

Use sparingly after Wave 2; prefer std / pure WJ packages above first:

| Concern | Preferred crates |
|---|---|
| CLI args | `clap` (or keep `wj-cli-args` until clap interop is ergonomic) |
| HTTP client | Prefer `std::http`; `ureq` / `reqwest` only if std gaps persist |
| JSON | Prefer `std::json`; `serde_json` only if needed |
| SQLite | `rusqlite` (legitimate FFI) |
| Logging | Prefer `std::log` / `wj-log`; `tracing` later if needed |
| Async | `tokio` when needed |

## Out of scope here

Browser/WASM product apps, OS kernels, full SQL engines, game engines, and wgpu wrappers belong in other Windjammer repositories.
