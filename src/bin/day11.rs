#![feature(portable_simd)]

use aoc_2021::*;

use std::simd::i8x16;

const GRID_DIM: usize = 10;
type Grid = [i8x16; GRID_DIM + 2];

fn main() {
    Harness::begin()
        .day(11)
        // .input_override("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n")
        .extract(|text| {
            let mut cells = text
                .as_bytes()
                .iter()
                .enumerate()
                // Skip newlines
                .filter_map(|(i, &c)| (i % (GRID_DIM + 1) < GRID_DIM).then(|| (c - b'0') as i8));

            let mut grid = Grid::default();
            let mut row = [0; 16];
            for i in 0..GRID_DIM {
                for j in 0..GRID_DIM {
                    row[3 + j] = cells.next().unwrap();
                }

                grid[1 + i] = i8x16::from_array(row);
            }

            grid
        })
        .run_part(1, |grid| {
            let mut grid = *grid;
            let mut flashes = 0;

            for _step in 0..100 {
                flashes += step_grid(&mut grid) as u32;
            }

            flashes
        })
        .run_part(2, |grid| {
            let mut grid = *grid;

            for step in 0..usize::MAX {
                if step_grid(&mut grid) == (GRID_DIM * GRID_DIM) as u8 {
                    return step + 1;
                }
            }

            panic!("no step found at which all octopusses flash")
        });
}

fn step_grid(grid: &mut Grid) -> u8 {
    for row in grid.iter_mut() {
        *row += i8x16::splat(1);
    }

    let mut flashes = i8x16::splat(0);

    loop {
        let old_flashes = flashes;

        for i in 1..GRID_DIM + 1 {
            // Flashed cells are negative so this is all we need.
            let cmp = grid[i].lanes_gt(i8x16::splat(9));
            let cur_flashes = cmp.select(i8x16::splat(1), i8x16::splat(0));
            flashes += cur_flashes;

            // Set a flag on all flashed cells
            grid[i] |= cur_flashes << 7;

            let left_shift = cur_flashes.rotate_lanes_left::<1>();
            let right_shift = cur_flashes.rotate_lanes_right::<1>();
            let shifts = left_shift + right_shift;

            grid[i - 1] += shifts + cur_flashes;
            grid[i] += shifts;
            grid[i + 1] += shifts + cur_flashes;
        }

        if flashes.lanes_eq(old_flashes).all() {
            break;
        }
    }

    // Set flashed cells to 0
    for row in grid.iter_mut() {
        let cmp = row.lanes_gt(i8x16::splat(-1));
        *row &= cmp.select(i8x16::splat(-1), i8x16::splat(0));
    }

    // Make sure padding cells are always considered as flashed
    grid[0] = i8x16::splat(1 << 7);
    grid[GRID_DIM + 1] = i8x16::splat(1 << 7);

    // Create a mask for setting the flashed flag on all horizontal padding cells
    let mut mask_buf = [0; 16];
    mask_buf[..3].fill(1 << 7);
    mask_buf[13..].fill(1 << 7);
    let mask = i8x16::from_array(mask_buf);

    for row in &mut grid[1..GRID_DIM + 1] {
        *row |= mask;
    }

    flashes.horizontal_sum() as u8
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    use termion::color;

    for row in &grid[1..1 + GRID_DIM] {
        for r in &row.as_array()[3..3 + GRID_DIM] {
            let r = *r as u8;
            let c = ((r & !0x80) + b'0') as char;
            if r & 0x80 != 0 {
                print!("{}{}", color::Fg(color::LightWhite), c);
            } else {
                print!("{}{}", color::Fg(color::LightBlack), c);
            }
        }
        println!("{}", color::Fg(color::Reset));
    }
}
