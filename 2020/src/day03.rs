use std::fs;

#[derive(Debug)]
struct Pos(usize, usize);

pub fn main() {
    let file = fs::read_to_string("puzzles/3.txt").unwrap();
    let map: Vec<Vec<char>> = file.split("\n").map(|l| l.chars().collect()).collect();

    let mut total = check_slope(&map, 1, 3);
    println!("Answer to exercise 1: {}", total);

    total *= check_slope(&map, 1, 1);
    total *= check_slope(&map, 1, 5);
    total *= check_slope(&map, 1, 7);
    total *= check_slope(&map, 2, 1);

    println!("Answer to exercise 2: {}", total);
}

fn check_slope(map: &Vec<Vec<char>>, slope_y: usize, slope_x: usize) -> usize {
    let mut treecount = 0;
    let mut pos = Pos(0, 0);

    while pos.0 < map.len()-slope_y {
        pos.0 += slope_y;
        pos.1 += slope_x;

        if map[pos.0][pos.1 % map[pos.0].len()] == '#' {
            treecount += 1;
        }
    }

    treecount
}