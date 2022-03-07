#![feature(portable_simd)]

use aoc_2021::*;

use std::{
    mem::transmute,
    simd::{u16x16, u8x32},
};

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
        .run_part(2, |&(grid, row_size, row_count)| {
            // Build a bitmap of 16x16 blocks, each bit represents a cell that is
            // empty (1) or filled (0).
            // The bitmap is rotated 90 degrees, but this doesn't matter for finding the
            // sizes of the basins.
            let h_blocks = (row_count + 15) / 16;
            let v_blocks = (row_size - 1 + 15) / 16;
            let mut bitmap = Vec::with_capacity(h_blocks * v_blocks);

            let mut create_block = |max_k: usize, max_l: usize, j: usize, i: usize| {
                let mut block = u16x16::splat(0);
                let mut bit_mask = u16x16::splat(1);
                let cmp_mask = u16x16::splat(u16::from(b'9'));
                for k in 0..max_k {
                    let mut cells = cmp_mask;
                    for l in 0..max_l {
                        // Horizontally adjacent bits in the block lanes are vertically
                        // adjacent in the grid.
                        cells[l] = u16::from(grid[(j * 16 + k) * row_size + i * 16 + l]);
                    }

                    let cmp = cells.lanes_ne(cmp_mask).to_int();
                    block |= unsafe { transmute::<_, u16x16>(cmp) } & bit_mask;
                    bit_mask <<= u16x16::splat(1);
                }

                bitmap.push(block);
            };

            for i in 0..(row_size - 1) / 16 {
                for j in 0..row_count / 16 {
                    create_block(16, 16, j, i);
                }

                if row_count % 16 != 0 {
                    create_block(row_count % 16, 16, row_count / 16, i);
                }
            }

            if (row_size - 1) % 16 != 0 {
                for j in 0..row_count / 16 {
                    create_block(16, (row_size - 1) % 16, j, (row_size - 1) / 16);
                }

                if row_count % 16 != 0 {
                    create_block(
                        row_count % 16,
                        (row_size - 1) % 16,
                        row_count / 16,
                        (row_size - 1) / 16,
                    );
                }
            }

            // for (a, b) in bitmap[5].as_array().iter().zip(bitmap[6].as_array().iter()) {
            //     println!("{:016b} {:016b}", a, b);
            // }
            // println!();
            // for (a, b) in bitmap[41].as_array().iter().zip(bitmap[42].as_array().iter()) {
            //     println!("{:016b} {:016b}", a, b);
            // }
            // return 0;

            // Keep a stack of activation blocks, these overlap with the bitmap blocks
            // and keep track of cells that are newly filled (1) or not (0).
            struct ActivationBlock {
                value: u16x16,
                x: usize,
                y: usize,
            }

            let mut stack = vec![];
            let mut basins = vec![];

            let push_activation = |stack: &mut Vec<ActivationBlock>,
                                   bitmap: &[u16x16],
                                   value: u16x16,
                                   x: usize,
                                   y: usize| {
                let block = bitmap[y * h_blocks + x];
                // Only if there is an activation on a cell, and the respective
                // bitmap cell is empty, will the activation block have any effect.
                // So don't push it to the stack if that is not the case for all cells.
                if (value & block).lanes_ne(u16x16::splat(0)).any() {
                    stack.push(ActivationBlock { value, x, y });
                }
            };

            for i in 0..v_blocks {
                let mut j = 0;
                while j < h_blocks {
                    // Find the first empty cell in the current block.
                    if let Some((b, zeros)) = bitmap[i * h_blocks + j]
                        .as_array()
                        .iter()
                        .copied()
                        .map(u16::leading_zeros)
                        .enumerate()
                        .find(|&(_, zeros)| zeros != 16)
                    {
                        let mut value = u16x16::splat(0);
                        value[b] |= 1 << (15 - zeros);

                        stack.push(ActivationBlock { value, x: j, y: i });
                    } else {
                        // Block is completely filled, no basin here.
                        j += 1;
                        continue;
                    }

                    let mut basin_size = 0;

                    while let Some(mut act) = stack.pop() {
                        let block_idx = act.y * h_blocks + act.x;
                        let mut block = bitmap[block_idx];
                        let old_block = block;
                        let mut act_left = u16x16::splat(0);
                        let mut act_right = u16x16::splat(0);
                        let mut act_up = u16x16::splat(0);
                        let mut act_down = u16x16::splat(0);

                        loop {
                            // let labels = [
                            //     "block",
                            //     "act.value",
                            //     "act_up",
                            //     "act_down",
                            //     "act_left",
                            //     "act_right",
                            // ];
                            // for label in labels {
                            //     print!("{}", label);
                            //     for _ in 0..17 - label.len() {
                            //         print!(" ");
                            //     }
                            // }
                            // println!();
                            // for b in 0..16 {
                            //     print!("{:016b}", block[b]);
                            //     print!(" {:016b}", act.value[b]);
                            //     print!(" {:016b}", act_up[b]);
                            //     print!(" {:016b}", act_down[b]);
                            //     print!(" {:016b}", act_left[b]);
                            //     print!(" {:016b}", act_right[b]);
                            //     println!();
                            // }
                            // println!();

                            // Bits are set for every cell that is about to be filled
                            let to_fill = block & act.value;
                            // The codegen is suboptimal for this line, but transmuting to u8x32
                            // seems to improve it a bit.
                            if unsafe { transmute::<_, u8x32>(to_fill) } == u8x32::splat(0) {
                                // No cells to fill, done with this block
                                break;
                            }
                            // Fill activated cells by setting them to 0
                            block &= !act.value;
                            // Set activation mask to only the cells that were just filled, to prevent
                            // growing past a basin border.
                            act.value = to_fill;

                            // Now we grow the activation wave by 1 in all directions
                            let mut first_lane_mask = u16x16::splat(0);
                            first_lane_mask[0] = u16::MAX;
                            let last_lane_mask = first_lane_mask.rotate_lanes_right::<15>();

                            // Handle growth past the block borders
                            act_left |= act.value << u16x16::splat(15);
                            act_right |= act.value >> u16x16::splat(15);
                            // First row becomes last row
                            act_up |= act.value.rotate_lanes_left::<1>() & last_lane_mask;
                            // Last row becomes first row
                            act_down |= act.value.rotate_lanes_left::<15>() & first_lane_mask;

                            // println!("{:04x}", act_up);
                            // println!("{:04x}", act.value);
                            // println!("{:04x}", act_down);
                            // println!();
                            // return 0;

                            // Grow the wave inside the block
                            let left = act.value << u16x16::splat(1);
                            let right = act.value >> u16x16::splat(1);
                            let up = act.value.rotate_lanes_left::<1>() & !last_lane_mask;
                            let down = act.value.rotate_lanes_right::<1>() & !first_lane_mask;
                            act.value = left | right | up | down;
                        }

                        basin_size += (old_block ^ block)
                            .as_array()
                            .iter()
                            .copied()
                            .map(u16::count_ones)
                            .sum::<u32>();
                        bitmap[block_idx] = block;

                        if act.x > 0 {
                            push_activation(&mut stack, &bitmap, act_left, act.x - 1, act.y);
                        }
                        if act.x + 1 < h_blocks {
                            push_activation(&mut stack, &bitmap, act_right, act.x + 1, act.y);
                        }
                        if act.y > 0 {
                            push_activation(&mut stack, &bitmap, act_up, act.x, act.y - 1);
                        }
                        if act.y + 1 < v_blocks {
                            push_activation(&mut stack, &bitmap, act_down, act.x, act.y + 1);
                        }
                    }

                    basins.push(basin_size);
                }
            }

            basins.sort_unstable();
            basins.iter().rev().take(3).product::<u32>()
        });
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
