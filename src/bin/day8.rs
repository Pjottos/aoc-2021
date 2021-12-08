use aoc_2021::*;

struct Notes;

impl InputExtractor for Notes {
    type Output = Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines()
            .map(|l| {
                let mut parts = l.split('|');
                let combos = parts
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|w| w.as_bytes().to_vec())
                    .collect();
                let output = parts
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|w| w.as_bytes().to_vec())
                    .collect();
                (combos, output)
            })
            .collect()
    }
}

fn main() {
    let unique = vec![(2, 1), (4, 4), (3, 7), (7, 8)];

    Harness::builder()
        .day(8)
        .extractor(Notes)
        .part_1(|lines| {
            lines
                .iter()
                .map(|(_, output)| {
                    output
                        .iter()
                        .filter(|w| unique.iter().position(|(l, _)| *l == w.len()).is_some())
                        .count() as i64
                })
                .sum()
        })
        .part_2(|lines| {
            lines
                .iter()
                .map(|(combos, encoded)| {
                    let mut digits = vec![];

                    let mut one = None;
                    let mut four = None;
                    let remaining: Vec<_> = combos
                        .iter()
                        .filter(|c| {
                            let bits = digit_bits(c);
                            unique
                                .iter()
                                .copied()
                                .find(|(l, _)| *l == c.len())
                                .map(|(_, v)| {
                                    if v == 1 {
                                        one = Some(bits);
                                    } else if v == 4 {
                                        four = Some(bits);
                                    }

                                    digits.push((bits, v));
                                })
                                .is_none()
                        })
                        .collect();

                    let one = one.unwrap();
                    let four = four.unwrap();
                    for r in remaining {
                        let bits = digit_bits(r);
                        let digit = if r.len() == 5 {
                            if bits & one == one {
                                3
                            } else {
                                if (bits & four).count_ones() == 2 {
                                    2
                                } else {
                                    5
                                }
                            }
                        } else {
                            if bits & one == one {
                                if bits & four == four {
                                    9
                                } else {
                                    0
                                }
                            } else {
                                6
                            }
                        };

                        digits.push((bits, digit));
                    }

                    encoded.iter().enumerate().fold(0, |num, (i, o)| {
                        let bits = digit_bits(o);
                        let digit = digits.iter().find(|(d, _)| *d == bits).unwrap().1;

                        num + digit * 10i64.pow(3 - i as u32)
                    })
                })
                .sum()
        })
        .run();
}

fn digit_bits(chars: &[u8]) -> u8 {
    chars.iter().fold(0, |result, &c| {
        let bit = 1 << (c - 'a' as u8);
        result | bit
    })
}
