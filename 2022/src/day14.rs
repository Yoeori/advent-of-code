use std::cmp;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Grid {
    Air, Rock, Sand
}

const SAND_SOURCE: (usize, usize) = (0, 500);

pub fn main() {
    let mut grid: Vec<Vec<Grid>> = vec![vec![Grid::Air; 1000]; 300]; // Bounds decided by random diceroll
    let mut max_y = 0;

    for mut line in include_str!("../puzzles/14.txt").lines().map(|l| l.split(" -> ")) {
        let mut from = line.next().unwrap();
        
        while let Some(to) = line.next() {
            let (fx, fy) = from.split_once(',').unwrap();
            let (fx, fy) = (fx.parse::<usize>().unwrap(), fy.parse::<usize>().unwrap());
            let (tx, ty) = to.split_once(',').unwrap();
            let (tx, ty) = (tx.parse::<usize>().unwrap(), ty.parse::<usize>().unwrap());

            max_y = cmp::max(cmp::max(fy, ty), max_y);

            for y in if fy <= ty { fy..=ty } else {ty..=fy } {
                for x in if fx <= tx { fx..=tx } else { tx..=fx } {
                    grid[y][x] = Grid::Rock;
                }
            }

            from = to;
        }
    }

    let mut count = 0;
    while simulate_sand(&mut grid, SAND_SOURCE) {
        count += 1;
    }

    println!("Exercise 1: {}", count);

    for x in 0..grid[0].len() {
        grid[max_y + 2][x] = Grid::Rock;
    }

    while grid[SAND_SOURCE.0][SAND_SOURCE.1] != Grid::Sand {
        simulate_sand(&mut grid, SAND_SOURCE);
        count += 1;
    }

    println!("Exercise 2: {}", count);

}

fn simulate_sand(grid: &mut Vec<Vec<Grid>>, pos: (usize, usize)) -> bool {
    // Out of bounds:
    if pos.0 == grid.len() - 2 {
        return false;
    }

    for (dy, dx) in &[(1, 0), (1, -1), (1, 1)] {
        if grid[pos.0 + dy][(pos.1 as isize + dx) as usize] == Grid::Air {
            return simulate_sand(grid, (pos.0 + dy, (pos.1 as isize + dx) as usize));
        }
    }

    // Settle here
    grid[pos.0][pos.1] = Grid::Sand;
    true
}