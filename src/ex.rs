use crate::InputExtractor;

pub struct Numbers;

impl InputExtractor for Numbers {
    type Output = Vec<i64>;

    fn extract(text: String) -> Self::Output {
        text.lines().map(|l| l.parse().unwrap()).collect()
    }
}
