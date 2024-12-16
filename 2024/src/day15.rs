use std::{collections::HashSet, hash::Hash, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn get<T: Copy>(&'_ self, v: &[Vec<T>]) -> Option<T> {
        v.get(self.0).and_then(|v| v.get(self.1)).copied()
    }

    fn gps(&self) -> usize {
        (100 * self.0) + self.1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

const DIRS: [Delta; 4] = [Delta(0, 1), Delta(0, -1), Delta(1, 0), Delta(-1, 0)];

fn dir_from_char(inp: char) -> Delta {
    match inp {
        '>' => DIRS[0],
        '<' => DIRS[1],
        'v' => DIRS[2],
        '^' => DIRS[3],
        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    Left,
    Right,
    Single,
}

fn get_pair(boxes: &HashSet<(Pos, Side)>, pos: Pos) -> Option<(Pos, Pos)> {
    // make sure the pair is always ordered left/right
    if boxes.contains(&(pos, Side::Left)) {
        Some((pos, pos + Delta(0, 1)))
    } else if boxes.contains(&(pos, Side::Right)) {
        Some((pos + Delta(0, -1), pos))
    } else if boxes.contains(&(pos, Side::Single)) {
        Some((pos, pos))
    } else {
        None
    }
}

fn check_boxes(
    wall_map: &[Vec<bool>],
    boxes: &HashSet<(Pos, Side)>,
    cur: Pos,
    delta: Delta,
    found: &mut HashSet<(Pos, Pos)>,
) -> bool {
    // Check if box
    if let Some(pair) = get_pair(boxes, cur) {
        // Check if already found
        if found.contains(&pair) {
            return true;
        }

        found.insert(pair);

        // Check up to 2 neighbours, could be that one of these is part of the pair
        // but we that will be caught by the found set.
        check_boxes(wall_map, boxes, pair.0 + delta, delta, found)
            && check_boxes(wall_map, boxes, pair.1 + delta, delta, found)
    } else {
        cur.get(wall_map).unwrap_or(false)
    }
}

fn move_boxes(
    boxes: &mut HashSet<(Pos, Side)>,
    pos: Pos,
    delta: Delta,
    found: &mut HashSet<(Pos, Pos)>,
) {
    if let Some(pair) = get_pair(boxes, pos) {
        if found.contains(&pair) {
            return;
        }

        found.insert(pair);

        move_boxes(boxes, pair.0 + delta, delta, found);
        move_boxes(boxes, pair.1 + delta, delta, found);

        // Move self
        if boxes.remove(&(pair.0, Side::Single)) {
            boxes.insert((pair.0 + delta, Side::Single));
        } else {
            boxes.remove(&(pair.0, Side::Left));
            boxes.remove(&(pair.1, Side::Right));
    
            boxes.insert((pair.0 + delta, Side::Left));
            boxes.insert((pair.1 + delta, Side::Right));
        }
    }
}

fn move_series(
    wall_map: &[Vec<bool>],
    mut boxes: HashSet<(Pos, Side)>,
    mut pos: Pos,
    directions: &str,
) -> usize {
    for delta in directions.chars().map(dir_from_char) {
        let new_pos = pos + delta;

        if check_boxes(wall_map, &boxes, new_pos, delta, &mut HashSet::new()) {
            // We can move, perform move
            move_boxes(&mut boxes, new_pos, delta, &mut HashSet::new());
            pos = new_pos;
        }

        // print(wall_map, &boxes, pos);
    }

    boxes
        .iter()
        .filter(|(_, side)| side == &Side::Left || side == &Side::Single)
        .map(|(pos, _)| pos.gps())
        .sum()
}

pub(crate) struct Day15;
impl Puzzle for Day15 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (map, directions) = inp.split_once("\n\n").unwrap();
        let directions: String = directions.split('\n').collect();

        let (initial_pos, _) = map
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (Pos(y, x), c)))
            .find(|&(_, c)| c == '@')
            .unwrap();

        let boxes: HashSet<(Pos, Side)> = map
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(move |&(_, c)| c == 'O')
                    .map(move |(x, _)| (Pos(y, x), Side::Single))
            })
            .collect();

        let wall_map: Vec<Vec<bool>> = map
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c == '.' || c == '@' || c == 'O')
                    .collect()
            })
            .collect();

        let part_one = move_series(&wall_map, boxes.clone(), initial_pos, &directions);

        // Part 2: double everything
        let wall_map = double_map(&wall_map);
        let boxes: HashSet<(Pos, Side)> = boxes
            .into_iter()
            .flat_map(|(Pos(y, x), _)| {
                [
                    (Pos(y, x * 2), Side::Left),
                    (Pos(y, (x * 2) + 1), Side::Right),
                ]
            })
            .collect();
        let initial_pos = Pos(initial_pos.0, initial_pos.1 * 2);

        let part_two = move_series(&wall_map, boxes, initial_pos, &directions);

        (part_one, part_two)
    }
}

fn double_map(wall_map: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut map = Vec::with_capacity(wall_map.len());

    for line in wall_map {
        let mut double_line = Vec::with_capacity(line.len() * 2);

        for &v in line {
            double_line.push(v);
            double_line.push(v);
        }

        map.push(double_line);
    }

    map
}

#[allow(dead_code)]
fn print(wall_map: &[Vec<bool>], boxes: &HashSet<(Pos, Side)>, pos: Pos) {
    let height = wall_map.len();
    let width = wall_map[0].len();

    let mut output = String::with_capacity(height * width);

    for i in 0..height {
        for j in 0..width {
            let check = Pos(i, j);
            output.push(if !check.get(wall_map).unwrap() {
                '#'
            } else if boxes.contains(&(check, Side::Single)) {
                'O'
            } else if boxes.contains(&(check, Side::Left)) {
                '['
            } else if boxes.contains(&(check, Side::Right)) {
                ']'
            } else if check == pos {
                '@'
            } else {
                '.'
            });
        }
        output.push('\n');
    }

    println!("{}", output);
}
