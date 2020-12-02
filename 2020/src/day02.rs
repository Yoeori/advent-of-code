use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Policy {
    atleast: usize,
    atmost: usize,
    letter: char,
    word: String
}

impl Policy {
    fn valid(&self) -> bool {
        let length = self.word.chars().filter(|&x| x == self.letter).count();
        length >= self.atleast && length <= self.atmost
    }

    fn valid_strict(&self) -> bool {
        (self.word.chars().nth(self.atleast - 1).unwrap() == self.letter) ^ (self.word.chars().nth(self.atmost - 1).unwrap() == self.letter)
    }
}

pub fn main() {
    let file = fs::read_to_string("puzzles/2.txt").unwrap();

    let re = Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();
    let policies: Vec<Policy> = re.captures_iter(&file).map(|cap| {
        Policy {
            atleast: cap[1].parse().unwrap(),
            atmost: cap[2].parse().unwrap(),
            letter: cap[3].parse().unwrap(),
            word: cap[4].to_string()
        }
    }).collect();

    println!("Answer to exercise 1: {}", policies.iter().filter(|policy| policy.valid()).count());
    println!("Answer to exercise 2: {}", policies.iter().filter(|policy| policy.valid_strict()).count());
}