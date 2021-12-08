use aoc_2021::*;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum SubmarineCmd {
    Forward,
    Up,
    Down,
}

impl FromStr for SubmarineCmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

fn main() {
    use SubmarineCmd::*;
    Harness::begin()
        .day(2)
        .extract(|text| {
            text.lines().map(|l| {
                let mut parts = l.split(' ');
                let kind = parts.next().unwrap().parse().unwrap();
                let val = parts.next().unwrap().parse::<i32>().unwrap();
                (kind, val)
            })
        })
        .run_part(1, |cmds| {
            let mut hor = 0;
            let mut depth = 0;

            for (kind, val) in cmds.clone() {
                match kind {
                    Forward => hor += val,
                    Down => depth += val,
                    Up => depth -= val,
                }
            }

            hor * depth
        })
        .run_part(2, |cmds| {
            let mut hor = 0;
            let mut depth = 0;
            let mut aim = 0;

            for (kind, val) in cmds.clone() {
                match kind {
                    Forward => {
                        hor += val;
                        depth += aim * val;
                    }
                    Down => aim += val,
                    Up => aim -= val,
                }
            }

            hor * depth
        });
}
