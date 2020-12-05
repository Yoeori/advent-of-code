use std::fs;
use std::collections::HashSet;

pub fn main() {
    let file = fs::read_to_string("puzzles/5.txt").unwrap();

    let passess: HashSet<usize> = file.split("\n").map(seat_number).collect();
    println!("Answer to first exercise: {}", passess.iter().max().unwrap());

    let mut n = *passess.iter().min().unwrap();
    while passess.contains(&n) {
        n += 1;
    }
    println!("Answer to second exercise: {}", n);
}

fn bin_search_text(inp: &str, mut min: usize, mut max: usize, letter: char) -> usize {
    let mut inp = inp.chars();
    max += 1;

    while let Some(c) = inp.next() {
        if c == letter {
            max = (min+max) / 2;
        } else {
            min = (min+max) / 2;
        }
    }

    min
}

fn seat_number(inp: &str) -> usize {
    return bin_search_text(&inp[0..7], 0, 127, 'F') * 8 + bin_search_text(&inp[7..10], 0, 7, 'L');
}