use std::{collections::HashSet, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

const DIRS: [Delta; 4] = [Delta(-1, 0), Delta(0, 1), Delta(1, 0), Delta(0, -1)];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Dir(u8);
impl Dir {
    fn turn_right(&mut self) {
        self.0 += 1;
        self.0 %= 4;
    }

    fn delta(&self) -> Delta {
        DIRS[self.0 as usize]
    }
}

fn read_map(inp: &str) -> (Vec<Vec<bool>>, Pos) {
    let mut map: Vec<Vec<bool>> = vec![];
    let mut initial_pos: Pos = Pos(0, 0);

    for (y, line) in inp.lines().enumerate() {
        let mut map_line = vec![];

        for (x, c) in line.chars().enumerate() {
            map_line.push(match c {
                '^' => {
                    initial_pos = Pos(y, x);
                    true
                }
                '.' => true,
                _ => false,
            });
        }

        map.push(map_line);
    }

    (map, initial_pos)
}
pub(crate) struct Day06;
impl Puzzle for Day06 {
    type Part1 = usize;
    type Part2 = i32;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (mut map, initial_pos) = read_map(inp);

        let mut dir = Dir(0);
        let mut pos = initial_pos;

        let mut visited: HashSet<Pos> = HashSet::new();

        loop {
            visited.insert(pos);

            let Pos(y, x): Pos = pos + dir.delta();

            if y >= map.len() || x >= map[0].len() {
                break;
            }

            if map[y][x] {
                pos = Pos(y, x)
            } else {
                dir.turn_right();
            }
        }

        let mut loop_count = 0;
        for &Pos(y, x) in &visited {
            map[y][x] = false;
            if is_loop(&map, initial_pos) {
                loop_count += 1;
            }
            map[y][x] = true;
        }

        (visited.len(), loop_count)
    }
}

fn is_loop(map: &[Vec<bool>], mut pos: Pos) -> bool {
    let mut dir = Dir(0);
    let mut visited: HashSet<(Pos, Dir)> = HashSet::new();

    loop {
        if visited.contains(&(pos, dir)) {
            return true;
        }

        visited.insert((pos, dir));

        let Pos(y, x): Pos = pos + dir.delta();

        if y >= map.len() || x >= map[0].len() {
            return false;
        }

        if map[y][x] {
            pos = Pos(y, x)
        } else {
            dir.turn_right();
        }
    }
}
