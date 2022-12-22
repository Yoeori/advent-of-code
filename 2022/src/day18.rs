use std::{collections::{HashMap, HashSet}, cmp, ops::Range};

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Water, Lava
}

const DELTA: &[(i32, i32, i32)] = &[(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

pub fn main() {
    let mut map: HashMap<(i32, i32, i32), Block> = include_str!("../puzzles/18.txt").split('\n').map(|line| {
        let mut it = line.split(',').map(|x| x.parse().unwrap());
        ((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()), Block::Lava)
    }).collect();

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut count = 0;

    for &(x, y, z) in map.keys() {
        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
        min_z = cmp::min(min_z, z);
        max_z = cmp::max(max_z, z);

        for (dx, dy, dz) in DELTA {
            if !map.contains_key(&(x + dx, y + dy, z + dz)) {
                count += 1;
            }
        }
    }

    println!("Exercise 1: {}", count);

    // Expand water on the outside:
    let mut seen = HashSet::new();
    dfs(&mut map, (min_x - 1, min_y - 1, min_z - 1), (&((min_x - 1)..(max_x + 2)), &((min_y - 1)..(max_y + 2)), &((min_z - 1)..(max_z + 2))), &mut seen);

    let mut count = 0;
    for (x, y, z) in seen {
        for (dx, dy, dz) in DELTA {
            if let Some(&Block::Water) = map.get(&(x + dx, y + dy, z + dz)) {
                count += 1;
            }
        }
    }

    println!("Exercise 2: {}", count);


}

fn dfs(map: &mut HashMap<(i32, i32, i32), Block>, (x, y, z): (i32, i32, i32), (rx, ry, rz): (&Range<i32>, &Range<i32>, &Range<i32>), seen: &mut HashSet<(i32, i32, i32)>) {
    if let Some(Block::Lava) = map.get(&(x, y, z)) {
        seen.insert((x, y, z));
        return;
    }
    
    if map.contains_key(&(x, y, z)) || !rx.contains(&x) || !ry.contains(&y) || !rz.contains(&z) {
        return;
    }

    map.insert((x, y, z), Block::Water);

    for (dx, dy, dz) in DELTA {
        dfs(map, (x + dx, y + dy, z + dz), (rx, ry, rz), seen);
    }
}