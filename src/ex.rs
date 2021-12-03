use crate::InputExtractor;

pub struct Numbers;

impl InputExtractor for Numbers {
    type Output = Vec<i64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines().map(|l| l.parse().unwrap()).collect()
    }
}

pub struct SubmarineCmds;

impl InputExtractor for SubmarineCmds {
    type Output = Vec<(types::SubmarineCmd, i64)>;

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

pub struct BinaryNumbers;

impl InputExtractor for BinaryNumbers {
    type Output = Vec<u64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines()
            .map(|l| u64::from_str_radix(l, 2).unwrap())
            .collect()
    }
}

pub mod types {
    use std::str::FromStr;

    pub enum SubmarineCmd {
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
}
