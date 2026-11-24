# wj-trace

## Purpose
Spans, trace ids, attributes, sampling, `traceparent` propagation and an exporter trait, deterministic under a seed.

## Inputs / outputs
- In: a seed, a sampling percentage, span names/attributes, `now` in milliseconds, incoming `traceparent` headers.
- Out: `Span` values via `Exporter`, `traceparent` header strings.

## Layout
- `src/lib.wj` (`Tracer`, `Span`, `Exporter`, `MemoryExporter`), `tests/trace_test.wj`.

## Invariants
- Trace ids are 32 hex digits, span ids 16; the same seed yields the same ids.
- Sampling is decided once per trace at the root; children and remote continuations follow it.
- Only ended, sampled spans reach an exporter, each once.
- No clock, no I/O: `[capabilities]` is empty.
