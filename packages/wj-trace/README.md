# wj-trace

Distributed tracing building blocks in pure Windjammer: trace and span ids, parent-child spans, attributes and error
status, head sampling, W3C `traceparent` propagation and a pluggable exporter. Ids come from a seeded generator and time
is an argument, so a test run always produces the same trace.

```windjammer
use wj_trace::new_sampled_tracer
use wj_trace::memory_exporter

fn main() {
    let mut t = new_sampled_tracer(42, 100)          // seed 42, keep 100% of traces
    let req = t.start_root("GET /users", now)
    let db = t.start_child(req, "db.query", now)
    t.set_attr(db, "db.table", "users")
    t.end(db, now + 4)
    t.end(req, now + 9)
    let mut out = memory_exporter()
    t.flush(out)                                     // any type with `impl Exporter` works
}
```

## How it behaves

| Call | Effect |
|---|---|
| `start_root`, `start_child` | new span handles; children share the trace id and sampling decision |
| `set_attr`, `set_error`, `end` | annotate and finish; unsampled spans are dropped at `end` |
| `traceparent(h)`, `start_from_traceparent(header, ...)` | propagate across services (`00-<trace>-<span>-<flags>`) |
| `flush(exporter)` | hand finished spans, oldest first, to an `Exporter` |

Implement `Exporter` (`fn export(self, span: Span)`) to send spans to a log, a collector or a test recorder;
`MemoryExporter` is included.

## Layout

```
src/lib.wj
tests/trace_test.wj
```

## License

MIT OR Apache-2.0
