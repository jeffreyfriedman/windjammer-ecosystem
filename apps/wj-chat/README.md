# wj-chat

A chat core in Windjammer: authenticated connections, rooms with presence, direct messages, bounded history, per-user
rate limiting and idle-connection expiry. The core (`Hub`) is transport-independent: it takes connection events and JSON
text frames and returns the frames to send, so it runs and is tested with no sockets, clock or network.

| Concern | Package |
|---|---|
| session tokens (HS256) | `wj-jwt` |
| per-user fixed-window rate limit | `wj-rate-limit` |

## Layout

```
src/domain/    config, frames (JSON in/out), hub (rooms, presence, DMs, history, rate limit, expiry), token
src/adapters/  transcript.wj: scripted stdin/stdout transport
src/main.wj    composition root
tests/         hub behavior tests and transcript tests
```

## Protocol

Client frames are JSON objects with a `kind`:

| kind | fields | effect |
|---|---|---|
| `auth` | `token` | sign in (a second sign-in for the same user replaces the first connection) |
| `join` / `leave` | `room` | membership; the room receives a `presence` frame with the member list |
| `say` | `room`, `text` | `message` frame to every member (max 2000 chars, must be a member) |
| `dm` | `to`, `text` | `dm` frame to the user, `ok` to the sender; error if the user is offline |
| `history` | `room`, optional `limit` | `history` frame with the latest messages, oldest first |
| `pong` | | keep-alive |

Server frames carry `kind` (`ok`, `error`, `message`, `dm`, `presence`, `history`), `room`, `from`, `text`, `users`, `items`.
Anything but `auth` and `pong` needs a signed-in connection and counts against the user's rate limit.

## Transports

The standard library has a WebSocket client but no WebSocket server, so no socket transport ships yet. A WebSocket adapter
only has to open connections (`Hub::connect`), pass text frames to `Hub::receive`, write the returned deliveries to the
addressed connections, call `Hub::disconnect` on close and poll `Hub::expired` for idle ones.

`wj-chat` itself reads a script from stdin and prints the deliveries. Time advances one second per line:

```text
open                      -> conn N
N {"kind":"join",...}     -> -> M {...} for each delivery
close N                   -> presence updates
```

## Build and test

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/wj
for p in wj-jwt wj-rate-limit; do (cd packages/$p && $WJ build --release src); done
cd apps/wj-chat
$WJ test
$WJ build --release src
cd build && printf 'open\n' | CHAT_JWT_SECRET=change-me cargo run --release
```
