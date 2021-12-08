use aoc_2021::*;

const BIT_COUNT: usize = 12;

fn main() {
    Harness::begin()
        .day(3)
        // .input_override("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n")
        .extract(|text| {
            text.as_bytes()
                .chunks_exact(BIT_COUNT + 1)
                .map(|b| &b[..BIT_COUNT])
        })
        .run_part(1, |lines| {
            let mut counts = [0; BIT_COUNT];
            let mut line_count = 0;
            for line in lines.clone() {
                line.iter()
                    .zip(counts.iter_mut())
                    .for_each(|(v, c)| *c += (v - b'0') as u32);
                line_count += 1;
            }

            let mut gamma = 0;
            for (i, count) in counts.iter().rev().copied().enumerate() {
                let bit = (count * 2 >= line_count) as u32;
                gamma |= bit << i;
            }
            let epsilon = !gamma & !(!0 << BIT_COUNT);

            // println!("g: {:12.b}\ne: {:12.b}", gamma, epsilon);

            gamma * epsilon
        })
        .run_part(2, |lines| {
            let mut oxygen: Vec<_> = lines.clone().collect();
            let mut co2 = oxygen.clone();
            for i in 0..BIT_COUNT {
                // println!(
                //     "m: {:12.b}, o: {:4}, c: {:4}",
                //     mask,
                //     oxygen.len(),
                //     co2.len()
                // );
                if oxygen.len() > 1 {
                    retain_common_bit_lines(&mut oxygen, i, false);
                }
                if co2.len() > 1 {
                    retain_common_bit_lines(&mut co2, i, true);
                }
            }

            binary_to_num(oxygen[0]) * binary_to_num(co2[0])
        });
}

fn retain_common_bit_lines(lines: &mut Vec<&[u8]>, bit_idx: usize, invert: bool) {
    // nums.iter().for_each(|n| println!("o: {:12.b}", n));
    // println!();

    let count: u32 = lines.iter().map(|l| (l[bit_idx] - b'0') as u32).sum();
    let allowed_value = (b'0' + (count * 2 >= lines.len() as u32) as u8) ^ invert as u8;

    let mut i = 0;
    while i < lines.len() {
        if lines[i][bit_idx] == allowed_value {
            i += 1;
        } else {
            lines.swap_remove(i);
        }
    }
}

fn binary_to_num(binary: &[u8]) -> u32 {
    let mut result = 0;

    for (i, c) in binary.iter().rev().copied().enumerate() {
        let bit = (c - b'0') as u32;
        result |= bit << i;
    }

    result
}
