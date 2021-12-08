use aoc_2021::*;

const BIT_COUNT: usize = 12;

fn main() {
    Harness::begin()
        .day(3)
        .extract(ex::binary_nums)
        // .input_override("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n")
        .run_part(1, |nums| {
            let gamma = make_common_mask(nums);
            let epsilon = !gamma & !(!0 << BIT_COUNT);

            // println!("g: {:12.b}\ne: {:12.b}", gamma, epsilon);

            gamma * epsilon
        })
        .run_part(2, |nums| {
            let mut oxygen = nums.clone();
            let mut co2 = nums.clone();
            for i in 0..BIT_COUNT {
                let mask = 1 << (BIT_COUNT - 1 - i);

                // println!(
                //     "m: {:12.b}, o: {:4}, c: {:4}",
                //     mask,
                //     oxygen.len(),
                //     co2.len()
                // );
                if oxygen.len() > 1 {
                    let common_mask = make_single_common_mask(&oxygen, i);
                    oxygen.retain(|n| n & mask == common_mask);
                }
                if co2.len() > 1 {
                    let common_mask = make_single_common_mask(&co2, i);
                    co2.retain(|n| !n & mask == common_mask);
                }
            }

            oxygen[0] * co2[0]
        });
}

fn make_common_mask(nums: &[u64]) -> u64 {
    let mut counts = [0; BIT_COUNT];
    for num in nums {
        for (i, c) in counts.iter_mut().enumerate() {
            *c += (num >> i) & 1;
        }
    }
    // println!("{:?}", counts);

    let mut common_mask = 0;
    for (i, count) in counts.iter().copied().enumerate() {
        let bit = (count * 2 >= nums.len() as u64) as u64;
        common_mask |= bit << i;
    }

    common_mask
}

fn make_single_common_mask(nums: &[u64], pos: usize) -> u64 {
    // nums.iter().for_each(|n| println!("o: {:12.b}", n));
    // println!();

    let shift = BIT_COUNT - 1 - pos;
    let count: u64 = nums.iter().map(|n| (n >> shift) & 1).sum();

    let bit = (count * 2 >= nums.len() as u64) as u64;
    bit << shift
}
