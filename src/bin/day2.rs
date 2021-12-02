use aoc_2021::{ex, input};

fn main() {
    let cmds = input::<ex::SubmarineCmds>(2);

    println!("part 1: {:#?}", {
        let mut hor = 0;
        let mut depth = 0;

        for (kind, val) in &cmds {
            match kind {
                0 => hor += val,
                1 => depth += val,
                2 => depth -= val,
                _ => (),
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
                0 => {
                    hor += val;
                    depth += aim * val;
                }
                1 => aim += val,
                2 => aim -= val,
                _ => (),
            }
        }

        hor * depth
    });
}
