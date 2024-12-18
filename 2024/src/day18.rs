use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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

const DELTA: [Delta; 4] = [Delta(0, 1), Delta(0, -1), Delta(1, 0), Delta(-1, 0)];

const MEMORY_SIZE: usize = 70;
const KILOBYTE: usize = 1024;

pub(crate) struct Day18;
impl Puzzle for Day18 {
    type Part1 = usize;
    type Part2 = String;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let blocks: Vec<Pos> = inp
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| Pos(x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        let part_one = bfs(
            &set(&blocks, 1024),
            Pos(0, 0),
            Pos(MEMORY_SIZE, MEMORY_SIZE),
        )
        .unwrap();

        let mut min: usize = KILOBYTE; // We know that part 1 gives us a valid answer
        let mut max: usize = blocks.len();

        while min <= max {
            let mid = (min + max) / 2;
            let res = bfs(&set(&blocks, mid), Pos(0, 0), Pos(MEMORY_SIZE, MEMORY_SIZE));

            if res.is_some() {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }

        let pos: &Pos = &blocks[max];

        (part_one, format!("{},{}", pos.0, pos.1))
    }
}

fn set(blocks: &[Pos], n: usize) -> HashSet<Pos> {
    blocks.iter().take(n).copied().collect::<HashSet<Pos>>()
}

fn bfs(blocks: &HashSet<Pos>, start: Pos, end: Pos) -> Option<usize> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        for delta in DELTA {
            let new_pos = pos + delta;
            if !blocks.contains(&new_pos) && new_pos.0 <= MEMORY_SIZE && new_pos.1 <= MEMORY_SIZE {
                queue.push_back((new_pos, dist + 1));
            }
        }
    }

    None
}
