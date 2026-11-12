# wj-session

Server-side sessions for web apps, in pure Windjammer: an in-memory store with idle and absolute timeouts, per-session
data, id rotation at login (session-fixation defence), a CSRF token per session, and the cookie helpers that go with
it. Time is an argument in milliseconds; ids come from you (a v4 UUID from `wj-uuid` is a good source), so the store
is deterministic and testable.

```windjammer
use wj_session::new_store
use wj_session::default_policy
use wj_session::set_cookie_value

fn main() {
    let mut sessions = new_store(default_policy())
    sessions.create(sid, csrf, now)
    sessions.set(sid, "user", "ada", now)

    // at login, swap the id so one planted before authentication is useless
    let _ = sessions.rotate(sid, fresh_sid, now)

    let header = set_cookie_value("sid", fresh_sid, 3600, true)
    // Set-Cookie: sid=...; Path=/; HttpOnly; SameSite=Lax; Secure; Max-Age=3600
}
```

| Call | Effect |
|---|---|
| `create(id, csrf, now)` | start a session |
| `touch(id, now)` | `Ok` and slide the idle timeout, or `Err(Miss::Unknown / IdleTimeout / AbsoluteTimeout)` |
| `set`, `get`, `remove` | per-session string data (each counts as use) |
| `rotate(old, new, now)` | move the session to a new id; the old id stops working |
| `csrf_token`, `csrf_valid` | per-session token, compared in constant time |
| `destroy(id)`, `purge_expired(now)` | logout and cleanup |
| `set_cookie_value`, `clear_cookie_value`, `id_from_cookie_header` | cookie headers |


## Layout

```
src/lib.wj
tests/session_test.wj
```

## License

MIT OR Apache-2.0
