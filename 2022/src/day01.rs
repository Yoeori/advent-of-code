use std::fs;

pub fn main() {
    let file = fs::read_to_string("puzzles/1.txt").unwrap();

    let mut calories: Vec<usize> = file.split("\n\n").map(|lines| lines.split("\n").map(|x| x.parse::<usize>().unwrap()).sum()).collect();
    calories.sort_by(|c1, c2| c2.cmp(c1));

    println!("Solution 1: {}", calories[0]);
    println!("Solution 2: {}", calories[0] + calories[1] + calories[2]);
}