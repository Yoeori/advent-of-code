use std::{collections::HashMap, fs};

pub fn main() {
    let file = fs::read_to_string("puzzles/01.txt").unwrap();

    let (mut l1, mut l2): (Vec<u32>, Vec<u32>) = file
        .lines()
        .map(|s| {
            let mut it = s.split("   ").map(|x| x.parse::<u32>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .unzip();

    l1.sort();
    l2.sort();

    let total_diff = l1
        .iter()
        .zip(l2.iter())
        .map(|(&v1, &v2)| v1.abs_diff(v2))
        .sum::<u32>();
    println!("Exercise 1: {}", total_diff);

    let mut counts: HashMap<u32, u32> = HashMap::new();
    for n in l2 {
        counts.entry(n).and_modify(|k| *k += 1).or_insert(1);
    }

    let similarity_score = l1
        .iter()
        .map(|v| v * counts.get(v).unwrap_or(&0))
        .sum::<u32>();
    println!("Exercise 2: {}", similarity_score);
}
