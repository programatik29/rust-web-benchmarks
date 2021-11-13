# Web Framework Benchmarks

Benchmarking web frameworks written in [rust] with [rewrk] tool.

## Benchmark Types

### Hello World

Respond "Hello, World!" to every request on "/" endpoint.

- [actix-web](benchmark/hello-world/actix-web/src/main.rs)
- [axum](benchmark/hello-world/axum/src/main.rs)
- [hyper](benchmark/hello-world/hyper/src/main.rs)

See [results](result/hello-world/hello-world.md).

[rewrk]: https://github.com/ChillFish8/rewrk
[rust]: https://github.com/rust-lang/rust
