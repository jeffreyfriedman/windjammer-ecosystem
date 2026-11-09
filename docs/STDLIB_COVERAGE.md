# Stdlib coverage

Windjammer `std::*` modules used on production paths in seed apps and packages.

| Std module | hello | dotenv | config | log | cli-args | fetch | notes-api | sitegen | webhook | http-client | json-util | fs-walk | template | uuid | semver | url | base64 | retry | sha | cors | router | path | auth-api | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| (println / core) | yes | | | | | | | yes | yes | | | | | | | | | | | | | | language builtins |
| `std::fs` | | yes | | | | | | yes | | | | yes | | | | | | | | | | | files / dotenv / sitegen / walk |
| `std::strings` | | yes | | | yes | | yes | yes | yes | | yes | yes | yes | yes | yes | yes | | | | | yes | yes | yes | parse / paths / yaml / mime / templates |
| `std::env` / CLI | | | | | yes | yes | yes | yes | yes | | | | | | | | | | | | | argv / bind / secrets |
| `std::http` | | | | | | yes | yes | | yes | yes | | | | | | | | | | | | client + server / webhooks |
| `std::json` | | | | | | yes | yes | | yes | | yes | | | | | | | | | | | yes | bodies / config / yaml / events / util; `wj-json-doc` keeps member order (std::json sorts keys) |
| `std::process` | | | | | | yes | | yes | | | | | | | | | | | | | | exit codes; `wj-proc` wraps `run_with_output` (status, stdout, stderr) and `/usr/bin/time` for CPU timing |
| `std::log` | | | | yes | | | planned | | planned | | | | | | | | | | | | | tagged helpers |
| `std::time` | | | | | | | | planned | | | | | | planned | | | | | | | | timestamps / uuid v1 |
| `std::random` | | | | | | | | | | | | | | planned | | | | | | | | uuid entropy |
| `std::db` | | | | | | | optional | | | | | | | | | | | | | | | yes via `wj-migrate` `db_apply` |
| `std::crypto` | | | | | | | | | yes | | | | | planned | | | | | yes | | | yes via `wj-hash` | webhook (HMAC-SHA256 + constant-time compare) + sha + uuid v5 + auth-api bcrypt |
| `std::jwt` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-jwt` | auth-api HS256 tokens |
| `std::compress` | | | | | | | | | | | | | | | | | | | | | | yes | auth-api gzip bodies |
| `std::encoding` | | | | | | | | | optional | | | | | yes | | | planned | | | | | hex / base64; `form_*` wired (querystring thin-wraps it) |
| `std::config` | | | yes | | | | | | | | | | | | | | | | | | | | parse_flat via wj-config + first-hour; merge local |
| `std::uuid` | | | | | | | | | | | | | | yes | | | | | | | | | first-hour + wj-uuid |
| `std::path` | | | | | | | | | | | | | | | | | | | | | | yes | join; `glob_match` wired (`wj-glob`) |
| `std::regex` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-regex` |
| `std::csv` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-csv` |
| `std::yaml` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-yaml` |
| `std::map` / collections | | yes | yes | | | | yes | | | | | | yes | | | | | | | | yes | | yes | config / notes / template / router / cookie |


Measured status for every package and app (interpreted and compiled test counts) is in the tip-health table of
[STDLIB_GRADUATION.md](STDLIB_GRADUATION.md); `std::sync`, `std::time` and `std::collections` are also used
widely (`wj-sync`, `wj-timefmt`, `wj-retry`, `wj-cron`, most apps) and are not given columns here.

Newer packages and the std modules they use: `wj-decimal` (`std::strings`), `wj-metrics` (`std::json` typed `@json` structs,
`std::strings`), `wj-markdown` (`std::strings`), `wj-ws` (`std::encoding`, `std::crypto` SHA-1, `std::strings`),
`wj-archive` (`std::encoding`, `std::strings`; DEFLATE and CRC-32 are written in Windjammer), `wj-oauth` (`std::crypto`
HMAC and constant-time compare, `std::encoding`, `std::json` with typed `@json` structs), `wj-smtp` (`std::encoding`,
`std::strings`), `wj-cache` (`std::collections`, `std::encoding`), `wj-redis` (`std::net` raw TCP, `std::thread`), `wj-cli-ui` (`std::env`, `std::fs` for `/dev/stdin`,
`std::strings`). Only `wj-cli-ui` declares capabilities (`env.read` for `NO_COLOR` and `TERM`, `fs.read` for `/dev/stdin`).

Mark a cell **yes** when production `.wj` (not only tests) uses that module. Prefer one clear owner app or package per concern.
