use std::{
    collections::{HashMap, HashSet},
    fs,
};

const DELTA: &[(isize, isize)] = &[
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (-1, 1),
    (1, -1),
];

pub fn main() {
    let file: String = fs::read_to_string("puzzles/3.txt").unwrap();
    let map: HashMap<(isize, isize), char> = file
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .flatten()
        .collect();

    let height = file.lines().count() as isize;
    let width = file.lines().next().unwrap().len() as isize;

    let mut marked: HashSet<(isize, isize)> = HashSet::new();

    for (&(x, y), c) in map.iter() {
        if !c.is_ascii_digit() && c != &'.' {
            for (dy, dx) in DELTA {
                marked.insert(((x + dx), (y + dy)));
            }
        }
    }

    let mut total = 0;

    let mut parts: Vec<u32> = vec![];
    let mut map_to_part: HashMap<(isize, isize), usize> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if map_to_part.contains_key(&(x, y)) {
                continue;
            }

            let mut marked_component: bool = false;
            let mut number = 0;

            // Try to find a number
            for dx in 0.. {
                if let Some(c) = map.get(&(x + dx, y)) {
                    if c.is_ascii_digit() {
                        map_to_part.insert((x + dx, y), parts.len());
                        marked_component = marked_component || marked.contains(&(x + dx, y));
                        number = number * 10 + c.to_digit(10).unwrap();
                        continue;
                    }
                }
                break;
            }

            if number != 0 {
                parts.push(number);
            }

            if marked_component {
                total += number;
            }
        }
    }

    println!("Exercise 1: {}", total);

    let mut total = 0;

    for (&(x, y), c) in map.iter() {
        if c == &'*' {
            let res: HashSet<usize> = DELTA
                .iter()
                .map(|(dx, dy)| map_to_part.get(&(x + dx, y + dy)))
                .filter(|x| x.is_some())
                .map(|x| *x.unwrap())
                .collect();

            if res.len() == 2 {
                let mut vals = res.iter();
                total += parts[*vals.next().unwrap()] * parts[*vals.next().unwrap()]
            }
        }
    }
    println!("Exercise 2: {}", total);
}
