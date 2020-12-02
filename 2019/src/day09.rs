use std::fs;

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/09.txt").unwrap();
    let memory: Vec<i64> = file_contents.split(",").map(|x| x.parse().unwrap()).collect();

    super::intcode_computer::simulate_computer(memory);
}