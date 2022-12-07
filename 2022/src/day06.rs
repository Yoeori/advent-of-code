use std::collections::HashSet;

fn signal_for_length(inp: &str, length: usize) -> Option<usize> {
    for i in 0..(inp.len() - length) {
        if inp[i..(i+length)].chars().collect::<HashSet<char>>().len() == length {
            return Some(i + length);
        }
    }
    None
}

pub fn main() {
    let signal = include_str!("../puzzles/6.txt");
    println!("Exercise 1: {}", signal_for_length(&signal, 4).unwrap());
    println!("Exercise 2: {}", signal_for_length(&signal, 14).unwrap()); 
}