use std::ops::{Add, Mul, Range};

use crate::puzzle::Puzzle;

pub(crate) struct Day06;
impl Puzzle for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let lines: Vec<&str> = inp.lines().collect();

        let ops: Vec<char> = lines
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|v| v.chars().next().unwrap())
            .collect();

        let value_lines = &lines[0..lines.len() - 1];

        // Part one
        let values: Vec<Vec<u64>> = value_lines
            .iter()
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut part_one = 0;

        for (x, &op) in ops.iter().enumerate() {
            let op = get_op(op);
            part_one += values.iter().map(|values| values[x]).reduce(op).unwrap();
        }

        // Part two
        let blocks: Vec<Vec<u64>> = find_blocks(lines.last().unwrap())
            .into_iter()
            .map(|block| read_block(&block, value_lines))
            .collect();

        let mut part_two = 0;

        for (block, &op) in blocks.iter().zip(&ops) {
            let op = get_op(op);
            part_two += block.iter().cloned().reduce(op).unwrap();
        }

        (part_one, part_two)
    }
}

fn get_op(op: char) -> fn(a: u64, b: u64) -> u64 {
    match op {
        '*' => u64::mul,
        '+' => u64::add,
        _ => unreachable!(),
    }
}

fn read_block(block: &Range<usize>, inp: &[&str]) -> Vec<u64> {
    let mut res = Vec::with_capacity(block.end - block.start);

    for i in block.clone() {
        let mut s = String::with_capacity(inp.len());
        for line in inp {
            s.push_str(&line[i..(i + 1)]);
        }

        res.push(s.trim().parse().unwrap_or(0));
    }

    res
}

fn find_blocks(inp: &str) -> Vec<Range<usize>> {
    let mut blocks: Vec<Range<usize>> = vec![];

    for (x, c) in inp.chars().enumerate().skip(1) {
        if c != ' ' {
            blocks.push(blocks.last().map(|x| x.end + 1).unwrap_or(0)..(x - 1));
        }
    }

    blocks.push(blocks.last().map(|x| x.end + 1).unwrap_or(0)..inp.len());

    blocks
}
