use aoc_2021::*;

const GRID_DIM: usize = 10;

// Add padding on all 4 sides of the grid
const DIM_SIZE: usize = GRID_DIM + 2;
const GRID_SIZE: usize = DIM_SIZE * DIM_SIZE;

fn main() {
    Harness::begin()
        .day(11)
        // .input_override("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n")
        .extract(|text| {
            let mut bytes = text
                .as_bytes()
                .iter()
                .enumerate()
                // Skip newlines
                .filter_map(|(i, &c)| (i % (GRID_DIM + 1) < GRID_DIM).then(|| c - b'0'));

            let mut grid = [0; GRID_SIZE];
            for y in 0..GRID_DIM {
                for x in 0..GRID_DIM {
                    let i = grid_index(x, y);
                    grid[i] = bytes.next().unwrap();
                }
            }

            grid
        })
        .run_part(1, |grid| {
            let mut grid = *grid;
            let mut flashes = 0;

            for _step in 0..100 {
                flashes += step_grid(&mut grid);
            }

            flashes
        })
        .run_part(2, |grid| {
            let mut grid = *grid;

            for step in 0..usize::MAX {
                if step_grid(&mut grid) == (GRID_DIM * GRID_DIM) as u32 {
                    return step + 1;
                }
            }

            panic!("no step found at which all octopusses flash")
        });
}

fn step_grid(grid: &mut [u8; GRID_SIZE]) -> u32 {
    let mut flashes = 0;

    grid.iter_mut().for_each(|c| *c += 1);

    loop {
        let mut any = false;
        for y in 0..GRID_DIM {
            for x in 0..GRID_DIM {
                let i = grid_index(x, y);

                if grid[i] > 9 && grid[i] < 0x80 {
                    any = true;
                    flashes += 1;

                    grid[i] |= 0x80;

                    grid[i - DIM_SIZE - 1] += 1;
                    grid[i - DIM_SIZE] += 1;
                    grid[i - DIM_SIZE + 1] += 1;

                    grid[i - 1] += 1;
                    grid[i + 1] += 1;

                    grid[i + DIM_SIZE - 1] += 1;
                    grid[i + DIM_SIZE] += 1;
                    grid[i + DIM_SIZE + 1] += 1;
                }
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

#[inline]
fn grid_index(x: usize, y: usize) -> usize {
    (y + 1) * DIM_SIZE + x + 1
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
