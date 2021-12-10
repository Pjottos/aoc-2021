use aoc_2021::*;

fn main() {
    Harness::begin()
        .day(10)
        .extract(|text| text.lines())
        .run_part(1, |lines| {
            let mut score = 0;

            'lines: for line in lines.clone() {
                let mut stack = [0; 64];
                let mut len = 0;

                for &c in line.as_bytes() {
                    match c {
                        b')' | b']' | b'}' | b'>' => {
                            len -= 1;
                            // The stack only contains opening brackets and `c` can only be a closing
                            // bracket at this point.
                            if brackets_match(stack[len], c) {
                                score += match c {
                                    b')' => 3,
                                    b']' => 57,
                                    b'}' => 1197,
                                    b'>' => 25137,
                                    _ => 0,
                                };
                                continue 'lines;
                            }
                        }
                        b'(' | b'[' | b'{' | b'<' => {
                            stack[len] = c;
                            len += 1;
                        }
                        _ => panic!(),
                    }
                }
            }

            score
        })
        .run_part(2, |lines| {
            let mut scores = vec![];

            'lines: for line in lines.clone() {
                let mut stack = [0; 64];
                let mut len = 0;

                for &c in line.as_bytes() {
                    match c {
                        b')' | b']' | b'}' | b'>' => {
                            len -= 1;
                            // The stack only contains opening brackets and `c` can only be a closing
                            // bracket at this point.
                            if brackets_match(stack[len], c) {
                                continue 'lines;
                            }
                        }
                        b'(' | b'[' | b'{' | b'<' => {
                            stack[len] = c;
                            len += 1;
                        }
                        _ => panic!(),
                    }
                }

                let mut score = 0u64;
                for &c in stack[..len].iter().rev() {
                    score *= 5;
                    score += match c {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => 0,
                    };
                }

                scores.push(score);
            }

            scores.sort_unstable();
            scores[scores.len() / 2]
        });
}

#[inline]
fn brackets_match(open: u8, close: u8) -> bool {
    // We can take a look at the ASCII values for the bracket
    // symbols to do a cheap comparison for checking if the brackets match.
    //
    // ( 0010_1000
    // ) 0010_1001
    //
    // < 0011_1100
    // > 0011_1110
    //
    // [ 0101_1011
    // ] 0101_1101
    //
    // { 0111_1011
    // } 0111_1101
    //
    // Evidently, if the high 4 bits are equal then the brackets match.
    // We can make this comparison with an xor between the opening and closing bracket
    // and checking if any of the high 4 bits are set in the result.
    open ^ close >= 0x10
}
