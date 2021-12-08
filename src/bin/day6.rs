use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(6)
        .extract(|text| {
            let mut bins = [0; 9];

            for p in text.split(',') {
                let idx: usize = p.trim().parse().unwrap();
                bins[idx] += 1;
            }

            bins
        })
        // .input_override("3,4,3,1,2")
        .run_part(1, |&bins| run_days(bins, 80))
        .run_part(2, |&bins| run_days(bins, 256));
}

fn run_days(mut bins: [u64; 9], days: usize) -> u64 {
    for day in 0..days {
        bins[(day + 7) % 9] += bins[day % 9];
    }

    bins.iter().sum()
}
