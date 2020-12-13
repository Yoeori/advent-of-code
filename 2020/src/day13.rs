use std::fs;
use ring_algorithm::chinese_remainder_theorem;

pub fn main() {
    let file = fs::read_to_string("puzzles/13.txt").unwrap();
    let mut lines = file.lines();

    let start_time: usize = lines.next().unwrap().parse().unwrap();
    let bus_string: &str = lines.next().unwrap();

    // Part 1
    println!("Solution to exercise 1: {}", bus_string.split(',').filter(|&x| x != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .map(|bus| (bus, ((start_time / bus) + 1) * bus)) // This only works if start_time % bus != 0
        .min_by(|x, y| x.1.cmp(&y.1))
        .map(|bus| (bus.1 - start_time) * bus.0)
        .unwrap());

    // Part 2
    let (offset, buses): (Vec<i64>, Vec<i64>) = bus_string
        .split(',')
        .enumerate()
        .filter(|(_, bus)| bus != &"x")
        .map(|(offset, bus)| (offset as i64, bus.parse::<i64>().unwrap()))
        .unzip();

    println!("Solution to exercise 2: {}", chinese_remainder_theorem::<i64>(&offset, &buses).unwrap().abs());
}