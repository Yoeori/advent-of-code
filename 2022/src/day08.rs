use std::{u32::MAX, cmp};

pub fn main() {
    let mut grid: Vec<Vec<(u32, u32)>> = include_str!("../puzzles/8.txt")
        .lines()
        .map(|x| x.chars().map(|x| (x.to_digit(10).unwrap() + 1, MAX)).collect())
        .collect();

    // From right/left
    for y in 0..grid.len() {
        let mut cur_vis = 0;
        for x in (0..grid[0].len()).rev() {
            grid[y][x].1 = cmp::min(cur_vis, grid[y][x].1);
            cur_vis = cmp::max(cur_vis, grid[y][x].0);
        }

        let mut cur_vis = 0;
        for x in 0..grid[0].len() {
            grid[y][x].1 = cmp::min(cur_vis, grid[y][x].1);
            cur_vis = cmp::max(cur_vis, grid[y][x].0);
        }
    }

    // From top/bottom
    for x in 0..grid[0].len() {
        let mut cur_vis = 0;
        for y in 0..grid.len() {
            grid[y][x].1 = cmp::min(cur_vis, grid[y][x].1);
            cur_vis = cmp::max(cur_vis, grid[y][x].0);
        }

        let mut cur_vis = 0;
        for y in (0..grid.len()).rev() {
            grid[y][x].1 = cmp::min(cur_vis, grid[y][x].1);
            cur_vis = cmp::max(cur_vis, grid[y][x].0);
        }
    }

    println!("Exercise 1: {}", grid.iter().map(|l| l.iter().filter(|c| c.0 > c.1).count()).sum::<usize>());
    println!("Exercise 2: {}", (1..grid.len()).map(|y| (1..grid[y].len()).map(|x| scenic_score(&grid, x, y)).max().unwrap()).max().unwrap());

}

fn scenic_score(grid: &Vec<Vec<(u32, u32)>>, x: usize, y: usize) -> usize {
    let height = grid[y][x].0;

    let mut left = 0;
    for lx in (0..x).rev() {
        left += 1;
        if grid[y][lx].0 >= height {
            break;
        }
    }

    let mut right = 0;
    for lx in (x+1)..grid[0].len() {
        right += 1;
        if grid[y][lx].0 >= height {
            break;
        }
    }

    let mut top = 0;
    for ly in (0..y).rev() {
        top += 1;
        if grid[ly][x].0 >= height {
            break;
        }
    }

    let mut below = 0;
    for ly in (y+1)..grid.len() {
        below += 1;
        if grid[ly][x].0 >= height {
            break;
        }
    }

    left * right * below * top
}