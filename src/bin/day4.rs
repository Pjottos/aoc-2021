use aoc_2021::*;

#[derive(Default, Clone)]
struct Board {
    finished: bool,
    cells: [(u8, bool); Self::DIM * Self::DIM],
}

impl Board {
    const DIM: usize = 5;

    fn check_victory(&mut self) -> bool {
        self.finished =
            if (0..Self::DIM).any(|r| self.cells.chunks_exact(Self::DIM).all(|col| col[r].1)) {
                // row
                true
            } else {
                // column
                self.cells
                    .chunks_exact(Self::DIM)
                    .any(|col| col.iter().all(|c| c.1))
            };

        self.finished
    }
}

fn main() {
    Harness::begin()
        .day(4)
        .extract(|text| {
            let mut lines = text.lines();

            let nums = lines.next().unwrap().split(',').map(|p| p.parse().unwrap());

            let mut boards = vec![];
            loop {
                match lines.next() {
                    Some("") => {
                        let mut board = Board::default();
                        for (y, line) in lines.by_ref().take(Board::DIM).enumerate() {
                            for (x, num) in line.split_ascii_whitespace().enumerate() {
                                board.cells[x * Board::DIM + y].0 = num.parse().unwrap();
                            }
                        }

                        boards.push(board);
                    }
                    None => break,
                    _ => panic!(),
                }
            }

            (nums, boards)
        })
        .run_part(1, |(nums, boards)| {
            calc_victory_score(nums.clone(), boards.clone(), true).unwrap()
        })
        .run_part(2, |(nums, boards)| {
            calc_victory_score(nums.clone(), boards.clone(), false).unwrap()
        });
}

fn calc_victory_score<N>(nums: N, mut boards: Vec<Board>, use_first_win: bool) -> Option<u32>
where
    N: Iterator<Item = u8>,
{
    let mut victory_score = None;

    'nums: for num in nums {
        for board in boards.iter_mut().filter(|b| !b.finished) {
            if let Some(cell) = board.cells.iter_mut().find(|(n, _)| *n == num) {
                cell.1 = true;
                if board.check_victory() {
                    let score = board
                        .cells
                        .iter()
                        .filter_map(|(num, marked)| (!*marked).then(|| u32::from(*num)))
                        .sum::<u32>()
                        * u32::from(num);

                    victory_score = Some(score);
                    if use_first_win {
                        break 'nums;
                    }
                }
            }
        }
    }

    victory_score
}
