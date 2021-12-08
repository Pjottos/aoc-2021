use aoc_2021::*;

fn main() {
    let unique = [(2, 1), (4, 4), (3, 7), (7, 8)];

    Harness::begin()
        .day(8)
        .extract(|text| {
            text.lines().map(|l| {
                let mut parts = l.split('|');
                let combos = parts
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|w| digit_bits(w.as_bytes()));
                let output = parts
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|w| digit_bits(w.as_bytes()));

                (combos, output)
            })
        })
        .run_part(1, |lines| {
            lines
                .clone()
                .map(|(_, output)| {
                    output
                        .filter(|w| unique.iter().any(|(l, _)| *l as u32 == w.count_ones()))
                        .count() as u32
                })
                .sum::<u32>()
        })
        .run_part(2, |lines| {
            lines
                .clone()
                .map(|(combos, encoded)| {
                    let mut digits = [0; 10];

                    for c in combos.clone() {
                        if let Some((_, v)) =
                            unique.iter().find(|(l, _)| *l as u32 == c.count_ones())
                        {
                            digits[*v] = c;
                        }
                    }

                    // all unique digits should have been found
                    assert!(unique.iter().all(|(_, v)| digits[*v] != 0));

                    for r in
                        combos.filter(|&c| !unique.iter().any(|(l, _)| *l as u32 == c.count_ones()))
                    {
                        let digit = if r.count_ones() == 5 {
                            if r & digits[1] == digits[1] {
                                3
                            } else if (r & digits[4]).count_ones() == 2 {
                                2
                            } else {
                                5
                            }
                        } else if r & digits[1] == digits[1] {
                            if r & digits[4] == digits[4] {
                                9
                            } else {
                                0
                            }
                        } else {
                            6
                        };

                        digits[digit] = r;
                    }

                    encoded.enumerate().fold(0, |num, (i, o)| {
                        let digit = digits.iter().position(|&b| b == o).unwrap() as u32;

                        num + digit * 10u32.pow(3 - i as u32)
                    })
                })
                .sum::<u32>()
        });
}

fn digit_bits(chars: &[u8]) -> u8 {
    chars.iter().fold(0, |result, &c| {
        let bit = 1 << (c - b'a');
        result | bit
    })
}
