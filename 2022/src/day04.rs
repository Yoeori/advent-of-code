use std::fs;

pub fn main() {
    let file = fs::read_to_string("puzzles/4.txt").unwrap();

    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in file.lines() {
        let (l1, l2) = line.split_once(',').unwrap();
        let (a1, a2) = l1.split_once('-').unwrap();
        let a = (a1.parse::<usize>().unwrap())..=(a2.parse().unwrap());
        let (b1, b2) = l2.split_once('-').unwrap();
        let b = (b1.parse::<usize>().unwrap())..=(b2.parse().unwrap());

        if (a.start() <= b.start() && a.end() >= b.end()) || (b.start() <= a.start() && b.end() >= a.end()) {
            total_1 += 1;
            total_2 += 1;
        } else if (a.start() >= b.start() && a.start() <= b.end()) || (b.start() >= a.start() && b.start() <= a.end()) {
            total_2 += 1;
        }
    }

    println!("Exercise 1: {}", total_1);
    println!("Exercise 2: {}", total_2);
    
}