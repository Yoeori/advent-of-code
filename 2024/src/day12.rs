use std::{collections::HashSet, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn get<T: Copy>(&'_ self, v: &[Vec<T>]) -> Option<T> {
        v.get(self.0).and_then(|v| v.get(self.1)).copied()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]

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

pub(crate) struct Day12;
impl Puzzle for Day12 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let map: Vec<Vec<char>> = inp.lines().map(|l| l.chars().collect()).collect();
        let mut positions: HashSet<Pos> = (0..map.len())
            .flat_map(|y| (0..map[y].len()).map(move |x| Pos(y, x)))
            .collect();
        let mut regions: Vec<HashSet<Pos>> = vec![];

        while let Some(&pos) = positions.iter().next() {
            let region = discover_region(&map, pos);

            for pos in region.iter() {
                positions.remove(pos);
            }

            regions.push(region);
        }

        let total_fence_cost: usize = regions.iter().map(fence_cost).sum::<usize>();
        let total_fence_cost_sides: usize = regions.iter().map(sides).sum::<usize>();

        (total_fence_cost, total_fence_cost_sides)
    }
}

fn discover_region(map: &[Vec<char>], start: Pos) -> HashSet<Pos> {
    let char = start.get(map);

    let mut found: HashSet<Pos> = HashSet::new();
    let mut queue: Vec<Pos> = Vec::new();

    found.insert(start);
    queue.push(start);

    while let Some(pos) = queue.pop() {
        for dir in DIRS {
            let new_pos = pos + dir;
            if !found.contains(&new_pos) && char == new_pos.get(map) {
                found.insert(new_pos);
                queue.push(new_pos);
            }
        }
    }

    found
}

fn fence_cost(region: &HashSet<Pos>) -> usize {
    let mut perimiter = 0;
    let area = region.len();

    for &pos in region.iter() {
        for dir in DIRS {
            if !region.contains(&(pos + dir)) {
                perimiter += 1;
            }
        }
    }

    perimiter * area
}

fn sides(region: &HashSet<Pos>) -> usize {
    let area = region.len();

    let mut fencing: HashSet<(Pos, Delta)> = HashSet::new();

    for &pos in region.iter() {
        for dir in DIRS {
            if !region.contains(&(pos + dir)) {
                fencing.insert((pos + dir, dir));
            }
        }
    }

    let mut sides = 0;
    while let Some(&pos) = fencing.iter().next() {
        sides += 1;
        bfs(&mut fencing, pos);
    }

    sides * area
}

fn bfs(map: &mut HashSet<(Pos, Delta)>, start: (Pos, Delta)) {
    map.remove(&start);

    let mut queue = vec![start];
    while let Some((pos, delta)) = queue.pop() {
        for dir in DIRS {
            let new_pos = pos + dir;
            if map.contains(&(new_pos, delta)) {
                map.remove(&(new_pos, delta));
                queue.push((new_pos, delta));
            }
        }
    }
}
