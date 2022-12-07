use std::{fs, collections::HashSet};

pub fn main() {
    let file = fs::read_to_string("puzzles/3.txt").unwrap();

    let mut total = 0;
    for line in file.lines() {
        let comp1: HashSet<char> = line[0..(line.len()/2)].chars().collect();
        let comp2: HashSet<char> = line[(line.len()/2)..].chars().collect();

        let j = *comp1.intersection(&comp2).next().unwrap();
        total += match j {
            'a'..='z' => j as u32 - 96,
            'A'..='Z' => j as u32 - 64 + 26,
            _ => panic!()
        }
    }

    println!("Exercise 1: {}", total);


    let lines: Vec<&str> = file.lines().collect();

    total = 0;
    for lns in lines.chunks(3) {
        let comp1: HashSet<char> = lns[0].chars().collect();
        let comp2: HashSet<char> = lns[1].chars().collect();
        let comp3: HashSet<char> = lns[2].chars().collect();
        let j = *comp1.intersection(&comp2).map(|x| *x).collect::<HashSet<char>>().intersection(&comp3).next().unwrap();

        total += match j {
            'a'..='z' => j as u32 - 96,
            'A'..='Z' => j as u32 - 64 + 26,
            _ => panic!()
        }

    }

    println!("Exercise 2: {}", total);
    
}