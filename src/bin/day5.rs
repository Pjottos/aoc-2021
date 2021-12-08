use aoc_2021::*;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',');
        let x = nums.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = nums.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(Self { x, y })
    }
}

fn main() {
    Harness::begin()
        .day(5)
        .extract(|text| {
            text.lines().map(|l| {
                let mut parts = l.split(" -> ");
                (
                    parts.next().unwrap().parse::<Point>().unwrap(),
                    parts.next().unwrap().parse::<Point>().unwrap(),
                )
            })
        })
        .run_part(1, |lines| {
            let mut grid = vec![0u8; 1024 * 1024];

            for line in lines.clone() {
                if line.0.x == line.1.x {
                    let min_y = line.0.y.min(line.1.y);
                    let max_y = line.0.y.max(line.1.y);
                    let start = line.0.x * 1024 + min_y;
                    let len = max_y - min_y + 1;
                    for cell in &mut grid[start..start + len] {
                        *cell += 1;
                    }
                } else if line.0.y == line.1.y {
                    let min_x = line.0.x.min(line.1.x);
                    let max_x = line.0.x.max(line.1.x);
                    let len = max_x - min_x + 1;
                    for col in grid.chunks_exact_mut(1024).skip(min_x).take(len) {
                        col[line.0.y] += 1;
                    }
                }
            }

            grid.iter().copied().filter(|&c| c > 1).count()
        })
        .run_part(2, |lines| {
            let mut grid = vec![0u8; 1024 * 1024];

            for line in lines.clone() {
                if line.0.x == line.1.x {
                    let min_y = line.0.y.min(line.1.y);
                    let max_y = line.0.y.max(line.1.y);
                    let start = line.0.x * 1024 + min_y;
                    let len = max_y - min_y + 1;
                    for cell in &mut grid[start..start + len] {
                        *cell += 1;
                    }
                } else if line.0.y == line.1.y {
                    let min_x = line.0.x.min(line.1.x);
                    let max_x = line.0.x.max(line.1.x);
                    let len = max_x - min_x + 1;
                    for col in grid.chunks_exact_mut(1024).skip(min_x).take(len) {
                        col[line.0.y] += 1;
                    }
                } else {
                    let min_x = line.0.x.min(line.1.x);
                    let max_x = line.0.x.max(line.1.x);
                    let min_y = line.0.y.min(line.1.y);

                    let len_x = max_x - min_x + 1;
                    let move_up = min_y == line.0.y;
                    let cols = grid.chunks_exact_mut(1024);

                    if min_x == line.0.x {
                        count_diagonal(cols.skip(line.0.x), line.0.y, move_up, len_x);
                    } else {
                        count_diagonal(
                            cols.rev().skip(1024 - line.0.x - 1),
                            line.0.y,
                            move_up,
                            len_x,
                        );
                    }
                }
            }

            grid.iter().copied().filter(|&c| c > 1).count()
        });
}

// Reverse iterator has a different type so this needs to be a generic function.
fn count_diagonal<'a, I>(cols: I, start_y: usize, move_up: bool, len_x: usize)
where
    I: Iterator<Item = &'a mut [u8]>,
{
    for (i, col) in cols.take(len_x).enumerate() {
        let idx = if move_up { start_y + i } else { start_y - i };

        col[idx] += 1;
    }
}
