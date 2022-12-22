use std::{collections::{HashSet, HashMap}, cmp};

#[derive(Debug)]
struct Shape {
    data: Vec<(i64, i64)>
}

impl Shape {
    fn from_string(inp: &str) -> Self {
        Shape {
            data: inp.rsplit('\n').enumerate().map(|(y, l)| l.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x, _)| (y as i64, x as i64))).flatten().collect()
        } 
    }

    fn collides(&self, (y, x): (i64, i64), map: &HashSet<(i64, i64)>) -> bool {
        for &(dy, dx) in &self.data {
            let np = (y + dy, x + dx);
            if map.contains(&np) || np.1 < 0 || np.1 > 6 {
                return true;
            }
        }
        false
    }

    fn fall(&self, map: &HashSet<(i64, i64)>, mut pos: (i64, i64), dir: &Vec<char>, dir_idx: &mut usize) -> (i64, i64) {
        loop {
            // Move right/left
            if dir[*dir_idx] == '>' {
                if !self.collides((pos.0, pos.1 + 1), &map) {
                    pos = (pos.0, pos.1 + 1);
                }
            } else {
                if !self.collides((pos.0, pos.1 - 1), &map) {
                    pos = (pos.0, pos.1 - 1);
                }
            }

            *dir_idx += 1;
            *dir_idx %= dir.len();

            // Move down
            if !self.collides((pos.0 - 1, pos.1), &map) {
                pos = (pos.0 - 1, pos.1);
            } else {
                break;
            }
        }

        pos
    }
}

pub fn main() {
    let dir: Vec<char> = include_str!("../puzzles/17.txt").chars().collect();
    let shapes: Vec<Shape> = include_str!("day17_shapes.txt").split("\n\n").map(|s| Shape::from_string(s)).collect();
    let mut map: HashSet<(i64, i64)> = HashSet::new();

    // Add floor to map:
    for x in 0..7 {
        map.insert((0, x));
    }

    let mut max_height = 0;
    let mut dir_idx = 0;
    let mut shape_idx = 0;

    let mut history: HashMap<usize, (usize, i64)> = HashMap::new();

    let mut simulate_til = None;
    let mut extra_rounds = None;

    for i in 1.. {
        let (y, x) = shapes[shape_idx].fall(&map, (max_height + 4, 2), &dir, &mut dir_idx);

        // Fix shape
        for (dy, dx) in &shapes[shape_idx].data {
            map.insert((y + dy, x + dx));
            max_height = cmp::max(y + dy, max_height);
        }

        shape_idx += 1;
        shape_idx %= shapes.len();

        if i == 2022 {
            println!("Exercise 1: {}", max_height);
        }

        if i > 10_000 && shape_idx == 0 {
            if let Some((old_i, old_height)) = history.get(&dir_idx) {
                let (di, d_height) = (i - old_i, max_height - old_height);

                let needed = 1000_000_000_000 - i;
                let rounds = needed / di;
                let left = needed % di;

                simulate_til = Some(i + left);
                extra_rounds = Some(d_height * rounds as i64);
            } else {
                history.insert(dir_idx, (i, max_height));
            }
        }

        if let Some(r) = simulate_til {
            if i == r {
                println!("Exercise 2: {}", max_height + extra_rounds.unwrap());
                break;
            }
        }
    }

}