pub fn nums(text: &str) -> Vec<i64> {
    text.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn binary_nums(text: &str) -> Vec<u64> {
    text.lines()
        .map(|l| u64::from_str_radix(l, 2).unwrap())
        .collect()
}
