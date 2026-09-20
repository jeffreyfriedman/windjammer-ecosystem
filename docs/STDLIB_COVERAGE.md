# Stdlib coverage

Windjammer `std::*` modules used on production paths in seed apps and packages.

| Std module | hello | dotenv | config | log | cli-args | fetch | notes-api | sitegen | webhook | http-client | json-util | fs-walk | template | uuid | semver | url | base64 | retry | sha | cors | router | path | auth-api | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| (println / core) | yes | | | | | | | yes | yes | | | | | | | | | | | | | | language builtins |
| `std::fs` | | yes | | | | | | yes | | | | yes | | | | | | | | | | | files / dotenv / sitegen / walk |
| `std::strings` | | yes | | | yes | | yes | yes | yes | | yes | yes | yes | yes | yes | yes | | | | | yes | yes | yes | parse / paths / yaml / mime / templates |
| `std::env` / CLI | | | | | yes | yes | yes | yes | yes | | | | | | | | | | | | | argv / bind / secrets |
| `std::http` | | | | | | yes | yes | | yes | yes | | | | | | | | | | | | client + server / webhooks |
| `std::json` | | | | | | yes | yes | | yes | | yes | | | | | | | | | | | yes | bodies / config / yaml / events / util |
| `std::process` | | | | | | yes | | yes | | | | | | | | | | | | | | exit codes |
| `std::log` | | | | yes | | | planned | | planned | | | | | | | | | | | | | tagged helpers |
| `std::time` | | | | | | | | planned | | | | | | planned | | | | | | | | timestamps / uuid v1 |
| `std::random` | | | | | | | | | | | | | | planned | | | | | | | | uuid entropy |
| `std::db` | | | | | | | optional | | | | | | | | | | | | | | | yes via `wj-migrate` `db_apply` |
| `std::crypto` | | | | | | | | | yes | | | | | planned | | | | | yes | | | yes via `wj-hash` | webhook + sha + uuid v5 + auth-api bcrypt |
| `std::jwt` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-jwt` | auth-api HS256 tokens |
| `std::compress` | | | | | | | | | | | | | | | | | | | | | | yes | auth-api gzip bodies |
| `std::encoding` | | | | | | | | | optional | | | | | yes | | | planned | | | | | hex / base64; form_* RED (querystring) |
| `std::config` | | | yes | | | | | | | | | | | | | | | | | | | | parse_flat via wj-config + first-hour; merge local |
| `std::uuid` | | | | | | | | | | | | | | yes | | | | | | | | | first-hour + wj-uuid |
| `std::path` | | | | | | | | | | | | | | | | | | | | | | yes | join; glob_match RED |
| `std::regex` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-regex` |
| `std::csv` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-csv` |
| `std::yaml` | | | | | | | | | | | | | | | | | | | | | | yes via `wj-yaml` |
| `std::map` / collections | | yes | yes | | | | yes | | | | | | yes | | | | | | | | yes | | yes | config / notes / template / router / cookie |


Mark a cell **yes** when production `.wj` (not only tests) uses that module. Prefer one clear owner app or package per concern.
