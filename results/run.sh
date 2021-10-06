#!/bin/bash
echo "" > results
(
cd ../frameworks
cargo update
cargo build --release
sleep 2
cargo run -q --release --bin=hyper-hello-world &
echo "Hyper - Hello World:"; rewrk -t 12 -c 500 -d 10s -h http://localhost:3000
kill $(jobs -p)
sleep 2
cargo run -q --release --bin=axum-hello-world &
echo "Axum - Hello World:"; rewrk -t 12 -c 500 -d 10s -h http://localhost:3000
kill $(jobs -p)
sleep 2
cargo run -q --release --bin=actix-web-hello-world &
echo "Actix Web - Hello World ST:"; rewrk -t 12 -c 500 -d 10s -h http://localhost:3000
kill $(jobs -p)
sleep 2
cargo run -q --release --bin=hyper-hello-world-st &
echo "Hyper - Hello World ST:"; rewrk -t 12 -c 500 -d 10s -h http://localhost:3000
kill $(jobs -p)
sleep 2
cargo run -q --release --bin=axum-hello-world-st &
echo "Axum - Hello World ST:"; rewrk -t 12 -c 500 -d 10s -h http://localhost:3000
kill $(jobs -p)
)\
| tee -a results
