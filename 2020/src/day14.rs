use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let file = fs::read_to_string("puzzles/14.txt").unwrap();

    let re = Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<val>\d+)$").unwrap();

    let mut mem: HashMap<usize, u64> = HashMap::with_capacity(file.lines().count());
    let mut memv2: HashMap<usize, u64> = HashMap::with_capacity(file.lines().count() * 2usize.pow(8));
    let mut mask = vec![];

    for line in file.lines() {
        if line.starts_with("mask") {
            mask = line.split(' ').nth(2).unwrap().chars().collect();
        } else {
            let caps = re.captures(line).unwrap();
            let index = caps.name("index").unwrap().as_str().parse::<usize>().unwrap();
            let val = caps.name("val").unwrap().as_str().parse().unwrap();

            // Part 1
            mem.insert(index,  mask_apply(&mask, val));

            // part 2
            memv2.extend(mask_applyv2(&mask, index).iter().map(|&mask| (mask, val)))
        }
    }

    println!("Solution to exercise 1: {}", mem.values().sum::<u64>());
    println!("Solution to exercise 2: {}", memv2.values().sum::<u64>());
}

fn mask_apply(mask: &Vec<char>, num: u64) -> u64 {
    let mut res = 0u64;
    for (loc, m) in mask.iter().enumerate() {
        res = res << 1;

        if m == &'X' {
            res += (num >> (35 - loc)) & 0b1;
        } else if m == &'1' {
            res += 1;
        }
    }

    res
}

fn mask_applyv2(mask: &Vec<char>, ind: usize) -> Vec<usize> {
    let mut mem_locations: Vec<usize> = vec![0];

    for (loc, m) in mask.iter().enumerate() {
        if m == &'X' {
            mem_locations = mem_locations.into_iter().map(|ind| vec![ind << 1, (ind << 1) + 1]).flatten().collect()
        } else {
            for mem_location in mem_locations.iter_mut() {
                *mem_location = (*mem_location << 1) + {
                    if m == &'1' {
                        1
                    } else {
                        (ind >> (35 - loc)) & 0b1
                    }
                }
            }
        }
    }

    mem_locations
}