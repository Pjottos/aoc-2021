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
}

impl<'a, E> Harness<E> {
    pub fn begin() -> Self {
        Self {
            day: None,
            input: None,
            text: None,
        }
    }

    pub fn day(&'a mut self, day: u32) -> &'a mut Self {
        self.day = Some(day);
        self
    }

    pub fn input_override<S: Into<String>>(&'a mut self, input_override: S) -> &'a mut Self {
        self.text = Some(input_override.into());
        self
    }

    pub fn extract<X>(&'a mut self, extractor: X) -> &'a Self
    where
        X: FnOnce(&'a str) -> E,
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

        let begin = Instant::now();
        let input = extractor(text);
        let extract_time = begin.elapsed();

        self.input = Some((input, extract_time));
        self
    }

    pub fn run_part<F, R>(&'a self, part_num: u32, func: F) -> &'a Self
    where
        F: FnOnce(&E) -> R,
        R: Debug,
    {
        let (input, extract_time) = self.input.as_ref().expect("input not extracted yet");

        let begin = Instant::now();
        let res = func(input);
        let run_time = begin.elapsed();

        println!(
            "part {}: {:?} in {:?} ({:?} excluding parsing)",
            part_num,
            res,
            *extract_time + run_time,
            run_time
        );

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
