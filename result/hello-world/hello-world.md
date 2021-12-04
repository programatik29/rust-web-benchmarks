# Benchmarked On

## CPU

Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz

Physical cores: 6

Logical cores: 12

## RAM

Type: DDR4

Size: 2 x 8 GB

Speed: 2667 MT/s

# Command

Executed [`hello-world.sh`](hello-world.sh) script.

# Frameworks

## Actix Web

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.51ms   3.18ms   0.04ms   115.76ms  
  Requests:
    Total: 3296342 Req/Sec: 331016.23
  Transfer:
    Total: 408.67 MB Transfer Rate: 41.04 MB/Sec
```

## Axum

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.76ms   1.86ms   0.04ms   113.10ms  
  Requests:
    Total: 2780321 Req/Sec: 279203.23
  Transfer:
    Total: 304.92 MB Transfer Rate: 30.62 MB/Sec
```

## Hyper

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.63ms   1.68ms   0.03ms   206.58ms  
  Requests:
    Total: 3003219 Req/Sec: 301233.03
  Transfer:
    Total: 254.90 MB Transfer Rate: 25.57 MB/Sec
```

## Rocket

```
Beginning round 1...
Benchmarking 500 connections @ http://127.0.0.1:8000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    2.79ms   2.23ms   0.05ms   43.82ms  
  Requests:
    Total: 1790261 Req/Sec: 179421.99
  Transfer:
    Total: 423.42 MB Transfer Rate: 42.44 MB/Sec
```
