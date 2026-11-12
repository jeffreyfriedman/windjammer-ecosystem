# wj-session

## Purpose
Session store with timeouts, id rotation, CSRF tokens and session-cookie headers.

## Inputs / outputs
- In: session ids and CSRF tokens (caller-generated), string keys and values, `now` in milliseconds.
- Out: values, `Result<bool, Miss>` liveness checks, `Set-Cookie` header values.

## Layout
- `src/lib.wj` (`Store`, `Policy`, `Miss`, cookie helpers), `tests/session_test.wj`.

## Invariants
- A miss removes the stale session, so an expired id never comes back.
- The absolute limit is checked before the idle limit and ignores activity.
- `rotate` keeps data and CSRF token and invalidates the old id at once.
- No randomness and no clock inside: `[capabilities]` is empty.
