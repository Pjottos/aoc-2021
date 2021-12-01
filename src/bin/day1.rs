use aoc_2021::{ex, input};

fn main() {
    let nums = input::<ex::Numbers>(1);

    let one = nums.windows(2).filter(|w| w[1] > w[0]).count();
    println!("part 1: {:#?}", one);

    let two = nums.windows(4).filter(|w| w[3] > w[0]).count();
    println!("part 2: {:#?}", two);
}
