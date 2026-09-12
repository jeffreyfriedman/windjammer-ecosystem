# wj-event

In-process event bus in pure Windjammer — subscribe by pattern, emit, drain queue.

Windjammer has no first-class closures yet, so this package uses a **queue + listener registry** model: adapters drain pending events and dispatch to app-specific handlers.

## API

```windjammer
use wj_event

let bus = subscribe(new_bus(), "audit", "user.*")
let queued = emit(bus, "user.created", "{\"id\":1}")
let drained = drain(queued)
let ids = listener_ids(drained.0, "user.created")
```

Patterns: exact name, `*` (all), or `prefix.*` suffix wildcard.

## Layout

```
src/lib.wj
tests/event_test.wj
```

## License

MIT OR Apache-2.0
