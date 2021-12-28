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
    1.49ms   3.15ms   0.04ms   101.22ms  
  Requests:
    Total: 10038182 Req/Sec: 335073.93
  Transfer:
    Total: 1.22 GB Transfer Rate: 41.54 MB/Sec
```

## Astra

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.09ms   0.75ms   0.02ms   98.08ms  
  Requests:
    Total: 4929262 Req/Sec: 164461.85
  Transfer:
    Total: 503.01 MB Transfer Rate: 16.78 MB/Sec
```

## Axum

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.81ms   1.77ms   0.04ms   67.54ms  
  Requests:
    Total: 8237620 Req/Sec: 274736.26
  Transfer:
    Total: 903.44 MB Transfer Rate: 30.13 MB/Sec
```

## Hyper

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.67ms   1.58ms   0.03ms   202.60ms  
  Requests:
    Total: 8925616 Req/Sec: 297808.15
  Transfer:
    Total: 757.58 MB Transfer Rate: 25.28 MB/Sec
```

## Poem

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.84ms   1.69ms   0.04ms   66.13ms  
  Requests:
    Total: 8133381 Req/Sec: 271318.17
  Transfer:
    Total: 892.01 MB Transfer Rate: 29.76 MB/Sec
```

## Rocket

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    2.83ms   2.23ms   0.06ms   75.67ms  
  Requests:
    Total: 5295914 Req/Sec: 176673.33
  Transfer:
    Total: 1.22 GB Transfer Rate: 41.79 MB/Sec
```

## Tide

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    4.05ms   11.64ms  0.04ms   208.19ms  
  Requests:
    Total: 3676607 Req/Sec: 122663.80
  Transfer:
    Total: 452.36 MB Transfer Rate: 15.09 MB/Sec
```

## Warp

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 30 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.70ms   1.62ms   0.04ms   103.57ms  
  Requests:
    Total: 8746555 Req/Sec: 291842.38
  Transfer:
    Total: 1.06 GB Transfer Rate: 36.18 MB/Sec
```
