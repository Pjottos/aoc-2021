use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(1)
        .extract(|text| {
            text.lines()
                .map(|l| l.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .run_part(1, |nums| nums.windows(2).filter(|w| w[1] > w[0]).count())
        .run_part(2, |nums| nums.windows(4).filter(|w| w[3] > w[0]).count());
}
