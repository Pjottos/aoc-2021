# aoc-2021
Advent of code 2021 solutions in Rust.

### Running solutions
A nightly version of rust is required. Solutions can be run with:
```
cargo run --release --bin day<X>
```
where `X` is the day number.

- If you want to download inputs automatically you have to put your session cookie of AoC in a file named `session`. It should be in the format `session=<hex>`.
- Alternatively, you can create the files manually in the `inputs` folder, inputs for day `x` are read from `x.txt`.

To benchmark a solution, pass the `--bench` flag and optimize the binary for your specific CPU like this:
```
RUSTFLAGS="-C target-cpu=native" cargo run --release --bin day<X> -- --bench
```

### Benchmark results
These are the benchmark results on my PC. Keep in mind the time is measured from the moment the input text is in memory until the final answer is calculated.

| Day | Part 1     | Part 2     |
|-----|------------|------------|
| 1   | 22.393 µs  | 22.394 µs  |
| 2   | 24.492 µs  | 24.672 µs  |
| 3   | 479 ns     | 5.42 µs    |
| 4   | 37.051 µs  | 73.21 µs   |
| 5   | 332.156 µs | 471.851 µs |
| 6   | 226 ns     | 374 ns     |
| 7   | 21.255 µs  | 32.354 µs  |
| 8   | 11.348 µs  | 51.588 µs  |
| 10  | 8.297 µs   | 8.665 µs   |
| 11  | 5.941 µs   | 26.288 µs  |
