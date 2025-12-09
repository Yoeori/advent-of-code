use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap, HashSet},
    ops::Add,
};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Point(isize, isize);

impl Point {
    fn size(&self, other: &Self) -> isize {
        ((self.0 - other.0).abs() + 1) * ((self.1 - other.1).abs() + 1)
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(',').unwrap();
        Point(left.parse().unwrap(), right.parse().unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Delta(isize, isize);

impl Add<&Delta> for &Point {
    type Output = Point;

    fn add(self, rhs: &Delta) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const DELTA: &[Delta] = &[Delta(0, 1), Delta(0, -1), Delta(1, 0), Delta(-1, 0)];

pub(crate) struct Day09;
impl Puzzle for Day09 {
    type Part1 = isize;
    type Part2 = isize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let points: Vec<Point> = inp.lines().map(Point::from).collect();

        // Part one
        let mut rectangles: BTreeSet<(isize, usize, usize)> = BTreeSet::new();

        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate().skip(i + 1) {
                rectangles.insert((p1.size(p2), i, j));
            }
        }

        let part_one = rectangles.last().unwrap().0;

        // Part two
        // Conversion from _enormous_ to tiny map
        let xs = simplify(points.iter().map(|p| p.0).collect());
        let ys = simplify(points.iter().map(|p| p.1).collect());
        let convert = |p: &Point| Point(xs[&p.0], ys[&p.1]);
        
        let converted_points: Vec<Point> = points.iter().map(convert).collect();

        // Draw path and determine outside
        let path = draw_path(&converted_points);
        let outside_grid = fill(&path);

        // Check squares
        let mut part_two = None;
        for &(size, i, j) in rectangles.iter().rev() {
            let p1 = convert(&points[i]);
            let p2 = convert(&points[j]);

            if check((p1, p2), &outside_grid) {
                part_two = Some(size);
                break;
            }
        }

        (part_one, part_two.unwrap())
    }
}

fn simplify(mut points: Vec<isize>) -> HashMap<isize, isize> {
    points.sort();
    points.dedup();

    points
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, (i * 2 + 1) as isize))
        .collect()
}

fn draw_path(points: &[Point]) -> HashSet<Point> {
    let mut res: HashSet<Point> = HashSet::new();

    for (p1, p2) in points
        .iter()
        .zip(points.iter().skip(1).chain([points.first().unwrap()]))
    {
        for x in min(p1.0, p2.0)..=max(p1.0, p2.0) {
            for y in min(p1.1, p2.1)..=max(p1.1, p2.1) {
                res.insert(Point(x, y));
            }
        }
    }

    res
}

fn fill(points: &HashSet<Point>) -> HashSet<Point> {
    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;

    let mut res: HashSet<Point> = HashSet::new();

    let mut queue: Vec<Point> = Vec::new();
    queue.push(Point(0, 0));

    while let Some(p) = queue.pop() {
        if res.contains(&p)
            || points.contains(&p)
            || p.0 < 0
            || p.1 < 0
            || p.0 > max_x
            || p.1 > max_y
        {
            continue;
        }

        for delta in DELTA {
            queue.push(&p + delta);
        }

        res.insert(p);
    }

    res
}

// Check that whole perimater of square is in grid
fn check((p1, p2): (Point, Point), grid: &HashSet<Point>) -> bool {
    for x in min(p1.0, p2.0)..=max(p1.0, p2.0) {
        if grid.contains(&Point(x, p1.1)) || grid.contains(&Point(x, p2.1)) {
            return false;
        }
    }

    for y in min(p1.1, p2.1)..=max(p1.1, p2.1) {
        if grid.contains(&Point(p1.0, y)) || grid.contains(&Point(p2.0, y)) {
            return false;
        }
    }

    true
}

#[allow(unused)]
fn visualize(points: &HashSet<Point>) {
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
