# wj-smtp

## Purpose
Build MIME messages and plan SMTP deliveries without owning a socket.

## Inputs / outputs
- In: addresses, text, attachment bytes, a date and a message id chosen by the caller.
- Out: the message text (CRLF line ends), SMTP steps, parsed server replies; every failure is `Err(message)`.

## Layout
- `src/encode.wj`, `src/address.wj`, `src/message.wj`, `src/smtp.wj`; `tests/encode_test.wj` (Python oracles), `tests/message_test.wj`.

## Invariants
- Pure: no I/O, no capabilities. No randomness or clock: `Message-ID`, `Date` and boundaries derive from caller input.
- Bcc recipients appear only in the envelope plan, never in headers. Header values and addresses containing CR or LF are refused.
