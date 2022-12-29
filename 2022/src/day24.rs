use std::collections::{VecDeque, HashSet};

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug)]
struct Blizzard {
    pos: (usize, usize),
    dir: char,
}

impl Blizzard {
    fn pos_at_time(&self, t: usize, width: usize, height: usize) -> (usize, usize) {
        match self.dir {
            '>' => ((self.pos.0 + t - 1) % (width - 2) + 1, self.pos.1),
            '<' => (((self.pos.0 as isize - 1 - t as isize).rem_euclid(width as isize - 2) + 1) as usize, self.pos.1),
            'v' => (self.pos.0, (self.pos.1 + t - 1) % (height - 2) + 1),
            '^' => (self.pos.0, ((self.pos.1 as isize - 1 - t as isize).rem_euclid(height as isize - 2) + 1) as usize),
            _ => panic!()
        }
    }
}

pub fn main() {
    let map: Vec<&str> = include_str!("../puzzles/24.txt").lines().collect();

    let blizzards: Vec<Blizzard> = map
        .iter().enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| ['<', '>', 'v', '^'].contains(c))
                .map(move |(x, c)| Blizzard {
                    pos: (x, y),
                    dir: c,
                })
        }).flatten().collect();

    let width = map[0].len();
    let height = map.len();

    let mut maps: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; width]; height]; lcm(width - 2, height - 2)];

    for t in 0..maps.len() {
        for x in 0..width {
            maps[t][0][x] = true;
            maps[t][height - 1][x] = true;
        }

        for y in 0..height {
            maps[t][y][0] = true;
            maps[t][y][width - 1] = true;
        }

        for blizzard in blizzards.iter() {
            let (x, y) = blizzard.pos_at_time(t, width, height);
            maps[t][y][x] = true;
        }

        maps[t][0][1] = false;
        maps[t][height - 1][width - 2] = false;
    }

    let start_finish = bfs(&maps, (1, 0), (width - 2, height - 1), 0).unwrap();
    println!("Exercise 1: {}", start_finish);

    let finish_start = bfs(&maps, (width - 2, height - 1), (1, 0),  start_finish - 1).unwrap();
    let start_finish_2 = bfs(&maps, (1, 0), (width - 2, height - 1), finish_start - 1).unwrap();
    println!("Exercise 2: {}", start_finish_2);
}

fn bfs(maps: &Vec<Vec<Vec<bool>>>, start: (usize, usize), goal: (usize, usize), start_t: usize) -> Option<usize> {
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((start, start_t));

    let mut cur_t = start_t;
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();

    while let Some(((x, y), t)) = queue.pop_front() {
        if cur_t != t {
            discovered.drain();
            cur_t = t;
        }

        if (x, y) == goal {
            return Some(t);
        }

        let map = &maps[(t + 1) % maps.len()];

        'inner: for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)] {
            if !(x > 0 || dx >= 0) || !(y > 0 || dy >= 0) || (x as isize + dx) as usize >= map[0].len() || (y as isize + dy) as usize >= map.len() {
                continue 'inner;
            }

            let newpos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if !map[newpos.1][newpos.0] && !discovered.contains(&newpos) {
                discovered.insert(newpos);
                queue.push_back((newpos, t + 1));
            }
        }
    }
    
    None
}