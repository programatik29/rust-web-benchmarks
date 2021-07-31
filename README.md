# Web Framework Benchmarks

Benchmarking web frameworks written in [rust](https://github.com/rust-lang/rust) with [rewrk](https://github.com/programatik29/rewrk) tool.

# Bench

Clone and compile [rewrk](https://github.com/programatik29/rewrk):

```
git clone https://github.com/programatik29/rewrk
cd rewrk/
cargo build --release
cd target/release/
```

Open a new tab in `rust-web-benchmarks` directory, compile and run desired server. Example:

```
cd frameworks/

cargo run --release --bin=hyper-hello-world

# In rewrk tab
echo "Results for hyper:" && ./rewrk -t 12 -c 500 -d 10s -h http://localhost:3000/

# Back to frameworks tab
cargo run --release --bin=axum-hello-world

# In rewrk tab
echo "Results for axum:" && ./rewrk -t 12 -c 500 -d 10s -h http://localhost:3000/
```

## Benchmark Types

### Hello World

Server responds `Hello, world!` to every request.

Available low-level frameworks:

- [hyper](https://github.com/hyperium/hyper), view [code](frameworks/hyper-hello-world/src/main.rs)

Available high-level frameworks:

- [axum](https://github.com/tokio-rs/axum), view [code](frameworks/axum-hello-world/src/main.rs)

See [results](results/hello-world.md).
