# wj-oauth

## Purpose
Client-side OAuth 2.0 / OIDC building blocks: PKCE, requests, responses, ID token validation.

## Inputs / outputs
- In: endpoints, client credentials, the redirect URL, token and discovery JSON, a JWT, the current time.
- Out: URLs and form bodies to send, parsed `TokenResponse` / `Discovery` / `Claims`, or `Err(message)`.

## Layout
- `src/pkce.wj`, `src/request.wj`, `src/token.wj`, `src/jwt.wj`, `src/b64.wj`; `tests/oauth_test.wj`.

## Invariants
- Pure: no I/O, no capabilities, no clock, no randomness (callers pass them in).
- Secrets and signatures are compared in constant time; unsigned and unsupported algorithms are refused, never skipped.
