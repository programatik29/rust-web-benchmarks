# note: "rewrk" needs to be in $PATH
# used rewrk: https://github.com/programatik29/rewrk/tree/single-thread
trap "kill 0" SIGINT
cd ../../benchmark
cargo build --release --bin hello-world-actix-web
cargo build --release --bin hello-world-axum
cargo build --release --bin hello-world-hyper
cargo +nightly build --release --bin hello-world-rocket
(
# actix-web
echo "Actix Web:"
cargo run -q --release --bin hello-world-actix-web &
sleep 1
rewrk -t 12 -c 500 -d 10s -h http://127.0.0.1:3000
kill $!
# axum
echo "Axum:"
cargo run -q --release --bin hello-world-axum &
sleep 1
rewrk -t 12 -c 500 -d 10s -h http://127.0.0.1:3000
kill $!
# hyper
echo "Hyper:"
cargo run -q --release --bin hello-world-hyper &
sleep 1
rewrk -t 12 -c 500 -d 10s -h http://127.0.0.1:3000
kill $!
# rocket
echo "Rocket:"
cargo +nightly run -q --release --bin hello-world-rocket &
sleep 1
rewrk -t 12 -c 500 -d 10s -h http://127.0.0.1:8000
kill $!
) | tee ../result/hello-world/hello-world.stdout
