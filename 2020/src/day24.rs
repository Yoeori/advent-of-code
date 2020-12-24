use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Dir {
    E, SE, SW, W, NW, NE
}

impl Dir {
    const fn iter() -> &'static [Dir] {
        &[Dir::E, Dir::SE, Dir::SW, Dir::W, Dir::NW, Dir::NE]
    }

    const fn delta(&self) -> (isize, isize) {
        match self {
            Dir::E => (1, 0),
            Dir::SE => (0, 1),
            Dir::SW => (-1, 1),
            Dir::W => (-1, 0),
            Dir::NW => (0, -1),
            Dir::NE => (1, -1)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Black, White
}

impl Color {
    fn toggle(&mut self) {
        *self = match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}

pub fn main() {
    let positions: Vec<(isize, isize)> = include_str!("../puzzles/24.txt").split('\n')
        .map(|l| to_pos(&to_directions(&mut l.chars()))).collect();

    let mut tiles: HashMap<(isize, isize), Color> = HashMap::new();
    for pos in positions.into_iter() {
        tiles.entry(pos).or_insert(Color::White).toggle();
    }

    println!("Solution to exercise 1: {}", tiles.values().filter(|&x| x == &Color::Black).count());

    for _ in 0..100 {
        simulate_day(&mut tiles);
    }

    println!("Solution to exercise 2: {}", tiles.values().filter(|&x| x == &Color::Black).count());
}

fn to_directions(iter: &mut impl Iterator<Item=char>) -> Vec<Dir> {
    let mut res = vec![];
    while let Some(c) = iter.next() {
        res.push(match c {
            'e' => Dir::E,
            'w' => Dir::W,
            's' => {
                match iter.next().unwrap() {
                    'w' => Dir::SW,
                    'e' => Dir::SE,
                    _ => panic!("Unknown subinput: {}", c)
                }
            },
            'n' => {
                match iter.next().unwrap() {
                    'w' => Dir::NW,
                    'e' => Dir::NE,
                    _ => panic!("Unknown subinput: {}", c)
                }
            }
            _ => panic!("Unknown char in input: {}", c)
        });
    }

    res
}

fn to_pos(directions: &Vec<Dir>) -> (isize, isize) {
    let mut x  = 0;
    let mut y = 0;

    for dir in directions.iter() {
        let (dx, dy) = dir.delta();
        x += dx;
        y += dy;
    }

    (x, y)
}

fn simulate_day(tiles: &mut HashMap<(isize, isize), Color>) {
    let mut black_count: HashMap<(isize, isize), usize> = tiles.keys().map(|pos| (*pos, 0)).collect();

    for ((x, y), _) in tiles.iter().filter(|(_, color)| color == &&Color::Black) {
        for dir in Dir::iter() {
            let (dx, dy) = dir.delta();
            *black_count.entry((*x + dx, *y + dy)).or_insert(0) += 1;
        }
    }

    for (pos, &count) in black_count.iter() {
        if count == 0 || count > 2 {
            tiles.insert(*pos, Color::White);
        } else if count == 2 {
            tiles.insert(*pos, Color::Black);
        }
    }
}
