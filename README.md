# aoc-2021
Advent of code 2021 solutions in Rust.

## Running solutions
Solutions can be run with:
```
cargo run --release --bin day<X>
```
where `X` is the day number.

- If you want to download inputs automatically you have to put your session cookie of AoC in a file with name `session`. It should be in the format `session=<hex>`.
- Alternatively, you can create the files manually in the `inputs` folder, inputs for day `x` are read from `x.txt`.

To benchmark a solution, pass the `--bench` flag like this:
```
cargo run --release --bin day<X> -- --bench
```
