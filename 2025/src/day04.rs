use std::{collections::HashSet, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Delta(isize, isize);

impl Add<Delta> for Pos {
    type Output = Pos;

    fn add(self, rhs: Delta) -> Self::Output {
        Pos(
            ((self.0 as isize) + rhs.0) as usize,
            ((self.1 as isize) + rhs.1) as usize,
        )
    }
}

const DELTA: &[Delta] = &[
    Delta(0, 1),
    Delta(0, -1),
    Delta(1, 0),
    Delta(-1, 0),
    Delta(1, 1),
    Delta(1, -1),
    Delta(-1, 1),
    Delta(-1, -1),
];

pub(crate) struct Day04;
impl Puzzle for Day04 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut papers: HashSet<Pos> = inp
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'@')
                    .map(move |(x, _)| Pos(x, y))
            })
            .collect();

        let part_one = filter_paper(&mut papers);
        let mut part_two = part_one;

        loop {
            let count = filter_paper(&mut papers);
            part_two += count;

            if count == 0 {
                break;
            }
        }

        (part_one, part_two)
    }
}

fn filter_paper(papers: &mut HashSet<Pos>) -> usize {
    let mut to_remove: HashSet<Pos> = HashSet::new();

    for &paper in papers.iter() {
        if DELTA
            .iter()
            .filter(|&&delta| papers.contains(&(paper + delta)))
            .count()
            < 4
        {
            to_remove.insert(paper);
        }
    }

    for paper in &to_remove {
        papers.remove(paper);
    }

    to_remove.len()
}
