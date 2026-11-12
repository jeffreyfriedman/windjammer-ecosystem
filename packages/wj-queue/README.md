# wj-queue

A job queue for background work, in pure Windjammer: priorities, delayed jobs, leases with heartbeats, retries with
exponential backoff, and a dead-letter list you can inspect and requeue. Time is an argument in milliseconds, so every
behaviour is deterministic and easy to test.

```windjammer
use wj_queue::new_queue
use wj_queue::queue_config

fn main() {
    let mut q = new_queue(queue_config(30000, 5, 1000, 300000))
    q.push_with("send-welcome-email", 5, 0, now)   // priority 5, ready now
    match q.pop(now) {
        Some(job) => {
            // ... do the work, then:
            q.ack(job.id)                          // or q.nack(job.id, "smtp timeout", now) to retry later
        },
        None => {},
    }
}
```

## How it behaves

| Call | Effect |
|---|---|
| `push`, `push_with` | enqueue with a priority (higher first) and an optional delay |
| `pop(now)` | lease the best ready job; a lease that runs out counts as a failed attempt |
| `ack(id)` | finished: the job is removed |
| `nack(id, error, now)` | failed: retry after `base * 2^(attempt-1)` (capped), or dead-letter after `max_attempts` |
| `extend(id, now)` | heartbeat: renew the lease of a slow job |
| `next_ready_at()` | when a worker loop should wake next |
| `dead_letters()`, `requeue_dead(id, now)` | inspect and revive jobs that ran out of attempts |
| `stats()` | enqueued, completed, retried, dead, waiting, leased |


## Layout

```
src/lib.wj
tests/queue_test.wj
```

## License

MIT OR Apache-2.0
