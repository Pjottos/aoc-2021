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
    let mut bins = input(6, &FishBins);
    // let mut bins = FishBins.extract("3,4,3,1,2");

    println!("part 1: {:#?}", run_days(&mut bins, 80));

    println!("part 2: {:#?}", run_days(&mut bins, 256 - 80));
}

fn run_days(bins: &mut [u64; 9], days: usize) -> u64 {
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

    bins.iter().sum()
}
