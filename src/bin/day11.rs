use aoc_2021::*;

const GRID_DIM: usize = 10;

fn main() {
    Harness::begin()
        .day(11)
        // .input_override("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n")
        .extract(|text| {
            text.as_bytes()
                .iter()
                .enumerate()
                // Skip newlines
                .filter_map(|(i, &c)| (i % (GRID_DIM + 1) < GRID_DIM).then(|| c - b'0'))
        })
        .run_part(1, |bytes| {
            let mut grid: Vec<_> = bytes.clone().collect();
            let mut flashes = 0;

            assert_eq!(grid.len(), GRID_DIM * GRID_DIM);

            for _step in 0..100 {
                flashes += step_grid(&mut grid);
            }

            flashes
        })
        .run_part(2, |bytes| {
            let mut grid: Vec<_> = bytes.clone().collect();

            assert_eq!(grid.len(), GRID_DIM * GRID_DIM);

            for step in 0..usize::MAX {
                if step_grid(&mut grid) == grid.len() as u32 {
                    return step + 1;
                }
            }

            panic!("no step found at which all octopusses flash")
        });
}

fn step_grid(grid: &mut [u8]) -> u32 {
    let mut flashes = 0;

    grid.iter_mut().for_each(|c| *c += 1);

    loop {
        let mut any = false;
        for i in 0..grid.len() {
            if grid[i] <= 9 || grid[i] >= 0x80 {
                continue;
            }

            any = true;
            flashes += 1;
            grid[i] = 0x80;

            if i == 0 {
                // Top left corner
                grid[1] += 1;
                grid[GRID_DIM] += 1;
                grid[GRID_DIM + 1] += 1;
            } else if i < GRID_DIM - 1 {
                // Top row
                grid[i - 1] += 1;
                grid[i + 1] += 1;

                grid[i + GRID_DIM - 1] += 1;
                grid[i + GRID_DIM] += 1;
                grid[i + GRID_DIM + 1] += 1;
            } else if i == GRID_DIM - 1 {
                // Top right corner
                grid[i - 1] += 1;
                grid[i + GRID_DIM - 1] += 1;
                grid[i + GRID_DIM] += 1;
            } else if i == grid.len() - 1 {
                // Bottom right corner
                grid[i - 1] += 1;
                grid[i - GRID_DIM] += 1;
                grid[i - GRID_DIM - 1] += 1;
            } else if i % GRID_DIM == GRID_DIM - 1 {
                // Right column
                grid[i - GRID_DIM] += 1;
                grid[i + GRID_DIM] += 1;

                grid[i - GRID_DIM - 1] += 1;
                grid[i - 1] += 1;
                grid[i + GRID_DIM - 1] += 1;
            } else if i > grid.len() - GRID_DIM {
                // Bottom row
                grid[i - 1] += 1;
                grid[i + 1] += 1;

                grid[i - GRID_DIM - 1] += 1;
                grid[i - GRID_DIM] += 1;
                grid[i - GRID_DIM + 1] += 1;
            } else if i == grid.len() - GRID_DIM {
                // Bottom left corner
                grid[i + 1] += 1;
                grid[i - GRID_DIM] += 1;
                grid[i - GRID_DIM + 1] += 1;
            } else if i % GRID_DIM == 0 {
                // Left column
                grid[i - GRID_DIM] += 1;
                grid[i + GRID_DIM] += 1;

                grid[i - GRID_DIM + 1] += 1;
                grid[i + 1] += 1;
                grid[i + GRID_DIM + 1] += 1;
            } else {
                // Any cell not at an edge
                grid[i - GRID_DIM - 1] += 1;
                grid[i - GRID_DIM] += 1;
                grid[i - GRID_DIM + 1] += 1;

                grid[i - 1] += 1;
                grid[i + 1] += 1;

                grid[i + GRID_DIM - 1] += 1;
                grid[i + GRID_DIM] += 1;
                grid[i + GRID_DIM + 1] += 1;
            }
        }

        if !any {
            break;
        }
    }

    // Reset octopusses that flashed
    grid.iter_mut()
        .filter(|c| **c & 0x80 != 0)
        .for_each(|c| *c = 0);

    flashes
}

#[allow(dead_code)]
fn print_grid(grid: &[u8]) {
    use termion::color;

    for row in grid.chunks_exact(GRID_DIM) {
        for r in row {
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
