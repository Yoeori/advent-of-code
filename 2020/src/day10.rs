use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn main() {
    let file = fs::read_to_string("puzzles/10.txt").unwrap();
    
    let numbers: HashSet<u64> = file.split("\n").map(|x| x.parse().unwrap()).collect();
    let max = *numbers.iter().max().unwrap();

    let mut cur = 0;
    let mut n3 = 1; // Last step is always 3
    let mut n1 = 0;

    while cur < max {
        if numbers.contains(&(cur + 1)) {
            cur += 1;
            n1 += 1;
        } else if numbers.contains(&(cur + 2)) {
            cur += 2;
            println!("Hello world!");
        } else if numbers.contains(&(cur + 3)) {
            cur += 3;
            n3 += 1;
        }
    }

    println!("Answer to exercise 1: {}", n3 * n1);

    let mut lookup: HashMap<u64, u64> = HashMap::new();
    println!("Answer to exercise 2: {}", count(&numbers, &mut lookup, 0, max));

}

fn count(set: &HashSet<u64>, lookup: &mut HashMap<u64, u64>, cur: u64, max: u64) -> u64 {
    if cur == max {
        return 1u64;
    } else if lookup.contains_key(&cur) {
        return *lookup.get(&cur).unwrap();
    }

    let mut total_ways = 0;
    for i in 1..=3 {
        if set.contains(&(cur + i)) {
            let res = count(set, lookup, cur + i, max);
            lookup.insert(cur + i, res);
            total_ways += res;
        } 
    }
    
    total_ways
}