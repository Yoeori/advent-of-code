use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

// type AdjacencyList = Vec<Vec<bool>>;

fn hash(name: &str) -> usize {
    let mut ch = name.chars();
    (ch.next().unwrap() as usize - 97) * 26 + (ch.next().unwrap() as usize - 97)
}

const SIZE: usize = 26 * 26 + 26;
type Adjl = [[bool; SIZE]; SIZE];

pub(crate) struct Day23;
impl Puzzle for Day23 {
    type Part1 = usize;
    type Part2 = String;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut adjl: Adjl = [[false; SIZE]; SIZE];

        let mut nodes: HashSet<&str> = HashSet::new();

        for (one, two) in inp.lines().map(|l| l.split_once('-').unwrap()) {
            nodes.insert(one);

            nodes.insert(two);

            adjl[hash(one)][hash(two)] = true;
            adjl[hash(two)][hash(one)] = true;
        }

        // nodes, with t-nodes first
        let mut nodes_vec: Vec<&str> = Vec::with_capacity(nodes.len());

        for &node in nodes.iter().filter(|x| x.starts_with('t')) {
            nodes_vec.push(node);
        }

        for &node in nodes.iter().filter(|x| !x.starts_with('t')) {
            nodes_vec.push(node);
        }

        // Part one
        // We count all cliques, where at least one node starts with 't'
        let mut count: usize = 0;

        for (x, &n1) in nodes_vec
            .iter()
            .enumerate()
            .take_while(|(_, x)| x.starts_with('t'))
        {
            for (y, &n2) in nodes_vec.iter().enumerate().skip(x + 1) {
                for &n3 in nodes_vec.iter().skip(y + 1) {
                    if is_clique(&[n1, n2, n3], &adjl) {
                        count += 1;
                    }
                }
            }
        }

        // Part two: find all biggest cliques using bron-kerbosch algorithm
        let node_map: HashMap<&str, usize> =
            nodes_vec.iter().enumerate().map(|(x, &y)| (y, x)).collect();
        let mut neighbours: Vec<NodeSet> = vec![NodeSet::default(); nodes.len()];

        for (one, two) in inp.lines().map(|l| l.split_once('-').unwrap()) {
            neighbours[node_map[one]] =
                neighbours[node_map[one]].or(&NodeSet::from_hash(node_map[two]));
            neighbours[node_map[two]] =
                neighbours[node_map[two]].or(&NodeSet::from_hash(node_map[one]));
        }

        let mut max_cliques = vec![];
        bron_kerbosch(
            NodeSet::default(),
            NodeSet::filled(nodes.len()),
            NodeSet::default(),
            &neighbours,
            &mut max_cliques,
        );

        // Find biggest clique
        let mut max_clique = max_cliques[0].hashes();

        for clique in max_cliques.iter().skip(1) {
            let hashes = clique.hashes();

            if hashes.len() > max_clique.len() {
                max_clique = hashes;
            }
        }

        // Convert to names
        let mut clique: Vec<&str> = max_clique.iter().map(|&x| nodes_vec[x]).collect();
        clique.sort();
        
        (count, clique.join(","))
    }
}

fn is_clique(nodes: &[&str], adjl: &Adjl) -> bool {
    for n1 in nodes {
        for n2 in nodes {
            if !(n1 == n2 || adjl[hash(n1)][hash(n2)]) {
                return false;
            }
        }
    }

    true
}

fn bron_kerbosch(
    cur_clique: NodeSet,
    mut left: NodeSet,
    mut right: NodeSet,
    neighbours: &Vec<NodeSet>,
    found_cliques: &mut Vec<NodeSet>,
) {
    if left.is_empty() && right.is_empty() {
        found_cliques.push(cur_clique);
    }

    for node in left.hashes() {
        let set = &NodeSet::from_hash(node);
        bron_kerbosch(
            cur_clique.or(set),
            left.intersect(&neighbours[node]),
            right.intersect(&neighbours[node]),
            neighbours,
            found_cliques,
        );
        left = left.remove(set);
        right = right.or(set);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct NodeSet([u128; 5]);

impl NodeSet {
    fn remove(&self, other: &Self) -> Self {
        Self::from_iterator(self.0.iter().zip(other.0).map(|(x, y)| x - y))
    }

    fn intersect(&self, other: &Self) -> Self {
        Self::from_iterator(self.0.iter().zip(other.0).map(|(x, y)| x & y))
    }

    fn or(&self, other: &Self) -> Self {
        Self::from_iterator(self.0.iter().zip(other.0).map(|(x, y)| x | y))
    }

    fn filled(mut n: usize) -> Self {
        let mut s: [u128; 5] = [0, 0, 0, 0, 0];

        for x in s.iter_mut() {
            if n >= 128 {
                *x = u128::MAX;
                n -= 128;
            } else if n > 0 {
                *x = (1 << n) - 1;
                n = 0;
            }
        }

        NodeSet(s)
    }

    fn from_iterator(mut it: impl Iterator<Item = u128>) -> Self {
        NodeSet([
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        ])
    }

    fn from_hash(hash: usize) -> Self {
        let mut s = [0, 0, 0, 0, 0];
        s[hash / 128] = 1 << (hash % 128);
        NodeSet(s)
    }

    fn hashes(&self) -> Vec<usize> {
        let mut res = vec![];

        for (i, &x) in self.0.iter().enumerate() {
            let mut cur = x;
            let mut y = i * 128;
            while cur != 0 {
                if cur % 2 == 1 {
                    res.push(y);
                }
                y += 1;
                cur >>= 1;
            }
        }

        res
    }

    fn is_empty(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }
}
