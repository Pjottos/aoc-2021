use crate::InputExtractor;

pub struct Numbers;

impl InputExtractor for Numbers {
    type Output = Vec<i64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.lines().map(|l| l.parse().unwrap()).collect()
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

pub struct CommaNumbers;

impl InputExtractor for CommaNumbers {
    type Output = Vec<i64>;

    fn extract(&self, text: &str) -> Self::Output {
        text.split(',').map(|p| p.trim().parse().unwrap()).collect()
    }
}
