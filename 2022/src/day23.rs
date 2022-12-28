use std::collections::{HashMap, HashSet};

type Pos = (isize, isize);
const DELTAS: &[&[Pos]] = &[&[(0, -1), (-1, -1), (1, -1)], &[(0, 1), (-1, 1), (1, 1)], &[(-1, 0), (-1, 1), (-1, -1)], &[(1, 0), (1, 1), (1, -1)]];

pub fn main() {
    let mut elves: HashSet<Pos> = include_str!("../puzzles/23.txt").lines().enumerate()
        .map(|(y, l)| l.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x, _)| (x as isize, y as isize)))
        .flatten().collect();

    let mut changed = true;
    let mut i = 0;
    while changed {

        (elves, changed) = round(elves, i);

        if i == 9 {
            let minx = *elves.iter().map(|(x, _)| x).min().unwrap();
            let maxx = *elves.iter().map(|(x, _)| x).max().unwrap();
            let miny = *elves.iter().map(|(_, y)| y).min().unwrap();
            let maxy = *elves.iter().map(|(_, y)| y).max().unwrap();
        
            println!("Exercise 1: {}", (maxx - minx + 1) * (maxy - miny + 1) - elves.len() as isize);
        }

        i += 1;
    }

    println!("Exercise 2: {}", i);
}

#[allow(dead_code)]
fn print(elves: &HashSet<Pos>) {
    let minx = *elves.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *elves.iter().map(|(x, _)| x).max().unwrap();
    let miny = *elves.iter().map(|(_, y)| y).min().unwrap();
    let maxy = *elves.iter().map(|(_, y)| y).max().unwrap();

    for y in miny..=maxy {
        for x in minx..=maxx {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn round(mut elves: HashSet<Pos>, n: usize) -> (HashSet<Pos>, bool) {
    let mut proposals: HashMap<Pos, Pos> = HashMap::with_capacity(elves.len());
    let mut proposed_positions: HashMap<Pos, usize> = HashMap::with_capacity(elves.len());

    // First half
    'outer: for &(x, y) in elves.iter() {

        if [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)].iter().filter(|(dx, dy)| elves.contains(&(x + dx, y + dy))).next().is_none() {
            continue 'outer;
        }

        'middle: for i in 0..(DELTAS.len()) {
            for (dx, dy) in DELTAS[(n + i) % DELTAS.len()] {
                if elves.contains(&(x + dx, y + dy)) {
                    continue 'middle;
                }
            }

            // We propose this position
            let (dx, dy) = DELTAS[(n + i) % DELTAS.len()][0];
            proposals.insert((x, y), (x + dx, y + dy));
            *proposed_positions.entry((x + dx, y + dy)).or_insert(0) += 1;

            continue 'outer;
        }

    }

    // Second half
    let mut new_elves: HashSet<Pos> = HashSet::with_capacity(elves.len());
    let mut changed = false;

    for elve in elves.drain() {
        if let Some(&proposal) = proposals.get(&elve) {
            if *proposed_positions.get(&proposal).unwrap() == 1 {
                new_elves.insert(proposal);
                changed = true;
            } else {
                new_elves.insert(elve);
            }
        } else {
            new_elves.insert(elve);
        }
    }

    (new_elves, changed)
}