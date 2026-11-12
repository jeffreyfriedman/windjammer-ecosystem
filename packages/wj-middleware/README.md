# wj-middleware

A request/response middleware pipeline in pure Windjammer. Layers are closures, so they capture their own
configuration; each layer can rewrite the request on the way in, answer early (auth, rate limits, validation), and
rewrite the response on the way out (headers, logging, compression decisions). Layers added first are outermost, and a
layer that answers early still lets the layers outside it see the response.

```windjammer
use wj_middleware::new_pipeline
use wj_middleware::request
use wj_middleware::response
use wj_middleware::Flow

fn main() {
    let mut app = new_pipeline()
    app.on_response("server-header", Box::new(|_req, resp| resp.with_header("Server", "wj")))
    app.on_request("auth", Box::new(|req| {
        if req.header("Authorization") == "" {
            Flow::Respond(response(401, "sign in first"))
        } else {
            Flow::Continue(req)
        }
    }))

    let reply = app.handle(
        request("GET", "/users").with_header("Authorization", "Bearer abc"),
        Box::new(|req| response(200, "users for " + req.path)),
    )
}
```

## How it behaves

| Call | Effect |
|---|---|
| `on_request(name, f)` | `f(req) -> Flow::Continue(req)` or `Flow::Respond(resp)` |
| `on_response(name, f)` | `f(req, resp) -> resp`, where `req` is the request as the inner layers left it |
| `layer(name, before, after)` | both hooks in one layer |
| `handle(req, handler)` | run the onion: layers in, handler, layers out |
| `names()`, `len()` | inspect the stack |

Headers are looked up case-insensitively. Pair it with `wj-router` for the handler, and with `wj-token-bucket`,
`wj-cors` and `wj-trace` as layers.

## Implementation note

The pipeline loops forward through the `before` hooks and unwinds backwards through the `after` hooks rather than
recursing through a `next` continuation: closure fields are bound to a local before the call (`let f = layer.before`,
then `f(req)`), which works the same interpreted and compiled.

## Layout

```
src/lib.wj
tests/middleware_test.wj
```

## License

MIT OR Apache-2.0
