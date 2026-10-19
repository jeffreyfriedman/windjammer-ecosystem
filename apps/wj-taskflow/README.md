# wj-taskflow

A task-management REST API (users, projects, tasks, bearer-token auth, per-project roles, rate limiting) built from
ecosystem packages. All state is in memory; no database or network is needed to run the tests.

| Concern | Package |
|---|---|
| route patterns (`/api/v1/tasks/:id`) | `wj-router` |
| HS256 access tokens | `wj-jwt` |
| password hashing (bcrypt) | `wj-hash` |
| fixed-window rate limiting per client key | `wj-rate-limit` |
| input validation (lengths, email, enum values) | `wj-validate` |
| query-string filters | `wj-querystring` |
| ordered SQL schema files | `wj-migrate` |

## Layout

```
src/domain/    pure application: model, store (in-memory port), rules, auth, projects, tasks, app (routing)
src/adapters/  http_server.wj: std::http server, the only code that touches sockets or the clock
src/main.wj    composition root
migrations/    reference PostgreSQL schema for a database-backed store
tests/         API-level tests through Taskflow::handle, store tests, migration tests
```

`Taskflow::handle(method, target, authorization, client_key, now_ms, body)` is the whole application surface; tests call it
directly. A database store would implement the same operations as `TaskStore` (the schema in `migrations/` is the contract).

## Endpoints

| Method | Path | Notes |
|---|---|---|
| GET | `/health` | open |
| POST | `/api/v1/auth/register`, `/api/v1/auth/login` | open; login returns `{"token", "expires_in"}` |
| GET | `/api/v1/auth/me` | bearer token |
| GET, POST | `/api/v1/projects` | list the caller's projects / create (creator becomes owner) |
| GET, DELETE | `/api/v1/projects/:id` | members read; owner/admin delete |
| POST | `/api/v1/projects/:id/members` | owner/admin; `{"user_id", "role"}` with role `admin`, `member` or `viewer` |
| GET, POST | `/api/v1/projects/:id/tasks` | filters `status`, `priority`, `assigned_to`, `q` |
| GET, PATCH, DELETE | `/api/v1/tasks/:id` | viewers read only |
| POST | `/api/v1/tasks/:id/assign` | assignee must be a project member |

Errors are `{"error": "..."}` with 400 (bad JSON), 401, 403, 404, 405, 409, 422 (validation) or 429 (with `Retry-After`).

## Run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj
for p in wj-router wj-jwt wj-hash wj-rate-limit wj-validate wj-migrate wj-querystring; do (cd packages/$p && $WJ build --release src); done
cd apps/wj-taskflow
$WJ test
$WJ build --release src
cd build && TASKFLOW_JWT_SECRET=change-me-to-a-long-random-string PORT=8080 cargo run --release
```

`TASKFLOW_RATE_LIMIT` sets requests per minute per `X-Client-Key` (default 100). The server keeps data in memory, so it
resets on restart.
