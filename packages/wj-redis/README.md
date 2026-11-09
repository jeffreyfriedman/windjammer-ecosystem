# wj-redis

A Redis client for Windjammer, built on raw TCP from `std::net` and the RESP2 codec of `wj-cache`, plus a tiny
Redis-compatible `TestServer` so your tests can run without a Redis install.

```windjammer
use wj_redis::client::Client

fn main() {
    match Client::connect("127.0.0.1:6379", 2000) {
        Ok(mut redis) => {
            let _ = redis.call(vec!["SET", "greeting", "hello"])
            match redis.call(vec!["GET", "greeting"]) {
                Ok(reply) => println(reply.text),
                Err(e) => println("error: ${e}"),
            }
        },
        Err(e) => println("cannot connect: ${e}"),
    }
}
```

Grant the address in `wj.toml`:

```toml
[capabilities]
net.connect = ["127.0.0.1:6379"]
```

`TestServer` speaks PING, ECHO, SET, GET, DEL and INCR. In a single-threaded test, send a command, let the server
answer it with `serve_one`, then read the reply; or run `serve_client` on a thread.
