use std::{collections::HashMap, fs};

pub fn main() {
    let file = fs::read_to_string("puzzles/15.txt").unwrap();
    let starting_numbers: Vec<usize> = file.split(',').map(|x| x.parse().unwrap()).collect();

    let mut nums: HashMap<usize, usize> = HashMap::new();
    nums.extend(starting_numbers[..starting_numbers.len()-1].iter().enumerate().map(|(i, x)| (*x, i)));
    
    let mut last_num = *starting_numbers.last().unwrap();

    for i in starting_numbers.len()..30_000_000 {

        if i == 2020 {
            println!("Answer to exercise 1: {}", last_num);
        }

        let next_num = {
            if let Some(n) = nums.get(&last_num) {
                i - n - 1
            } else {
                0
            }
        };

        nums.insert(last_num, i-1);
        last_num = next_num;
    }

    println!("Answer to exercise 2: {}", last_num);   
}