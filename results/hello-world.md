# Hardware

## CPU

Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz

Physical cores: 6

Logical cores: 12

## RAM

Type: DDR4

Size: 2 x 8 GB

Speed: 2667 MT/s

# Results

Command for all tests:

```
./rewrk -t 12 -c 500 -d 30s -h http://localhost:3000/
```

## hyper

```
Benchmarking 500 connections @ http://localhost:3000/ for 30 seconds
  Latencies:
    Avg      Stdev    Min      Max
    1.26ms   0.61ms   0.02ms   32.44ms
  Requests:
    Total: 11816709 Req/Sec: 393881.24
  Transfer:
    Total: 991.70 MB Transfer Rate: 33.06 MB/Sec
```

## axum

```
Benchmarking 500 connections @ http://localhost:3000/ for 30 seconds
  Latencies:
    Avg      Stdev    Min      Max
    1.39ms   0.72ms   0.03ms   67.02ms
  Requests:
    Total: 10687977 Req/Sec: 356258.15
  Transfer:
    Total: 907.16 MB Transfer Rate: 30.24 MB/Sec
```

## actix-web

```
Benchmarking 500 connections @ http://localhost:3000/ for 30 seconds
  Latencies:
    Avg      Stdev    Min      Max
    1.22ms   0.74ms   0.02ms   65.33ms
  Requests:
    Total: 12068603 Req/Sec: 402278.83
  Transfer:
    Total: 1.46 GB Transfer Rate: 49.87 MB/Sec
```

# Observed Memory Usage

`hyper`: 8 MB

`axum`: 10 MB

`actix-web`: 21 MB
