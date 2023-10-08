# surrealdb-rust-dbpool
This is an example of how to use the `deadpool` crate to create a pool of db connections for surrealdb

I have been playing around a lot with surrealdb in my sideprojects and have been really enjoying it, however it lacked the ability to create db pools to pass around like you would do with postgres. So after some playing around with it I ended up with this :) I havent done any optimizations, this is just me getting it to "work".

This example shows using `Axum` but any web framework that allows you to have shared stat should work, just pass the `shared_pool` however the framework you're using does things.

## TODO
I need to implement the `recycle` function so that deadpool is able to throw out bad connections, so far though none of my connections have died, so thats good.


## Benchmarks

When running the web server and the surrealdb instance local I got the following results

```bash
./wrk -t12 -c400 -d30s http://127.0.0.1:3000/endpoint
Running 30s test @ http://127.0.0.1:3000/endpoint
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    32.77ms   19.27ms 461.91ms   99.05%
    Req/Sec     1.06k    82.76     1.47k    92.71%
  375767 requests in 30.06s, 94.25MB read
Requests/sec:  12498.49
Transfer/sec:      3.13MB
```

For reference I was trying to do this in python with FastAPI and was getting around 200ish request/sec
