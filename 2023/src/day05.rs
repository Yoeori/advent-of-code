use std::{cmp, fs, ops::Range};

#[derive(Debug)]
struct Map(Vec<(Range<i64>, i64)>);

impl Map {
    fn parse(inp: &str) -> Self {
        let mut lines = inp.lines();
        let _ = lines.next(); // map name

        let mut converters: Vec<(Range<i64>, i64)> = lines
            .map(|l| {
                let mut l = l.split(' ').map(|x| x.parse::<i64>().unwrap());
                let to: i64 = l.next().unwrap();
                let from = l.next().unwrap();
                let length: i64 = l.next().unwrap();
                (from..(from + length), to - from)
            })
            .collect();

        converters.sort_by_key(|c| c.0.start);

        Map(converters)
    }

    fn convert_range(&self, from: i64, length: i64) -> (i64, i64) {
        for (range, delta) in self.0.iter() {
            if from < range.start {
                // inbetween the cracks
                return (from, cmp::min(range.start - from, length));
            }

            if range.contains(&from) {
                return (from + delta, cmp::min(range.end - from, length));
            }
        }
        (from, length)
    }
}

fn find_lowest_for_range(range: Range<i64>, maps: &[Map]) -> i64 {
    if maps.len() == 0 {
        return range.start;
    }

    let mut min = i64::MAX;
    let mut cur = range.start;
    while cur != range.end {
        let (res, size) = maps[0].convert_range(cur, range.end - cur);
        min = cmp::min(min, find_lowest_for_range(res..(res + size), &maps[1..]));
        cur += size;
    }

    min
}

pub fn main() {
    let file: String = fs::read_to_string("puzzles/5.txt").unwrap();

    let mut sections = file.split("\n\n");

    let seeds: Vec<i64> = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();

    let maps: Vec<Map> = sections.map(|section| Map::parse(section)).collect();
    println!(
        "Exercise 1: {}",
        seeds
            .iter()
            .map(|&x| maps.iter().fold(x, |c, map| map.convert_range(c, 1).0))
            .min()
            .unwrap()
    );

    println!(
        "Exercise 2: {}",
        seeds
            .chunks(2)
            .map(|chunk| find_lowest_for_range(chunk[0]..(chunk[0] + chunk[1]), &maps))
            .min()
            .unwrap()
    );
}