use std::fs;
use std::collections::HashSet;

pub fn main() {

    let file = fs::read_to_string("puzzles/6.txt").unwrap();

    let groups: Vec<&str> = file.split("\n\n").collect();
    println!("Answer to exercise 1: {}", groups.iter().map(|x| x.split("\n").fold(HashSet::new(), 
        |mut set, l| {
            set.extend(l.chars()); 
            set
        }
    ).len()).sum::<usize>());

    println!("Answer to exercise 2: {}", groups.iter().map(|x| x.split("\n").map(|x| x.chars().collect::<HashSet<char>>()).fold(None,
        |set: Option<HashSet<char>>, l| {
            set.map(|other| other.intersection(&l).map(|x| *x).collect()).or(Some(l))
        }
    ).unwrap().len()).sum::<usize>());

}