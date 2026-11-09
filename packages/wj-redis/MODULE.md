# wj-redis

## Purpose
Talk to Redis over TCP (`std::net::TcpStream`), and run a small Redis-compatible server in tests.

## Inputs / outputs
- In: a `host:port`, a timeout in milliseconds, commands as `Vec<string>`.
- Out: `wj_cache::resp::Reply` values; every failure is `Err(message)` (`connection refused`, `timed out`, ...).

## Layout
- `src/client.wj` (`Client`: connect, send, receive, call, close), `src/server.wj` (`TestServer`: bind, accept, serve_one, serve_client);
  `tests/redis_test.wj`.

## Invariants
- The wire format is `wj-cache`'s RESP2 codec; this package only moves bytes.
- Replies come back in command order; bytes after a complete reply stay buffered for the next one.
- Tests use loopback only (`[capabilities]` grants `127.0.0.1`), run interpreted, compiled and under `wj test --interpret --simulate`.
