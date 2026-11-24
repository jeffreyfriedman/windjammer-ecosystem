# wj-oauth

OAuth 2.0 and OpenID Connect client logic without the HTTP: PKCE, authorization URLs, callback checking, token requests, token and discovery responses (typed `@json`), and ID token validation. Everything is a pure function; you perform the HTTP calls and pass in the clock and the random bytes.

```windjammer
use wj_oauth::pkce::challenge_s256
use wj_oauth::request::accept_callback
use wj_oauth::request::parse_callback
use wj_oauth::jwt::validate_id_token_hs256

let code = accept_callback(parse_callback(redirect_url), expected_state)?
// POST token_request_authorization_code(...) to the token endpoint, parse_token_response(body), then:
let claims = validate_id_token_hs256(id_token, secret, expect)?
```

## API

- `pkce`: `challenge_s256`, `verifier_from_random`, `is_valid_verifier`, `verify_s256`
- `request`: `authorization_url`, `parse_callback`, `accept_callback` (state compared in constant time), `token_request_authorization_code`, `token_request_refresh`, `token_request_client_credentials`, `basic_auth_header`, `bearer_header`, `form_encode`
- `token`: `parse_token_response`, `parse_discovery` (checks the issuer), `expires_at`, `is_expired`
- `jwt`: `split`, `verify_hs256`, `validate_claims`, `validate_id_token_hs256`

Tokens are checked against PyJWT, encoding against Python's `urllib.parse.quote`, PKCE against RFC 7636 appendix B. `alg: none` is refused. RS256, ES256 and PS256 are refused with an explicit error: `std` has no RSA or ECDSA yet.

## Build / test

```bash
cd packages/wj-oauth
wj test --interpret
wj test
```
