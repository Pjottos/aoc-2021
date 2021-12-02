use aoc_2021::{ex, input};

fn main() {
    let nums = input(1, &ex::Numbers);

    println!("part 1: {:#?}", {
        nums.windows(2).filter(|w| w[1] > w[0]).count()
    });

    println!("part 2: {:#?}", {
        nums.windows(4).filter(|w| w[3] > w[0]).count()
    });
}
