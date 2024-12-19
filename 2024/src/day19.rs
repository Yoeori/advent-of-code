use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

pub(crate) struct Day19;
impl Puzzle for Day19 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (patterns, flags) = inp.split_once("\n\n").unwrap();

        let patterns: HashSet<&str> = patterns.split(", ").collect();
        let flags: Vec<&str> = flags.split('\n').collect();
        let max_pattern_length = patterns.iter().map(|p| p.len()).max().unwrap();

        let mut lut: HashMap<&str, usize> = HashMap::new();

        let part_one = flags
            .iter()
            .filter(|&&flag| check(flag, &patterns, max_pattern_length, &mut lut) != 0)
            .count();
        let part_two = flags
            .iter()
            .map(|&flag| check(flag, &patterns, max_pattern_length, &mut lut))
            .sum::<usize>();

        (part_one, part_two)
    }
}

fn check<'a>(
    flag: &'a str,
    patterns: &HashSet<&'a str>,
    max_pattern_length: usize,
    lut: &mut HashMap<&'a str, usize>,
) -> usize {
    if flag.is_empty() {
        return 1;
    }

    if let Some(&b) = lut.get(flag) {
        return b;
    }

    let res = (1..=max_pattern_length.min(flag.len()))
        .filter(|&i| patterns.contains(&flag[0..i]))
        .map(|i| check(&flag[i..], patterns, max_pattern_length, lut))
        .sum();

    lut.insert(flag, res);
    
    res
}
