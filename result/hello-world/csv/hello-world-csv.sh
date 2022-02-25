# note: "rewrk" needs to be in $PATH
# used rewrk: https://github.com/programatik29/rewrk/tree/single-thread
# note: this csv version of the benchmarks also need "pq" and "csvkit" to be installed. They are available on mac homebrew.
trap "kill 0" SIGINT
export bench_cmd="rewrk --json -t 12 -c 500 -d 30s -h http://127.0.0.1:3000
| jq -r 'def roundit: .*100.0|round/100.0;
    [.latency_avg,.latency_max,.latency_min,.latency_std_deviation,.requests_avg,.requests_total,.transfer_rate,.transfer_total|roundit] | @csv'"
cd ../../../benchmark
cargo build --release --bin hello-world-actix-web
cargo build --release --bin hello-world-astra
cargo build --release --bin hello-world-axum
cargo build --release --bin hello-world-hyper
cargo build --release --bin hello-world-poem
cargo +nightly build --release --bin hello-world-rocket
cargo build --release --bin hello-world-tide
cargo build --release --bin hello-world-warp
(
echo "framework,latency_avg,latency_max,latency_min,latency_std_deviation,requests_avg,requests_total,transfer_rate,transfer_total"
# actix-web
cargo run -q --release --bin hello-world-actix-web &
sleep 1
eval $bench_cmd | echo "Actix Web,$(cat -)"
kill $!

# astra
cargo run -q --release --bin hello-world-astra &
sleep 2
eval $bench_cmd | echo "Astra,$(cat -)"
kill $!

# axum
cargo run -q --release --bin hello-world-axum &
sleep 1
eval $bench_cmd | echo "Axum,$(cat -)"
kill $!

# hyper
cargo run -q --release --bin hello-world-hyper &
sleep 1
eval $bench_cmd | echo "Hyper,$(cat -)"
kill $!

# poem
cargo run -q --release --bin hello-world-poem &
sleep 1
eval $bench_cmd | echo "Poem,$(cat -)"
kill $!

# rocket
ROCKET_ENV=prod ROCKET_PORT=3000 cargo +nightly run -q --release --bin hello-world-rocket > /dev/null &
sleep 1
eval $bench_cmd | echo "Rocket,$(cat -)"
kill $!

# tide
cargo run -q --release --bin hello-world-tide &
sleep 1
eval $bench_cmd | echo "Tide,$(cat -)"
kill $!

# warp
cargo run -q --release --bin hello-world-warp &
sleep 1
eval $bench_cmd | echo "Warp,$(cat -)"
kill $!
) | tee ../result/hello-world/csv/hello-world-csv.stdout
cd ../result/hello-world/csv
csvlook hello-world-csv.stdout | echo "\`\`\`\n$(cat -)\n\`\`\`" | tee hello-world-csv.md
