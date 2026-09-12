# Cross-ecosystem top-100 gap analysis

Synthesis of download leaders from **npm**, **PyPI**, **crates.io**, and **Go** de-facto standards, mapped onto this repository’s scope (CLI + small HTTP APIs + libraries). Updated 2026-08-23.

Literal top-100 lists are dominated by tooling, frameworks, and transitive plumbing. After filtering (see “Out of scope”), ~35 concerns cover everyday backend/CLI work.

## Sources (indicative)

| Registry | Snapshot | Notes |
|---|---|---|
| npm | Weekly leaders (chalk, commander, ajv, uuid, zod, lodash, dotenv, axios, express, jwt, glob, mime, …) | Transitive CLI utils inflate ranks |
| PyPI | [hugovk/top-pypi-packages](https://github.com/hugovk/top-pypi-packages) 30-day CSV | boto3 #1; **pyyaml** ~#23; pydantic, dateutil, pyjwt high |
| crates.io | All-time + “State of the Crates 2025” | serde/tokio dominate; reqwest, clap, chrono, uuid, base64 |
| Go | Ecosystem norms | cobra/viper, gin/chi, yaml.v3, jwt, migrate, httprate |

## Scorecard (Pareto concerns)

| Concern | Status | Ours |
|---|---|---|
| HTTP client | ✅ | `wj-http-client`, `std::http` |
| HTTP routing | ✅ | `wj-router`, `std::http` |
| JSON | ✅ | `wj-json-util`, `std::json` |
| Env / config | ✅ | `wj-dotenv`, `wj-config` |
| CLI args | ✅ | `wj-cli-args` |
| Logging | ✅ | `wj-log` |
| Retry | ✅ | `wj-retry` |
| Validation | ✅ | `wj-validate` (email/url/uuid + ranges + `all_ok`) |
| Query string | ✅ | `wj-querystring` |
| Path / walk | ✅ | `wj-path` (std join/file_name + sugar), `wj-fs-walk` |
| Glob | ✅ | `wj-glob` (`*`/`?`/`**`) |
| Semver | ✅ | `wj-semver` |
| CORS | ✅ | `wj-cors` |
| Cookies | ✅ | `wj-cookie` |
| MIME | ✅ | `wj-mime` (full thin-wrap `std::mime`; P3.243 ✅) |
| Duration | ✅ | `wj-duration` |
| Date/time | ✅ | `wj-timefmt` (offsets + epoch) |
| Templates | ✅ | `wj-template` (defaults, HTML escape) |
| SHA hashing | ✅ | `wj-sha` |
| UUID | ✅ | `wj-uuid` |
| Base64 | ✅ | `wj-base64` |
| **YAML** | ✅ | `wj-yaml` (`std::yaml.to_json` + path getters; P3.244 ✅) |
| **JWT** | ✅ | `wj-jwt` / `std::jwt` |
| **DB migrate** | ✅ | `wj-migrate` (domain + `std::db` apply) |
| **Rate limit** | ✅ | `wj-rate-limit` |
| **Security headers** | ✅ | `wj-headers` |
| **Multipart** | ✅ | `wj-multipart` |
| TOML | ✅ | `wj-toml` (arrays + dotted keys + inline tables → flat map) |
| CSV | ✅ | `wj-csv` |
| Regex package | ✅ | `wj-regex` / `std::regex` |
| Inflect / slug | ✅ | `wj-inflect` |
| Events | ✅ | `wj-event` |
| Proxy app | ✅ | `wj-proxy` |
| Todo CLI app | ✅ | `wj-todo-cli` (validate + JSON list/export/import + stats + clear) |
| Migrate CLI app | ✅ | `wj-migrate-cli` (apply + status via `wj-migrate`) |
| Scheduler CLI app | ✅ | `wj-scheduler` (check/next/explain + crontab list/due via `wj-cron`) |

**Rough coverage:** ~34/35 (~97%) of Pareto concerns for CLI + small HTTP APIs.

## Priority queue

### P0 — multi-registry top-50 / production blockers

1. ~~`wj-yaml`~~ ✅ — `to_json` graduated to `std::yaml`; getters stay package sugar
2. ~~Unblock `wj-uuid` / `wj-base64`~~ ✅
3. ~~`wj-jwt`~~ ✅ — HS256 via `std::jwt`
4. ~~Richer datetime beyond Zulu~~ ✅ (`wj-timefmt` offsets + epoch)
5. ~~`wj-migrate`~~ ✅ — domain + `std::db` apply adapter (`db_apply::apply_url`)

### P1 — production HTTP hardening / Wave 2 apps

- ~~`wj-rate-limit`, `wj-headers`~~ ✅
- ~~`wj-multipart`~~ ✅, ~~`wj-querystring`~~ ✅
- Apps: ~~`wj-todo-cli`~~ ✅, ~~`wj-proxy`~~ ✅ (rate-limit + headers), ~~`wj-auth-api`~~ ✅, ~~`wj-notes-api` hardening~~ ✅, ~~`wj-webhook` hardening~~ ✅ (CORS/headers/validate)
- ~~`wj-csv`~~ ✅, ~~deepen TOML in config~~ ✅ (`wj-toml`)

### P2

- ~~`wj-inflect`~~ ✅
- ~~`wj-event`~~ ✅
- ~~password hashing~~ ✅ (`wj-hash` / bcrypt via `std::crypto`)
- ~~gzip middleware~~ ✅ (`wj-compress` negotiate + `std::compress` body codecs)

### P2+ (nice-to-have utilities)
- ~~cron expression parsing~~ ✅ (`wj-cron` parse/match + `next_run` + aliases + range/step)

## Out of scope (wrong repo)

React/Next/Vue, webpack/vite/eslint, boto3 and cloud SDKs, numpy/pandas/scipy, full ORMs (Prisma/Spring/Rails), image libs, job queues, gRPC/protobuf product stacks, game/browser/GPU engines.

## Related

- [ROADMAP.md](ROADMAP.md) — package catalog and waves
- [PROGRESS.md](PROGRESS.md) — weekly green status
- [STDLIB_COVERAGE.md](STDLIB_COVERAGE.md) — std usage matrix
