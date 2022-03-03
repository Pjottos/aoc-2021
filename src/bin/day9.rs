use aoc_2021::*;

use std::collections::VecDeque;

fn main() {
    Harness::begin()
        .day(9)
        // .input_override("2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n")
        .extract(|text| text)
        .run_part(1, |text| {
            let mut window = VecDeque::new();
            let mut risk_level = 0;

            for (i, line) in text.lines().map(|line| line.as_bytes()).enumerate() {
                window.push_back(line);

                if i == 1 {
                    risk_level += count_edges(window[0], window[1], None);
                    window[0]
                        .windows(3)
                        .zip(window[1].windows(3))
                        .for_each(|(r0, r1)| {
                            let same_row = r0[1] < r0[0] && r0[1] < r0[2];
                            let below = r0[1] < r1[1];

                            if same_row && below {
                                let v = u32::from(r0[1] - b'0');
                                risk_level += 1 + v;
                            }
                        })
                }

                if window.len() == 4 {
                    window.pop_front();
                }

                if window.len() != 3 {
                    continue;
                } else {
                    risk_level += count_edges(window[1], window[0], Some(window[2]));
                    window[0]
                        .windows(3)
                        .zip(window[1].windows(3))
                        .zip(window[2].windows(3))
                        .for_each(|((r0, r1), r2)| {
                            let above = r1[1] < r0[1];
                            let same_row = r1[1] < r1[0] && r1[1] < r1[2];
                            let below = r1[1] < r2[1];

                            if above && same_row && below {
                                let v = u32::from(r1[1] - b'0');
                                risk_level += 1 + v;
                            }
                        });
                };
            }

            // Make sure we don't skip the last row.
            risk_level += count_edges(window[2], window[1], None);
            window[1]
                .windows(3)
                .zip(window[2].windows(3))
                .for_each(|(r1, r2)| {
                    let above = r2[1] < r1[1];
                    let same_row = r2[1] < r2[0] && r2[1] < r2[2];

                    if above && same_row {
                        let v = u32::from(r2[1] - b'0');
                        risk_level += 1 + v;
                    }
                });

            risk_level
        })
        .run_part(2, |_text| {});
}

fn count_edges(target_line: &[u8], adjacent_one: &[u8], adjacent_two: Option<&[u8]>) -> u32 {
    let first = target_line[0];
    let row_len = target_line.len();
    let last = target_line[row_len - 1];

    if first < target_line[1]
        && first < adjacent_one[0]
        && adjacent_two.map_or(true, |l| first < l[0])
    {
        1 + u32::from(first - b'0')
    } else if last < target_line[row_len - 2]
        && last < adjacent_one[row_len - 1]
        && adjacent_two.map_or(true, |l| last < l[row_len - 1])
    {
        1 + u32::from(last - b'0')
    } else {
        0
    }
}

#[allow(dead_code)]
fn print_basin_grid(grid: &[u8], row_size: usize) {
    for line in grid.chunks_exact(row_size) {
        let line: String = line[..row_size - 1]
            .iter()
            .map(|&c| if c == b'9' { ' ' } else { c as char })
            .collect();
        println!("{}", line);
    }
}
