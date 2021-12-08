use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(6)
        .extract(|text| {
            // Instead of keeping track of each fish individually, we group them by their
            // birth timer values.
            let mut bins = [0; 9];

            // Parsing like this is ~5x faster than splitting on `,` and parsing a usize.
            for chunk in text.as_bytes().chunks_exact(2) {
                let idx = match chunk[0] {
                    c @ b'0'..=b'8' => (c - b'0') as usize,
                    _ => panic!("fish age out of range"),
                };
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
        // This line does 2 things:
        // - It adds the parents of the current day to the bin that contains the
        //   parents for 7 days into the future, effectively resetting their birth timer.
        // - By not clearing the current bin, the newborn fish will be in
        //   the last bin on the next day.
        bins[(day + 7) % 9] += bins[day % 9];
    }

    // Return total amount of fish.
    bins.iter().sum()
}
