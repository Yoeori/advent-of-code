use std::{collections::HashMap, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Pos(usize, usize);

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

// const DELTA: [(Delta, char); 4] = [
//     (Delta(0, 1), '>'),
//     (Delta(0, -1), '<'),
//     (Delta(1, 0), 'v'),
//     (Delta(-1, 0), '^'),
// ];

struct Robot<'a> {
    layout: &'a [&'a [char]],
}

impl Robot<'_> {
    fn pos_for(&self, inp: char) -> Pos {
        for (y, &row) in self.layout.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == inp {
                    return Pos(y, x);
                }
            }
        }

        panic!();
    }

    fn get_pos(&self, pos: Pos) -> Option<char> {
        self.layout
            .get(pos.0)
            .and_then(|&v| v.get(pos.1))
            .filter(|&&x| x != ' ')
            .copied()
    }
}

const ROBOTS: &[Robot] = &[
    Robot {
        layout: &[
            &['7', '8', '9'],
            &['4', '5', '6'],
            &['1', '2', '3'],
            &[' ', '0', 'A'],
        ],
    },
    Robot {
        layout: &[&[' ', '^', 'A'], &['<', 'v', '>']],
    },
];

pub(crate) struct Day21;
impl Puzzle for Day21 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let sequences: Vec<&str> = inp
            .split('\n')
            .collect();

        let mut part_one = 0;
        let mut part_two = 0;

        // lut for part two:
        let mut lut: HashMap<String, usize> = HashMap::new();

        for &sequence in sequences.iter() {
            let numeric_value = numeric_value(sequence);
            let last_robot = solve_sequence(sequence, &ROBOTS[0], 1);

            // Part one
            let two_robots: usize = solve_sequence(&last_robot, &ROBOTS[1], 2).len();
            part_one += two_robots *  numeric_value;

            // Part two: we split into two iterations: 12 and 13 rounds, put 13 rounds in lut
            let first_section = solve_sequence(&last_robot, &ROBOTS[1], 12);
            
            let score: usize = first_section
                .split_inclusive('A')
                .map(|r| {
                    if !lut.contains_key(r) {
                        lut.insert(r.to_string(), solve_sequence(r, &ROBOTS[1], 13).len());
                    }
                    *lut.get(r).unwrap()
                })
                .sum();

            part_two += score * numeric_value;
        }

        (part_one, part_two)
    }
}

fn solve_sequence(sequence: &str, robot: &Robot, length: usize) -> String {
    let mut cur: Vec<char> = ['A'].iter().copied().chain(sequence.chars()).collect();

    for _ in 0..length {
        let mut next: Vec<char> = Vec::new();
        next.push('A');

        for ab in cur.windows(2) {
            next.extend(path_for(robot, ab[0], ab[1]).chars());
            next.push('A');
        }

        cur = next;
    }

    cur.iter().skip(1).collect()
}

fn path_for(robot: &Robot<'_>, from: char, to: char) -> String {
    if from == to {
        return "".to_string();
    }

    let Pos(from_y, from_x) = robot.pos_for(from);
    let Pos(to_y, to_x) = robot.pos_for(to);

    let vertical: String = if from_y > to_y {
        (to_y..from_y).map(|_| '^').collect()
    } else {
        (from_y..to_y).map(|_| 'v').collect()
    };

    let horizontal: String = if from_x > to_x {
        (to_x..from_x).map(|_| '<').collect()
    } else {
        (from_x..to_x).map(|_| '>').collect()
    };

    if vertical.is_empty() {
        return horizontal;
    }

    if horizontal.is_empty() {
        return vertical;
    }

    // Rules after some bruteforcing
    // 1. Prefer going left first
    // 2. Prefer going down
    // 3. Prefer up - right

    if from_x > to_x && robot.get_pos(Pos(from_y, to_x)).is_some() {
        return format!("{}{}", horizontal, vertical);
    }

    if robot.get_pos(Pos(to_y, from_x)).is_some() {
        return format!("{}{}", vertical, horizontal);
    }

    format!("{}{}", horizontal, vertical)
}

fn numeric_value(chars: &str) -> usize {
    let mut v: usize = 0;

    for char in chars.chars() {
        if char.is_ascii_digit() {
            v *= 10;
            v += char.to_digit(10).unwrap() as usize;
        }
    }

    v
}