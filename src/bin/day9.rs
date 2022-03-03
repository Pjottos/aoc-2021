#![feature(portable_simd)]

use aoc_2021::*;

use std::simd::u8x32;

fn main() {
    Harness::begin()
        .day(9)
        // .input_override("2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n")
        .extract(|text| {
            let grid = text.as_bytes();

            // assert!(grid.iter().all(|&c| (c >= b'0' && c <= b'9') || c == b'\n'));
            let row_size = grid.iter().position(|&c| c == b'\n').unwrap() + 1;
            // assert!(grid
            //     .chunks(row_size)
            //     .all(|chunk| *chunk.last().unwrap() == b'\n'));

            (grid, row_size)
        })
        .run_part(1, |(grid, row_size)| {
            let padded_read = |i: usize, j: usize| -> u8x32 {
                // Compensate for taking one less when left-padding
                let start = i * row_size + j * 30 - j.min(1);
                let end = (start + 32).min((i + 1) * row_size - 1);

                let mut buf = u8x32::splat(b'9');
                let pad_left = (j == 0) as usize;
                buf.as_mut_array()[pad_left..end - start]
                    .copy_from_slice(&grid[start..end - pad_left]);
                buf
            };

            let mut low_point_count = 0;

            let row_count = grid.len() / row_size;
            for i in 0..row_count {
                for j in 0..row_size / 30 + (row_size % 30).min(1) {
                    let base = padded_read(i, j);
                    let left = base.rotate_lanes_right::<1>();
                    let right = base.rotate_lanes_left::<1>();
                    let up = if i == 0 {
                        u8x32::splat(b'9')
                    } else {
                        padded_read(i - 1, j)
                    };
                    let down = if i == row_count - 1 {
                        u8x32::splat(b'9')
                    } else {
                        padded_read(i + 1, j)
                    };

                    // Create a mask of elements in the base lane that are surrounded by higher value
                    // elements in the grid.
                    let mut mask = base.lanes_lt(left)
                        & base.lanes_lt(right)
                        & base.lanes_lt(up)
                        & base.lanes_lt(down);

                    // Ignore padding lanes
                    mask.set(0, false);
                    mask.set(31, false);

                    // Bring ASCII values in range 0..=9 and add 1
                    let true_values = base - u8x32::splat(b'0' - 1);
                    let scores = mask.select(true_values, u8x32::splat(0));
                    // The horizontal sum cannot overflow because the max score per cell is 9,
                    // there are 30 cells (excluding padding) but it's only possible for a cell to have
                    // a nonzero score if the cells to the left and right of it have a score of 0. Which
                    // brings the maximum value of the horizontal sum to 15 * 9 = 135
                    low_point_count += scores.horizontal_sum() as u32;

                    // if mask.any() {
                    //     println!("left-base-right");
                    //     println!("{:?}", left);
                    //     println!("{:?}", base);
                    //     println!("{:?}", right);
                    //     println!("up-base-down");
                    //     println!("{:?}", up);
                    //     println!("{:?}", base);
                    //     println!("{:?}", down);
                    //     println!("{:2.?}", scores);
                    //     println!();
                    //     println!();

                    //     let up = &grid[(i - 1) * row_size + j * 30..(i - 1) * row_size + (j + 1) * 30];
                    //     let base = &grid[i * row_size + j * 30..i * row_size + (j + 1) * 30];
                    //     let down = &grid[(i + 1) * row_size + j * 30..(i + 1) * row_size + (j + 1) * 30];

                    //     for (s, score) in scores.as_array().iter().skip(1).take(31).enumerate().filter(|(_, score)| **score != 0) {
                    //         if up[s] < base[s]
                    //             || down[s] < base[s]
                    //             || s.checked_sub(1).map_or(b'9', |l| base[l]) < base[s]
                    //             || base.get(s + 1).map_or(false, |&r| r < base[s])
                    //         {
                    //             println!("{i} {j}");
                    //             println!("    {:?}", up);
                    //             println!("    {:?}", base);
                    //             println!("    {:?}", down);
                    //             println!("{:2.?}", scores);
                    //             println!();
                    //         }
                    //     }
                    // }
                }
            }

            low_point_count
        })
        .run_part(2, |(_grid, _row_size)| {});
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
