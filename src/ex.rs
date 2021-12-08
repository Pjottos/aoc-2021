pub fn nums(text: &str) -> Vec<i64> {
    text.lines().map(|l| l.parse().unwrap()).collect()
}
