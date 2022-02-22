# note: "rewrk" needs to be in $PATH
# used rewrk: https://github.com/programatik29/rewrk/tree/single-thread
trap "kill 0" SIGINT
export bench_cmd="rewrk -t 12 -c 500 -d 30s -h http://127.0.0.1:3000"
cd ../../benchmark
cargo build --release --bin hello-world-actix-web
cargo build --release --bin hello-world-astra
cargo build --release --bin hello-world-axum
cargo build --release --bin hello-world-hyper
cargo build --release --bin hello-world-poem
cargo +nightly build --release --bin hello-world-rocket
cargo build --release --bin hello-world-thruster
cargo build --release --bin hello-world-tide
cargo build --release --bin hello-world-warp
(
# actix-web
echo "Actix Web:"
cargo run -q --release --bin hello-world-actix-web &
sleep 1
eval $bench_cmd
kill $!

# astra
echo "Astra:"
cargo run -q --release --bin hello-world-astra &
sleep 2
eval $bench_cmd
kill $!

# axum
echo "Axum:"
cargo run -q --release --bin hello-world-axum &
sleep 1
eval $bench_cmd
kill $!

# hyper
echo "Hyper:"
cargo run -q --release --bin hello-world-hyper &
sleep 1
eval $bench_cmd
kill $!

# poem
echo "Poem:"
cargo run -q --release --bin hello-world-poem &
sleep 1
eval $bench_cmd
kill $!

# rocket
echo "Rocket:"
ROCKET_ENV=prod ROCKET_PORT=3000 cargo +nightly run -q --release --bin hello-world-rocket &
sleep 1
eval $bench_cmd
kill $!

# thruster
echo "Thruster:"
cargo run -q --release --bin hello-world-thruster &
sleep 1
eval $bench_cmd
kill $!

# tide
echo "Tide:"
cargo run -q --release --bin hello-world-tide &
sleep 1
eval $bench_cmd
kill $!

# warp
echo "Warp:"
cargo run -q --release --bin hello-world-warp &
sleep 1
eval $bench_cmd
kill $!
) | tee ../result/hello-world/hello-world.stdout
