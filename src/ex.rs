use crate::InputExtractor;

pub struct Numbers;

impl InputExtractor for Numbers {
    type Output = Vec<i64>;

    fn extract(text: String) -> Self::Output {
        text.lines().map(|l| l.parse().unwrap()).collect()
    }
}

pub struct SubmarineCmds;

impl InputExtractor for SubmarineCmds {
    type Output = Vec<(u8, i64)>;

    fn extract(text: String) -> Self::Output {
        text.lines()
            .map(|l| {
                let mut parts = l.split(' ');
                let kind = match parts.next().unwrap() {
                    "forward" => 0,
                    "down" => 1,
                    "up" => 2,
                    _ => panic!(),
                };
                let val = parts.next().unwrap().parse().unwrap();
                (kind, val)
            })
            .collect()
    }
}
