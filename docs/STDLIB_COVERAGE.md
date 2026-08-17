# Stdlib coverage

Windjammer `std::*` modules used on production paths in seed apps and packages.

| Std module | wj-hello | wj-dotenv | wj-fetch | wj-notes-api | wj-sitegen | wj-webhook | Notes |
|---|---|---|---|---|---|---|---|
| (println / core) | yes | | | | | | language builtins |
| `std::fs` | | yes | | | planned | | files / dotenv / sitegen |
| `std::strings` | | yes | | | planned | | parse / templates |
| `std::env` / CLI | | | yes | | | | argv via `env.args()` |
| `std::http` | | | yes | planned | | planned | client + server + webhooks |
| `std::json` | | | yes | planned | | planned | bodies / config |
| `std::process` | | | yes | | | | exit codes |
| `std::log` | | | | planned | | planned | workers / API |
| `std::time` | | | | | | planned | timestamps / retries |
| `std::db` | | | | optional | | | CRUD persistence |
| `std::crypto` | | | | | | planned | webhook signatures |
| `std::encoding` | | | | | | optional | base64 |
| `std::regex` | | | | | optional | | frontmatter / routes |
| `std::csv` | | | | | | | later utilities |
| `std::map` / collections | | yes (`HashMap`) | | planned | | | config / notes store |

Mark a cell **yes** when production `.wj` (not only tests) uses that module. Prefer one clear owner app or package per concern.
