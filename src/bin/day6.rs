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
    for _ in 0..days {
        // amount of new born fish this day, same number as the amount of parents
        let new_born = bins[0];
        // decrease the timer on all fish, parents of this day are overwritten which is why we saved
        // the amount of parents.
        for i in 0..bins.len() - 1 {
            bins[i] = bins[i + 1];
        }
        // give birth to the fish, overwriting because the fish that were in this bin are now in 7.
        bins[8] = new_born;
        // reset the timer on the parents.
        bins[6] += new_born;
    }

    bins.iter().sum::<u64>() as i64
}
