use aoc_2021::{
    ex::{self, types::SubmarineCmd},
    input,
};

fn main() {
    use SubmarineCmd::*;
    let cmds = input(2, &ex::SubmarineCmds);

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
