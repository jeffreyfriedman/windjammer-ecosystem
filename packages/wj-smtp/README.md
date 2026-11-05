# wj-smtp

Email as text. Build an RFC 5322 / MIME message (encoded-word headers, quoted-printable bodies, `multipart/alternative`, base64 attachments), then plan the SMTP exchange as a list of steps. Headers and addresses are validated, so a line break in a subject or recipient cannot smuggle in extra headers or commands.

```windjammer
use wj_smtp::address::address
use wj_smtp::message::new_message
use wj_smtp::message::build
use wj_smtp::smtp::plan

let mut m = new_message(address("Ada", "ada@example.com"), [address("", "bob@example.org")], "Hello", "Hi Bob\n")
m.message_id = "20260105.1@example.com"
let text = build(m)?
let steps = plan("client.example.com", address("", "ada@example.com"), [address("", "bob@example.org")], text, "", "")?
```

## API

- `address`: `Address`, `address`, `parse_address`, `format_address`, `is_valid_email`
- `message`: `Message`, `new_message`, `build`, `format_date`, `Attachment`
- `encode`: `encoded_words` (RFC 2047), `quoted_printable`, `base64_lines`
- `smtp`: `plan` (EHLO, AUTH PLAIN, MAIL FROM, RCPT TO, DATA, QUIT), `parse_reply`, `step_succeeded`, `extensions`, `auth_mechanisms`, `max_size`, `dot_stuff`, `auth_plain`

Quoted-printable, base64, dates and address formatting match Python's `quopri`, `base64` and `email.utils`; a complete message with attachment parses back cleanly with Python's `email` package.

`std::net` has no raw TCP yet, so transport is yours: send each `Step.send`, read the reply, call `step_succeeded`, move on.

## Build / test

```bash
cd packages/wj-smtp
wj test --interpret
wj test
```
