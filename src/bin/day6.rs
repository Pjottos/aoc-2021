use aoc_2021::*;

pub struct FishBins;

impl InputExtractor for FishBins {
    type Output = [u64; 9];

    fn extract(&self, text: &str) -> Self::Output {
        let mut bins = [0; 9];

        for p in text.split(',') {
            let idx: usize = p.trim().parse().unwrap();
            bins[idx] += 1;
        }

        bins
    }
}

fn main() {
    Harness::builder()
        .day(6)
        .extractor(FishBins)
        // .input_override("3,4,3,1,2")
        .part_1(|&bins| run_days(bins, 80))
        .part_2(|&bins| run_days(bins, 256))
        .run();
}

fn run_days(mut bins: [u64; 9], days: usize) -> i64 {
    for day in 0..days {
        bins[(day + 7) % 9] += bins[day % 9];
    }

    bins.iter().sum::<u64>() as i64
}
