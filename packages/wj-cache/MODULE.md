# wj-cache

## Purpose
LRU + TTL cache and Redis RESP2 encoding and decoding.

## Inputs / outputs
- In: keys, string values, a millisecond clock value per call; bytes for RESP.
- Out: values, counters, `Reply` trees.

## Layout
- `src/lru.wj`, `src/resp.wj`; `tests/model_test.wj` (Python model), `tests/cache_test.wj`, `tests/resp_test.wj`.

## Invariants
- Pure: no I/O, no capabilities, no clock. Capacity is at least 1.
- Bulk lengths count bytes; replies nest at most 32 deep; bulk strings over 512 MiB are refused.
