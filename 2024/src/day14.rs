use std::collections::HashMap;

use regex::Regex;

use crate::puzzle::Puzzle;

const SIZE: (isize, isize) = (101, 103);

#[derive(Debug, Clone)]
struct Robot {
    pos: (isize, isize),
    delta: (isize, isize),
}

impl Robot {
    fn pos_at(&self, time: usize) -> (isize, isize) {
        (
            (self.pos.0 + self.delta.0 * (time as isize)).rem_euclid(SIZE.0),
            (self.pos.1 + self.delta.1 * (time as isize)).rem_euclid(SIZE.1),
        )
    }

    fn quadrant(&self, time: usize) -> usize {
        let (x, y) = self.pos_at(time);

        if x < SIZE.0 / 2 {
            if y < SIZE.1 / 2 {
                return 0;
            } else if y >= (SIZE.1 / 2 + 1) {
                return 1;
            }
        } else if x >= (SIZE.0 / 2 + 1) {
            if y < SIZE.1 / 2 {
                return 2;
            } else if y >= (SIZE.1 / 2 + 1) {
                return 3;
            }
        }

        4
    }
}

pub(crate) struct Day14;
impl Puzzle for Day14 {
    type Part1 = i32;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let robots: Vec<Robot> = re
            .captures_iter(inp)
            .map(|c| c.extract())
            .map(|(_, [px, py, dx, dy])| Robot {
                pos: (px.parse().unwrap(), py.parse().unwrap()),
                delta: (dx.parse().unwrap(), dy.parse().unwrap()),
            })
            .collect();

        let mut quadrant_count = [0, 0, 0, 0, 0];
        for quad in robots.iter().map(|robot| robot.quadrant(100)) {
            quadrant_count[quad] += 1;
        }

        let safety_factor =
            quadrant_count[0] * quadrant_count[1] * quadrant_count[2] * quadrant_count[3];

        let (min_t, _) = (0..10_000)
            .map(|t| avg_dist(&robots, t))
            .enumerate()
            .min_by_key(|(_, v)| *v)
            .unwrap();

        (safety_factor, min_t)
    }
}

#[allow(dead_code)]
fn print(robots: &[Robot], time: usize) {
    let mut count = HashMap::new();

    for pos in robots.iter().map(|r| r.pos_at(time)) {
        *count.entry(pos).or_insert(0) += 1;
    }

    let mut output = String::with_capacity(((SIZE.1 + 1) * SIZE.0) as usize);

    for i in 0..SIZE.1 {
        for j in 0..SIZE.0 {
            if let Some(c) = count.get(&(j, i)) {
                output.push(char::from_digit(*c, 10).unwrap());
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }

    println!("{}[2J{}", 27 as char, output);
}

fn avg_dist(robots: &[Robot], time: usize) -> usize {
    let positions: Vec<_> = robots.iter().map(|r| r.pos_at(time)).collect();

    let avg_x: isize = (positions.iter().map(|&(x, _)| x).sum::<isize>()) / (positions.len() as isize);
    let avg_y: isize = (positions.iter().map(|&(x, _)| x).sum::<isize>()) / (positions.len() as isize);

    let mut dists: usize = 0;

    for (x, y) in positions.iter() {
        dists += ((x - avg_x).pow(2) + (y - avg_y).pow(2)) as usize;
    }

    dists / positions.len()
}
