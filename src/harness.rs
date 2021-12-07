use reqwest::{blocking::ClientBuilder, cookie};

use std::{fs, sync::Arc, time::Instant};

pub trait InputExtractor {
    type Output;

    fn extract(&self, text: &str) -> Self::Output;
}

pub struct Harness<X, P1, P2>
where
    X: InputExtractor,
    P1: FnOnce(&X::Output) -> i64,
    P2: FnOnce(&X::Output) -> i64,
{
    day: u32,
    extractor: X,
    part_1: Option<P1>,
    part_2: Option<P2>,
}

impl<X, P1, P2> Harness<X, P1, P2>
where
    X: InputExtractor,
    P1: FnOnce(&X::Output) -> i64,
    P2: FnOnce(&X::Output) -> i64,
{
    pub fn builder() -> HarnessBuilder<X, P1, P2> {
        HarnessBuilder::default()
    }

    pub fn run(self) {
        let input_path = format!("inputs/{}.txt", self.day);
        let text = fs::read_to_string(&input_path).unwrap_or_else(|_| {
            let text = download_input(self.day);
            fs::write(&input_path, &text).unwrap();
            text
        });

        let begin = Instant::now();
        let input = self.extractor.extract(&text);
        let extract_time = begin.elapsed();

        let begin = Instant::now();
        let res_1 = self.part_1.map(|f| f(&input));
        let part_1_time = begin.elapsed();

        let begin = Instant::now();
        let res_2 = self.part_2.map(|f| f(&input));
        let part_2_time = begin.elapsed();

        println!("part 1: {:?} in {:?}", res_1, extract_time + part_1_time);
        println!("part 2: {:?} in {:?}", res_2, extract_time + part_2_time);
    }
}

pub struct HarnessBuilder<X, P1, P2>
where
    X: InputExtractor,
    P1: FnOnce(&X::Output) -> i64,
    P2: FnOnce(&X::Output) -> i64,
{
    day: Option<u32>,
    extractor: Option<X>,
    part_1: Option<P1>,
    part_2: Option<P2>,
}

impl<X, P1, P2> Default for HarnessBuilder<X, P1, P2>
where
    X: InputExtractor,
    P1: FnOnce(&X::Output) -> i64,
    P2: FnOnce(&X::Output) -> i64,
{
    fn default() -> Self {
        Self {
            day: None,
            extractor: None,
            part_1: None,
            part_2: None,
        }
    }
}

impl<X, P1, P2> HarnessBuilder<X, P1, P2>
where
    X: InputExtractor,
    P1: FnOnce(&X::Output) -> i64,
    P2: FnOnce(&X::Output) -> i64,
{
    pub fn day(mut self, day: u32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn extractor(mut self, extractor: X) -> Self {
        self.extractor = Some(extractor);
        self
    }

    pub fn part_1(mut self, part_1: P1) -> Self {
        self.part_1 = Some(part_1);
        self
    }

    pub fn part_2(mut self, part_2: P2) -> Self {
        self.part_2 = Some(part_2);
        self
    }

    pub fn run(self) {
        let harness = Harness {
            day: self.day.unwrap(),
            extractor: self.extractor.unwrap(),
            part_1: self.part_1,
            part_2: self.part_2,
        };

        harness.run();
    }
}

fn download_input(day: u32) -> String {
    const YEAR: u32 = 2021;

    let jar = Arc::new(cookie::Jar::default());
    let session = fs::read_to_string("session").unwrap();
    jar.add_cookie_str(&session, &"https://adventofcode.com".parse().unwrap());
    let client = ClientBuilder::new()
        .cookie_provider(jar)
        .gzip(true)
        .build()
        .unwrap();

    client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap()
}
