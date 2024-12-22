use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Pos(usize, usize);

impl Pos {
    fn get<T: Copy>(&'_ self, v: &[Vec<T>]) -> Option<T> {
        v.get(self.0).and_then(|v| v.get(self.1)).copied()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Delta(isize, isize);

impl Delta {
    fn dist(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

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

pub(crate) struct Day20;
impl Puzzle for Day20 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut map: Vec<Vec<bool>> = vec![];
        let mut start_position = None;
        let mut end_position = None;

        for (y, line) in inp.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());

            for (x, c) in line.chars().enumerate() {
                row.push(c != '#');

                match c {
                    'S' => {
                        start_position = Some(Pos(y, x));
                    }
                    'E' => {
                        end_position = Some(Pos(y, x));
                    }
                    _ => {}
                }
            }

            map.push(row);
        }

        let (dist, path) = flood(&map, start_position.unwrap(), end_position.unwrap());

        let part_one = count_cheats_for_distance(&dist, &path, 2);
        let part_two = count_cheats_for_distance(&dist, &path, 20);

        (part_one, part_two)
    }
}

fn flood(map: &[Vec<bool>], start: Pos, end: Pos) -> (HashMap<Pos, usize>, Vec<Pos>) {
    let mut queue: VecDeque<(Pos, Option<Pos>, usize)> = VecDeque::new();
    queue.push_back((end, None, 0));

    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new(); // For reconstructing the initial path


    while let Some((pos, prev_pos, cost)) = queue.pop_front() {
        if dist.contains_key(&pos) {
            continue;
        }

        dist.insert(pos, cost);
        if let Some(prev_pos) = prev_pos {
            prev.insert(pos, prev_pos);
        }

        for delta in DELTA {
            let new_pos = pos + delta;
            if new_pos.0 < map.len() && new_pos.1 < map[0].len() && new_pos.get(map).unwrap() {
                queue.push_back((new_pos, Some(pos), cost + 1));
            }
        }
    }

    let path = restore_path(&prev, start);

    (dist, path)
}

fn restore_path(prev: &HashMap<Pos, Pos>, mut cur: Pos) -> Vec<Pos> {
    let mut path: Vec<Pos> = vec![];
    path.push(cur);

    while let Some(&pos) = prev.get(&cur) {
        path.push(pos);
        cur = pos;
    }

    path
}

const MINIMUM_SAVED_TIME: usize = 100;

fn count_cheats_for_distance(dist: &HashMap<Pos, usize>, path: &[Pos], cheat: isize) -> usize {
    let original_dist = path.len();
    let mut count = 0;

    for (initial_dist, &pos) in path.iter().enumerate() {

        // Generate all possible cheats from this position
        for y in -cheat..=cheat {
            for x in (-cheat + y.abs())..=(cheat - y.abs()) {
                let delta = Delta(y, x);
                let new_pos = pos + delta;

                if let Some(distance) = dist.get(&new_pos) {
                    let total_dist = distance + initial_dist + delta.dist();
                    if total_dist < original_dist && original_dist - total_dist >= MINIMUM_SAVED_TIME {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}