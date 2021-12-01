use std::{
    collections::VecDeque,
    io::{self, prelude::*},
};

fn main() {
    part_2();
}

fn part_1() {
    let res = io::stdin()
        .lock()
        .lines()
        .map(|res| res.unwrap().parse().unwrap())
        .fold((0, None), |(mut count, prev), v: u32| {
            if prev.map(|p| v > p).unwrap_or(false) {
                count += 1;
            }
            (count, Some(v))
        });

    println!("{}", res.0)
}

fn part_2() {
    let mut buf = VecDeque::new();
    let count = io::stdin()
        .lock()
        .lines()
        .map(|res| res.unwrap().parse().unwrap())
        .fold(0, |mut count, v: u32| {
            buf.push_back(v);

            if buf.len() == 4 {
                let a: u32 = buf.iter().take(3).sum();
                let b: u32 = buf.iter().skip(1).sum();

                if b > a {
                    count += 1;
                }

                buf.pop_front();
            }

            count
        });

    println!("{}", count)
}
