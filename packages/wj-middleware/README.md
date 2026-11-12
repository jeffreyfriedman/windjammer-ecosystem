# wj-middleware

A request/response middleware pipeline in pure Windjammer. A layer is a value that implements `Layer`: it can rewrite
the request on the way in (`before`), answer early (auth, rate limits, validation), and rewrite the response on the way
out (`after`). Layers added first are outermost, and a layer that answers early still lets the layers outside it see the
response. The terminal handler is a closure and can capture whatever it likes.

```windjammer
use wj_middleware::new_pipeline
use wj_middleware::request
use wj_middleware::response
use wj_middleware::RequireHeader
use wj_middleware::ResponseHeader

fn main() {
    let mut app = new_pipeline()
    app.add(Box::new(ResponseHeader { header: "Server", value: "wj" }))
    app.add(Box::new(RequireHeader { header: "Authorization", status: 401, body: "sign in first" }))

    let reply = app.handle(
        request("GET", "/users").with_header("Authorization", "Bearer abc"),
        |req| response(200, "users for " + req.path),
    )
}
```

## Writing a layer

```windjammer
use wj_middleware::Flow
use wj_middleware::Layer

struct Timing {}

impl Layer for Timing {
    fn name(&self) -> string { "timing" }
    fn before(&self, req: Request) -> Flow { Flow::Continue(req) }
    fn after(&self, req: Request, resp: Response) -> Response { resp.with_header("X-Handled", "yes") }
}
```

## How it behaves

| Call | Effect |
|---|---|
| `add(layer)` | push a `Box<dyn Layer>` inside the layers already added |
| `handle(req, handler)` | run the onion: layers in, handler, layers out |
| `names()`, `len()` | inspect the stack |
| `ResponseHeader`, `RequireHeader`, `PathPrefix` | ready-made layers |

`before` returns `Flow::Continue(req)` or `Flow::Respond(resp)`; `after` receives the request as that layer passed it
inward. Headers are looked up case-insensitively. Pair it with `wj-router` for the handler, and write layers that wrap
`wj-token-bucket`, `wj-cors` and `wj-trace`.

## Workaround note: closures in struct fields

The first design stored each layer as a pair of closures in the pipeline. The compiler build at wj `b5444baa7` does not
yet carry that shape through to Rust: `Box<dyn Fn(A) -> R>` is emitted as `Box<dyn Fn()>` and a closure passed to a
`fn(A) -> R` parameter cannot be stored in a `fn(A) -> R` field. Layers are therefore trait objects (which compile and
run identically interpreted and compiled), and the handler, a plain parameter, stays a closure. Neutral repros live with
the compiler findings (`dyn_fn_signature_dropped.wj`, `closure_in_struct_field.wj`); when closure fields land, a
closure-based `layer(name, before, after)` convenience can sit beside `add`.

## Layout

```
src/lib.wj
tests/middleware_test.wj
```

## License

MIT OR Apache-2.0
