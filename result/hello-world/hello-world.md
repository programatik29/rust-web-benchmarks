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
    1.49ms   3.13ms   0.04ms   61.62ms  
  Requests:
    Total: 10046862 Req/Sec: 335152.51
  Transfer:
    Total: 1.22 GB Transfer Rate: 41.55 MB/Sec
```

## Axum

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.79ms   1.69ms   0.03ms   202.05ms  
  Requests:
    Total: 8295218 Req/Sec: 276734.54
  Transfer:
    Total: 909.76 MB Transfer Rate: 30.35 MB/Sec
```

## Hyper

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.63ms   1.56ms   0.03ms   205.96ms  
  Requests:
    Total: 9156460 Req/Sec: 305494.56
  Transfer:
    Total: 777.17 MB Transfer Rate: 25.93 MB/Sec
```

## Poem

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.82ms   1.77ms   0.04ms   72.70ms  
  Requests:
    Total: 8242669 Req/Sec: 275005.05
  Transfer:
    Total: 903.99 MB Transfer Rate: 30.16 MB/Sec
```

## Rocket

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    2.83ms   2.29ms   0.05ms   75.97ms  
  Requests:
    Total: 5301129 Req/Sec: 176856.69
  Transfer:
    Total: 1.22 GB Transfer Rate: 41.83 MB/Sec
```

## Tide

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    4.03ms   11.60ms  0.05ms   86.87ms  
  Requests:
    Total: 3683696 Req/Sec: 122913.64
  Transfer:
    Total: 453.23 MB Transfer Rate: 15.12 MB/Sec
```

## Warp

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.67ms   1.62ms   0.03ms   72.00ms  
  Requests:
    Total: 8863526 Req/Sec: 295927.43
  Transfer:
    Total: 1.07 GB Transfer Rate: 36.69 MB/Sec
```
