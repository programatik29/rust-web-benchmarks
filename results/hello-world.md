```
# In "frameworks/" tab
cargo run --release --bin=hyper-hello-world

# In "rewrk/" tab
echo "Results for hyper:" && ./rewrk -t 12 -c 500 -d 30s -h http://localhost:3000/
Results for hyper:
Benchmarking 500 connections @ http://localhost:3000/ for 30 seconds
  Latencies:
    Avg      Stdev    Min      Max
    1.26ms   0.61ms   0.02ms   32.44ms
  Requests:
    Total: 11816709 Req/Sec: 393881.24
  Transfer:
    Total: 991.70 MB Transfer Rate: 33.06 MB/Sec

# In "frameworks/" tab
cargo run --release --bin=axum-hello-world

# In "rewrk/" tab
echo "Results for axum:" && ./rewrk -t 12 -c 500 -d 30s -h http://localhost:3000/
Results for axum:
Benchmarking 500 connections @ http://localhost:3000/ for 30 seconds
  Latencies:
    Avg      Stdev    Min      Max
    1.39ms   0.72ms   0.03ms   67.02ms
  Requests:
    Total: 10687977 Req/Sec: 356258.15
  Transfer:
    Total: 907.16 MB Transfer Rate: 30.24 MB/Sec
```
