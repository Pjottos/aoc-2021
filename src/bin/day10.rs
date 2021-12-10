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

                for &c in line.as_bytes().iter() {
                    match c {
                        b')' | b']' | b'}' | b'>' => {
                            len -= 1;
                            if stack[len] != invert_bracket(c) {
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

                for &c in line.as_bytes().iter() {
                    match c {
                        b')' | b']' | b'}' | b'>' => {
                            len -= 1;
                            if stack[len] != invert_bracket(c) {
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

fn invert_bracket(c: u8) -> u8 {
    if c <= b')' {
        c ^ 1
    } else if c <= b'>' {
        c ^ 0b10
    } else {
        c ^ 0b110
    }
}
