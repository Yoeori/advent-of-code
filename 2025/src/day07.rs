use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

pub(crate) struct Day07;
impl Puzzle for Day07 {
    type Part1 = u32;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut lines = inp.lines();
        
        let start = lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, v)| v == &'S')
            .unwrap()
            .0;

        let splitter_lines: Vec<HashSet<usize>> = lines
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'^')
                    .map(|(i, _)| i)
                    .collect::<HashSet<usize>>()
            })
            .filter(|set| !set.is_empty())
            .collect();

        let (final_row, split_count) = splitter_lines.iter().fold(
            (HashMap::<usize, u64>::from([(start, 1)]), 0),
            |(cur, splits), item| {
                let (res, res_splits) = exec_split(&cur, item);
                (res, res_splits + splits)
            },
        );

        (split_count, final_row.values().sum::<u64>())
    }
}

fn exec_split(
    row: &HashMap<usize, u64>,
    splitters: &HashSet<usize>,
) -> (HashMap<usize, u64>, u32) {
    let mut res= HashMap::new();
    let mut splits = 0;

    for (pos, count) in row {
        if splitters.contains(pos) {
            *res.entry(pos - 1).or_insert(0) += count;
            *res.entry(pos + 1).or_insert(0) += count;
            splits += 1;
        } else {
            *res.entry(*pos).or_insert(0) += count;
        }
    }

    (res, splits)
}
