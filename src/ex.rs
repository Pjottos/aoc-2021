use crate::{ex::types::*, InputExtractor};

pub struct Numbers;

impl InputExtractor for Numbers {
    type Output = Vec<i64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines().map(|l| l.parse().unwrap()).collect()
    }
}

pub struct SubmarineCmds;

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

pub struct BinaryNumbers;

impl InputExtractor for BinaryNumbers {
    type Output = Vec<u64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines()
            .map(|l| u64::from_str_radix(l, 2).unwrap())
            .collect()
    }
}

pub struct CloudLines;

impl InputExtractor for CloudLines {
    type Output = Vec<(Point, Point)>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines()
            .map(|l| {
                let mut parts = l.split(" -> ");
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }
}

pub mod types {
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug, Clone, Copy)]
    pub struct Point {
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
}
