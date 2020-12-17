use std::{hash::Hash, time::SystemTime};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cord4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize
}

trait Neighbours<T> {
    fn neighbours(&self) -> Vec<T>;
}

impl Neighbours<Cord4> for Cord4 {
    fn neighbours(&self) -> Vec<Cord4> {
        let mut res = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if !(z == 0 && y == 0 && x == 0 && w == 0) {
                            res.push(Cord4 {
                                x: self.x + x, y: self.y + y, z: self.z + z, w: self.w + w
                            });
                        }
                    }
                }
            }
        }

        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cord3 {
    x: isize,
    y: isize,
    z: isize
}

impl Neighbours<Cord3> for Cord3 {
    fn neighbours(&self) -> Vec<Cord3> {
        let mut res = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !(z == 0 && y == 0 && x == 0) {
                        res.push(Cord3 {
                            x: self.x + x, y: self.y + y, z: self.z + z
                        });
                    }
                }
            }
        }

        res
    }
}

pub fn main() {
    let simple_map: HashMap<usize, Vec<usize>> = include_str!("../puzzles/17.txt").split('\n').map(
        |l | l.chars().enumerate().filter(|(_, c)| *c == '#').map(|(x, _)| x).collect()
    ).enumerate().collect();

    let time = SystemTime::now();
    let map: HashSet<Cord3> = simple_map.iter().map(|(y, xs)| xs.iter().map(|x| Cord3 {
        z: 0,
        y: *y as isize,
        x: *x as isize
    }).collect::<HashSet<Cord3>>()).flatten().collect();

    println!("Solution to exercise 1: {}", simulate(map));

    let map: HashSet<Cord4> = simple_map.iter().map(|(y, xs)| xs.iter().map(|x| Cord4 {
        z: 0,
        w: 0,
        y: *y as isize,
        x: *x as isize
    }).collect::<HashSet<Cord4>>()).flatten().collect();

    println!("Solution to exercise 2: {}", simulate(map));
    println!("Took: {:?}", time.elapsed().unwrap());
}

fn simulate<M: Eq + Neighbours<M> + Clone + Hash>(mut map: HashSet<M>) -> usize {
    for _ in 0..6 {
        let mut count: HashMap<M, usize> = map.iter().map(|coord| (coord.clone(), 0)).collect();

        // Count neighbours for every position
        for c in &map {
            for neighbour in c.neighbours() {
                count.entry(neighbour).and_modify(|count| { *count += 1 }).or_insert(1);
            }
        }

        // Update map
        for (coordinate, c) in count {
            if map.contains(&coordinate) {
                if !(c == 3 || c == 2) {
                    map.remove(&coordinate);
                }
            } else {
                if c == 3 {
                    map.insert(coordinate);
                }
            }
        }
    }

    map.len()
}