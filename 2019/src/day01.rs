use math::round;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let reader = BufReader::new(File::open("puzzles/01.txt").unwrap());

    let mut total: f32 = 0.0;
    for (_index, line) in reader.lines().enumerate() {
        total += calculate_fuel(line.unwrap().parse::<f32>().unwrap())
    }
    println!("{}", total)
}

fn calculate_fuel(on: f32) -> f32 {
    let fuel = (round::floor((on as f64) / 3.0, 0) as f32) - 2.0;

    return if fuel > 0.0 {
        fuel + calculate_fuel(fuel)
    } else {
        0.0
    };
}