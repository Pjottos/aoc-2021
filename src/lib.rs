use reqwest::{blocking::ClientBuilder, cookie};

use std::{fs, sync::Arc};

pub mod ex;

const YEAR: u32 = 2021;

pub fn input<X: InputExtractor>(day: u32) -> X::Output {
    let path = format!("inputs/{}.txt", day);
    let text = fs::read_to_string(&path).unwrap_or_else(|_| {
        let text = download_input(day);
        fs::write(&path, &text).unwrap();
        text
    });
    X::extract(text)
}

fn download_input(day: u32) -> String {
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

pub trait InputExtractor {
    type Output;

    fn extract(text: String) -> Self::Output;
}
