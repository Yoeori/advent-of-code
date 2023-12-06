use std::{fs, ops::Range};

#[derive(Debug)]
struct Pair {
    time: f64,
    dist: f64,
}

impl Pair {
    fn beatable_range(&self) -> Range<i64> {
        let delta = ((self.time * self.time) - (4.0 * self.dist)).sqrt();
        let left = (self.time - delta) / 2.0;
        let right = (self.time + delta) / 2.0;

        ((left + 1.0).trunc() as i64)..((right - 1.0).ceil() as i64)
    }

    fn beatable_size(&self) -> i64 {
        let range = self.beatable_range();
        range.end - range.start + 1
    }
}

fn parse_without_whitespace(inp: &str) -> f64 {
    inp.split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn main() {
    let file: String = fs::read_to_string("puzzles/6.txt").unwrap();
    let (time_str, dist_str) = file.split_once('\n').unwrap();

    let pairs: Vec<Pair> = time_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .zip(
            dist_str
                .split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap()),
        )
        .map(|(time, dist)| Pair { time, dist })
        .collect();

    println!(
        "Exercise 1: {}",
        pairs
            .iter()
            .map(|x| x.beatable_size())
            .fold(1, |x, y| x * y)
    );

    let big_pair = Pair {
        time: parse_without_whitespace(time_str),
        dist: parse_without_whitespace(dist_str),
    };

    println!("Exercise 2: {}", big_pair.beatable_size());
}
