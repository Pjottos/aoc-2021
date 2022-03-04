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
            let row_count = grid.len() / row_size;
            // assert!(grid
            //     .chunks(row_size)
            //     .all(|chunk| *chunk.last().unwrap() == b'\n'));

            (grid, row_size, row_count)
        })
        .run_part(1, |&(grid, row_size, row_count)| {
            let read_chunk = |i: usize, j: usize, pad_left: bool, pad_right: bool| -> u8x32 {
                let start = i * row_size + j * 30 - (!pad_left) as usize;
                let end = if pad_right {
                    (i + 1) * row_size - 1
                } else {
                    start + 32
                };

                // PERF: splats seem to be implemented as a memory load every time,
                // maybe there's a way to convince the compiler to load it once and keep it in
                // a register?
                let mut buf = if pad_left || pad_right {
                    u8x32::splat(b'9')
                } else {
                    u8x32::splat(0)
                };

                buf[pad_left as usize..end - start]
                    .copy_from_slice(&grid[start..end - pad_left as usize]);
                buf
            };

            #[inline]
            fn count_low_points(base: u8x32, up: u8x32, down: u8x32) -> u32 {
                let left = base.rotate_lanes_right::<1>();
                let right = base.rotate_lanes_left::<1>();

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
                u32::from(scores.horizontal_sum())
            }

            #[inline]
            fn process_row<FB, FU, FD>(
                row_size: usize,
                base_func: FB,
                up_func: FU,
                down_func: FD,
            ) -> u32
            where
                FB: Fn(usize, bool, bool) -> u8x32,
                FU: Fn(usize, bool, bool) -> u8x32,
                FD: Fn(usize, bool, bool) -> u8x32,
            {
                let mut result = 0;

                // First chunk
                result += count_low_points(
                    base_func(0, true, false),
                    up_func(0, true, false),
                    down_func(0, true, false),
                );

                // Read whole chunks
                for j in 1..row_size / 30 {
                    result += count_low_points(
                        base_func(j, false, false),
                        up_func(j, false, false),
                        down_func(j, false, false),
                    );
                }

                // Padded chunk
                if row_size / 30 >= 1 && row_size % 30 != 0 {
                    result += count_low_points(
                        base_func(row_size / 30, false, true),
                        up_func(row_size / 30, false, true),
                        down_func(row_size / 30, false, true),
                    );
                }

                result
            }

            let mut low_point_count = 0;

            assert!(row_count > 1);
            assert!(row_size >= 30);

            // First row
            low_point_count += process_row(
                row_size,
                |j, l, r| read_chunk(0, j, l, r),
                |_, _, _| u8x32::splat(b'9'),
                |j, l, r| read_chunk(1, j, l, r),
            );

            // Middle rows
            for i in 1..row_count - 1 {
                low_point_count += process_row(
                    row_size,
                    |j, l, r| read_chunk(i, j, l, r),
                    |j, l, r| read_chunk(i - 1, j, l, r),
                    |j, l, r| read_chunk(i + 1, j, l, r),
                );
            }

            // Last row
            low_point_count += process_row(
                row_size,
                |j, l, r| read_chunk(row_count - 1, j, l, r),
                |j, l, r| read_chunk(row_count - 2, j, l, r),
                |_, _, _| u8x32::splat(b'9'),
            );

            low_point_count
        })
        .run_part(2, |&(_grid, _row_size, _row_count)| {});
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
