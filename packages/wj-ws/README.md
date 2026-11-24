# wj-ws

WebSocket (RFC 6455) wire format without the socket: encode and decode frames, reassemble fragmented messages, and run the opening handshake. Bytes in, frames out, so it works over any transport and is trivial to test.

```windjammer
use wj_ws::frame::text_frame
use wj_ws::frame::encode
use wj_ws::frame::decode
use wj_ws::frame::DEFAULT_MAX_PAYLOAD
use wj_ws::handshake::accept_key
use wj_ws::handshake::client_request
use wj_ws::handshake::parse_request
use wj_ws::handshake::server_response

let wire = encode(text_frame("Hello"), [0x37, 0xfa, 0x21, 0x3d])?   // clients mask
let got = decode(wire, DEFAULT_MAX_PAYLOAD)?                         // Ok(None) means "need more bytes"
```

## API

- `frame`: `Frame`, `text_frame`, `binary_frame`, `ping_frame`, `pong_frame`, `close_frame`, `parse_close`, `encode(frame, mask_key)`, `decode(bytes, max_payload)`
- `message`: `Assembler` joins fragments into `Message::Text | Binary | Ping | Pong | Close`, allowing control frames between fragments and checking UTF-8 and size limits
- `handshake`: `accept_key`, `client_request`, `parse_request`, `server_response`, `validate_response`

`decode` rejects reserved bits, unknown opcodes, non-minimal lengths, oversized or fragmented control frames and payloads over the limit before the bytes arrive. Frames are checked against the `websockets` library.

## Layout

```
src/frame.wj  src/message.wj  src/handshake.wj
tests/
```

## Build / test

```bash
cd packages/wj-ws
wj test --interpret
wj test
```
