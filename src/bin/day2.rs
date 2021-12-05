use aoc_2021::*;

use std::str::FromStr;

struct SubmarineCmds;

impl InputExtractor for SubmarineCmds {
    type Output = Vec<(SubmarineCmd, i64)>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines()
            .map(|l| {
                let mut parts = l.split(' ');
                let kind = parts.next().unwrap().parse().unwrap();
                let val = parts.next().unwrap().parse().unwrap();
                (kind, val)
            })
            .collect()
    }
}

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
    let cmds = input(2, &SubmarineCmds);

    println!("part 1: {:#?}", {
        let mut hor = 0;
        let mut depth = 0;

        for (kind, val) in &cmds {
            match kind {
                Forward => hor += val,
                Down => depth += val,
                Up => depth -= val,
            }
        }

        hor * depth
    });

    println!("part 2: {:#?}", {
        let mut hor = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (kind, val) in &cmds {
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
