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
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.56ms   3.17ms   0.04ms   99.92ms  
  Requests:
    Total: 9605731 Req/Sec: 320654.89
  Transfer:
    Total: 1.16 GB Transfer Rate: 39.75 MB/Sec
```

## Axum

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.83ms   1.77ms   0.03ms   211.36ms  
  Requests:
    Total: 8095701 Req/Sec: 270146.24
  Transfer:
    Total: 887.88 MB Transfer Rate: 29.63 MB/Sec
```

## Hyper

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.69ms   1.63ms   0.03ms   203.87ms  
  Requests:
    Total: 8796473 Req/Sec: 293359.90
  Transfer:
    Total: 746.62 MB Transfer Rate: 24.90 MB/Sec
```

## Rocket

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    2.91ms   2.30ms   0.05ms   71.96ms  
  Requests:
    Total: 5144448 Req/Sec: 171646.69
  Transfer:
    Total: 1.19 GB Transfer Rate: 40.60 MB/Sec
```

## Tide

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    4.15ms   11.74ms  0.04ms   64.92ms  
  Requests:
    Total: 3573597 Req/Sec: 119222.47
  Transfer:
    Total: 439.69 MB Transfer Rate: 14.67 MB/Sec
```

## Warp

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.75ms   1.67ms   0.03ms   86.02ms  
  Requests:
    Total: 8485373 Req/Sec: 283105.89
  Transfer:
    Total: 1.03 GB Transfer Rate: 35.10 MB/Sec
```
