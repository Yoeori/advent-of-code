use std::fs;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::cmp::Ordering;

use num_rational::Ratio;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn angle(&self, other: &Point) -> Angle {
        assert_ne!(self, other);
        if other.x == self.x {
            return if other.y > self.y { Angle::DownStraight } else { Angle::UpStraight };
        } else if other.x < self.x {
            Angle::Left(Ratio::new_raw(self.y - other.y, other.x - self.x))
        } else {
            Angle::Right(Ratio::new_raw(self.y - other.y, other.x - self.x))
        }   
    }

    fn line_of_sight(&self, others: &Vec<Point>) -> usize {
        let mut angles = HashSet::<Angle>::new();

        for other in others {
            if other != self {
                angles.insert(self.angle(other));
            }
        }

        return angles.len();
    }

    fn dist_point<'a>(&self, other: &'a Point) -> DistPoint<'a> {
        DistPoint {
            point: other,
            dist: (self.x - other.x).abs() + (self.y - other.y).abs()
        }
    }

    fn vaporize_order<'a>(&self, others: &'a Vec<Point>) -> Vec<&'a Point> {
        
        let mut map: BTreeMap<Angle, Vec<DistPoint>> = BTreeMap::new();
        for point in others {
            if point == self {
                continue;
            }

            let angle = self.angle(point);
            if let Some(vec) = map.get_mut(&angle) {
                vec.push(self.dist_point(point));
            } else {
                map.insert(angle, vec![self.dist_point(point)]);
            }
        }

        let mut map: BTreeMap<Angle, Vec<DistPoint>> = map.into_iter().map(|(k, mut v)| {
            v.sort();
            v.reverse();
            (k, v)
        }).collect();

        let mut result: Vec<&Point> = vec![];
        while result.len() != others.len() - 1 {
            for (_, v) in map.iter_mut() {
                if !v.is_empty() {
                    result.push(v.pop().unwrap().point);
                }
            }
        }
        
        return result;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DistPoint<'a> {
    point: &'a Point,
    dist: i32
}

impl Ord for DistPoint<'_> {
    fn cmp(&self, other: &DistPoint) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for DistPoint<'_> {
    fn partial_cmp(&self, other: &DistPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Angle {
    Left(Ratio<i32>), Right(Ratio<i32>), UpStraight, DownStraight
}

impl Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        match (self, other) {
            (Angle::UpStraight, Angle::UpStraight) => Ordering::Equal,
            (Angle::UpStraight, _) => Ordering::Less,
            (_, Angle::UpStraight) => Ordering::Greater,

            (Angle::Right(r1), Angle::Right(r2)) => r1.cmp(r2).reverse(),
            (Angle::Right(_), _) => Ordering::Less,
            (_, Angle::Right(_)) => Ordering::Greater,

            (Angle::DownStraight, Angle::DownStraight) => Ordering::Equal,
            (Angle::DownStraight, _) => Ordering::Less,
            (_, Angle::DownStraight) => Ordering::Greater,

            (Angle::Left(l1), Angle::Left(l2)) => l1.cmp(l2).reverse()
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/10.txt").unwrap();

    let mut points = vec![];
    for (y, line) in file_contents.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push(Point {x: x as i32, y: y as i32});
            }
        }
    }

    let sol1 = points.iter().map(|point| (point, point.line_of_sight(&points))).max_by(|(_, s1), (_, s2)| s1.cmp(s2)).unwrap();
    println!("Solution to part 1: {}", sol1.1);

    let sol2 = sol1.0.vaporize_order(&points)[199];
    println!("Solution to part 2: {}", sol2.x * 100 + sol2.y);
}