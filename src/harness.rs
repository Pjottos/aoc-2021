use reqwest::{blocking::ClientBuilder, cookie};

use std::{
    fmt::Debug,
    fs,
    sync::Arc,
    time::{Duration, Instant},
};

pub struct Harness<E> {
    day: Option<u32>,
    input: Option<(E, Duration)>,
    text: Option<String>,
    bench: bool,
}

impl<'a, E> Harness<E> {
    const ITERATIONS: usize = 1 << 15;

    pub fn begin() -> Self {
        Self {
            day: None,
            input: None,
            text: None,
            bench: false,
        }
    }

    pub fn day(&'a mut self, day: u32) -> &'a mut Self {
        self.day = Some(day);
        self
    }

    pub fn bench(&'a mut self) -> &'a mut Self {
        self.bench = true;
        self
    }

    pub fn input_override<S: Into<String>>(&'a mut self, input_override: S) -> &'a mut Self {
        self.text = Some(input_override.into());
        self
    }

    pub fn extract<X>(&'a mut self, extractor: X) -> &'a Self
    where
        X: Fn(&'a str) -> E,
    {
        let day = self.day.unwrap();
        if self.text.is_none() {
            let input_path = format!("inputs/{}.txt", day);
            let text = fs::read_to_string(&input_path).unwrap_or_else(|_| {
                let text = download_input(day);
                fs::write(&input_path, &text).unwrap();
                text
            });
            self.text = Some(text);
        }
        let text = self.text.as_ref().unwrap();

        let mut time_sum = 0;
        let input = extractor(text);

        if self.bench {
            for _ in 0..Self::ITERATIONS {
                let begin = Instant::now();
                extractor(text);
                time_sum += begin.elapsed().as_nanos();
            }
        }

        let extract_time = Duration::from_nanos((time_sum / Self::ITERATIONS as u128) as u64);

        self.input = Some((input, extract_time));
        self
    }

    pub fn run_part<F, R>(&'a self, part_num: u32, func: F) -> &'a Self
    where
        F: Fn(&E) -> R,
        R: Debug,
    {
        let (input, extract_time) = self.input.as_ref().expect("input not extracted yet");

        let mut time_sum = 0;
        let res = func(input);
        println!("Part {}: {:?}", part_num, res);

        if self.bench {
            println!("Running benchmark...");
            for _ in 0..Self::ITERATIONS {
                let begin = Instant::now();
                func(input);
                time_sum += begin.elapsed().as_nanos();
            }

            let run_time = Duration::from_nanos((time_sum / Self::ITERATIONS as u128) as u64);

            println!(
                "Result: {:?} ({:?} excluding extract)",
                *extract_time + run_time,
                run_time,
            );
        }

        self
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
