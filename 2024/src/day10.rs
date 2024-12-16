use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
};

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

const DIRS: [Delta; 4] = [Delta(0, 1), Delta(0, -1), Delta(1, 0), Delta(-1, 0)];

pub(crate) struct Day10;
impl Puzzle for Day10 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let map: Vec<Vec<u32>> = inp
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let trailheads: Vec<Pos> = map
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(move |(x, v)| if *v != 0 { None } else { Some(Pos(y, x)) })
            })
            .collect();

        let scores: Vec<HashMap<Pos, usize>> = trailheads
            .iter()
            .map(|&trailhead| find_trails(&map, trailhead))
            .collect();

        let score_sum = scores.iter().map(|score| score.len()).sum::<usize>();
        let trail_sum = scores
            .iter()
            .flat_map(|score| score.values())
            .sum::<usize>();

        (score_sum, trail_sum)
    }
}

fn find_trails(map: &[Vec<u32>], start: Pos) -> HashMap<Pos, usize> {
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(start);

    let mut ends: HashMap<Pos, usize> = HashMap::new();

    while let Some(pos) = queue.pop_front() {
        let height = map[pos.0][pos.1];

        if height == 9 {
            *ends.entry(Pos(pos.0, pos.1)).or_insert(0) += 1;
            continue;
        }

        for delta in DIRS {
            let new_pos = pos + delta;

            if new_pos.0 < map.len()
                && new_pos.1 < map[new_pos.0].len()
                && height + 1 == map[new_pos.0][new_pos.1]
            {
                queue.push_back(new_pos);
            }
        }
    }

    ends
}
