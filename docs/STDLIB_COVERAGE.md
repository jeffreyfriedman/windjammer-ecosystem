# Stdlib coverage

Windjammer `std::*` modules used on production paths in seed apps and packages.

| Std module | wj-hello | wj-dotenv | wj-config | wj-log | wj-cli-args | wj-fetch | wj-notes-api | wj-sitegen | wj-webhook | wj-http-client | wj-json-util | wj-fs-walk | wj-template | wj-uuid | wj-semver | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| (println / core) | yes | | | | | | | yes | yes | | | | | | language builtins |
| `std::fs` | | yes | | | | | | yes | | | | yes | | | files / dotenv / sitegen / walk |
| `std::strings` | | yes | | | yes | | yes | yes | yes | | yes | yes | yes | yes | yes | parse / paths / templates / uuid / semver |
| `std::env` / CLI | | | | | yes | yes | yes (`PORT`) | yes | yes (`PORT`, `WEBHOOK_SECRET`) | | | | | | argv / bind / secrets |
| `std::http` | | | | | | yes | yes (server) | | yes (server) | yes (client) | | | | | client + server / webhooks |
| `std::json` | | | | | | yes | yes | | yes | | yes | | | | bodies / config / events / util |
| `std::process` | | | | | | yes | | yes | | | | | | | exit codes |
| `std::log` | | | | yes | | | planned | | planned | | | | | | tagged helpers / workers / API |
| `std::time` | | | | | | | | planned | | | | | | planned | timestamps / uuid v1 |
| `std::random` | | | | | | | | | | | | | | planned | uuid v4/v1 entropy |
| `std::db` | | | | | | | optional | | | | | | | | CRUD persistence |
| `std::crypto` | | | | | | | | | yes (`sha256`) | | | | | planned (`sha1`) | webhook + uuid v5 |
| `std::encoding` | | | | | | | | | optional | | | | | yes | hex for uuid bytes |
| `std::regex` | | | | | | | | optional | | | | | | | frontmatter / routes |
| `std::csv` | | | | | | | | | | | | | | | later utilities |
| `std::map` / collections | | yes (`HashMap`) | yes (`HashMap`) | | | | yes (`HashMap`) | | | | | | yes (`HashMap`) | | config / notes / template vars |

Mark a cell **yes** when production `.wj` (not only tests) uses that module. Prefer one clear owner app or package per concern.
