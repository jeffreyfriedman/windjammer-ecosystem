# wj-middleware

## Purpose
Closure-based request/response middleware pipeline with early responses and onion-ordered unwinding.

## Inputs / outputs
- In: `Request` (method, path, headers, body), closures per layer, a terminal handler closure.
- Out: `Response` (status, headers, body).

## Layout
- `src/lib.wj` (`Pipeline`, `Request`, `Response`, `Flow`), `tests/middleware_test.wj`.

## Invariants
- Layers run outermost first on the way in and innermost first on the way out.
- A layer that returns `Flow::Respond` skips the handler and all inner layers; outer layers' `after` hooks still run.
- Header names compare case-insensitively.
- No clock, no I/O: `[capabilities]` is empty.
