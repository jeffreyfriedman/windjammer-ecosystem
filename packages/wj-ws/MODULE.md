# wj-ws

## Purpose
RFC 6455 frame codec, message assembly and handshake text, independent of any socket.

## Inputs / outputs
- In: bytes (frames), header text (handshake).
- Out: `Frame` / `Message` values or `Err(message)`.

## Layout
- `src/frame.wj`, `src/message.wj`, `src/handshake.wj`; `tests/codec_test.wj` (websockets 12.0 oracle), `tests/protocol_test.wj`.

## Invariants
- Pure: no I/O, no capabilities. `std::net` has no raw sockets yet, so the transport stays with the caller.
- Control frames: at most 125 bytes, never fragmented. Servers must receive masked frames; callers pick the key.
