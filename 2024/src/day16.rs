use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

impl Add<Delta> for Pos {
    type Output = Pos;

    fn add(self, rhs: Delta) -> Self::Output {
        Pos(
            ((self.0 as isize) + rhs.0) as usize,
            ((self.1 as isize) + rhs.1) as usize,
        )
    }
}

const DIRS: [Delta; 4] = [Delta(0, 1), Delta(-1, 0), Delta(0, -1), Delta(1, 0)];

pub(crate) struct Day16;
impl Puzzle for Day16 {
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

        dijkstra(&map, start_position.unwrap(), end_position.unwrap())
    }
}

fn walk(
    found: &mut HashSet<(Pos, Delta)>,
    prev: &HashMap<(Pos, Delta), Vec<(Pos, Delta)>>,
    cur: (Pos, Delta),
) {
    if found.contains(&cur) {
        return;
    }

    found.insert(cur);

    for pos in prev.get(&cur).unwrap_or(&Vec::new()) {
        walk(found, prev, *pos);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Entry(usize, Pos, Delta, Option<(Pos, Delta)>); // dist, pos, delta, prev

fn dijkstra(map: &[Vec<bool>], start: Pos, end: Pos) -> (usize, usize) {
    let mut visited: HashMap<(Pos, Delta), usize> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<Entry>> = BinaryHeap::new();
    let mut prev: HashMap<(Pos, Delta), Vec<(Pos, Delta)>> = HashMap::new();

    queue.push(Reverse(Entry(0, start, DIRS[0], None)));

    while let Some(Reverse(Entry(score, pos, delta, entry_prev))) = queue.pop() {
        let best_score = *visited.get(&(pos, delta)).unwrap_or(&usize::MAX);
        if score > best_score {
            continue;
        }

        visited.insert((pos, delta), score);

        // We keep track of _all_ possible previous positions
        if let Some(entry_prev) = entry_prev {
            prev.entry((pos, delta)).or_default().push(entry_prev);
        }

        // Move forward
        let new_pos = pos + delta;
        if let Some(true) = new_pos.get(map) {
            queue.push(Reverse(Entry(
                score + 1,
                new_pos,
                delta,
                Some((pos, delta)),
            )));
        }

        // Turn -90deg / +90deg
        let idx = DIRS.iter().position(|&v| v == delta).unwrap();
        queue.push(Reverse(Entry(
            score + 1000,
            pos,
            DIRS[(idx - 1).rem_euclid(DIRS.len())],
            Some((pos, delta)),
        )));
        queue.push(Reverse(Entry(
            score + 1000,
            pos,
            DIRS[(idx + 1).rem_euclid(DIRS.len())],
            Some((pos, delta)),
        )));
    }

    let mut found = HashSet::new();
    let best_score = *DIRS
        .iter()
        .filter_map(|&dir| visited.get(&(end, dir)))
        .min()
        .unwrap();

    // Walk from all dirs that have achieved the best score
    for &dir in DIRS.iter() {
        if let Some(&score) = visited.get(&(end, dir)) {
            if score == best_score {
                walk(&mut found, &prev, (end, dir));
            }
        }
    }

    let found_positions: HashSet<Pos> = found.into_iter().map(|(pos, _)| pos).collect();
    (best_score, found_positions.len())
}
