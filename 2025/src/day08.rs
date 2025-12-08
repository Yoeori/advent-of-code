use std::collections::BTreeSet;

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Pos(isize, isize, isize);

impl Pos {
    fn dist(&self, other: &Self) -> isize {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let mut vs = value.splitn(3, ',').map(|v| v.parse().unwrap());
        Pos(vs.next().unwrap(), vs.next().unwrap(), vs.next().unwrap())
    }
}

pub(crate) struct Day08;
impl Puzzle for Day08 {
    type Part1 = u64;
    type Part2 = isize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let positions: Vec<Pos> = inp.lines().map(Pos::from).collect();

        let mut distances: BTreeSet<(isize, usize, usize)> = BTreeSet::new();

        for (i, p1) in positions.iter().enumerate() {
            for (j, p2) in positions.iter().enumerate().skip(i + 1) {
                distances.insert((p1.dist(p2), i, j));
            }
        }

        let mut part_one: Option<u64> = None;
        let mut last: Option<(usize, usize)> = None;

        let mut parent: Vec<usize> = (0..positions.len()).collect();

        for (x, &(_dist, i, j)) in distances.iter().enumerate() {
            // Find if common parent
            let parent_i = find(&mut parent, i);
            let parent_j = find(&mut parent, j);

            if parent_i != parent_j {
                // Connect the sets
                parent[parent_i] = parent_j;
                last = Some((i, j));
            }

            // Part one, should be 10 for example input
            if x == 1_000 {
                // Calculate size of sets
                let mut sizes = vec![0; parent.len()];
                for p in 0..parent.len() {
                    sizes[find(&mut parent, p)] += 1;
                }
                sizes.sort();

                part_one = Some(sizes.iter().rev().take(3).product());
            }
        }

        let (left, right) = last.unwrap();
        let part_two = positions[left].0 * positions[right].0;

        (part_one.unwrap(), part_two)
    }
}

// Simple set implementation to keep track of groups
fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}
