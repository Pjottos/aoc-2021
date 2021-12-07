use aoc_2021::*;

fn main() {
    Harness::builder()
        .day(1)
        .extractor(ex::Numbers)
        .part_1(|nums| nums.windows(2).filter(|w| w[1] > w[0]).count() as i64)
        .part_2(|nums| nums.windows(4).filter(|w| w[3] > w[0]).count() as i64)
        .run();
}
