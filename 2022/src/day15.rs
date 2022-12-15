use std::{cmp, collections::HashSet};

#[derive(Debug)]
struct SensorResult {
    sensor: (isize, isize),
    beacon: (isize, isize),
    dist: isize
}

impl SensorResult {
    fn parse(inp: &str) -> Self {

        fn parse_loc_pair(inp: &str) -> (isize, isize) {
            let (left, right) = inp.split_once(", y=").unwrap();
            let (_, left) = left.split_once("=").unwrap();
        
            (left.parse().unwrap(), right.parse().unwrap())
        }

        let (left, right) = &inp[10..].split_once(": closest beacon is at ").unwrap();
        let sensor = parse_loc_pair(left);
        let beacon = parse_loc_pair(right);

        SensorResult {
            sensor,
            beacon,
            dist: (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
        }
    }

    fn dist_to(&self, to: (isize, isize)) -> isize {
        (self.sensor.0 - to.0).abs() + (self.sensor.1 - to.1).abs()
    }

    /// Find first x position to the right where this sensor field ends
    fn find_end_on_y_from(&self, y: isize) -> isize {
        (self.dist - (self.sensor.1 - y).abs()) + self.sensor.0 + 1
    }

}

const SEARCH_Y: isize = 2_000_000;
const SEARCH_BOUNDS: (isize, isize) = (4_000_000, 4_000_000);

pub fn main() {
    let sensor_results: Vec<SensorResult> = include_str!("../puzzles/15.txt").split('\n').map(|l| SensorResult::parse(l)).collect();

    let min_x = sensor_results.iter().map(|s| s.sensor.0 - s.dist).min().unwrap();
    let max_x = sensor_results.iter().map(|s| s.sensor.0 + s.dist).max().unwrap();
    let mut count = 0;

    'outerloop: for x in min_x..max_x {
        for sensor_res in &sensor_results {
            if sensor_res.dist_to((x, SEARCH_Y)) <= sensor_res.dist {
                count += 1;
                continue 'outerloop;
            }
        }
    }

    println!("Exercise 1: {}", count - sensor_results.iter().filter(|s| s.beacon.1 == SEARCH_Y).map(|s| s.beacon.0).collect::<HashSet<isize>>().len());

    // :drake_no: n^2 goes brrr..
    // :drake_yes: n*k goes brrr..
    'outer: for y in 0..SEARCH_BOUNDS.1 {

        let mut x = 0;
        'inner: while x <= SEARCH_BOUNDS.0 {

            for sensor in sensor_results.iter() {
                if sensor.dist_to((x, y)) <= sensor.dist {
                    x = cmp::max(x, sensor.find_end_on_y_from(y));
                    continue 'inner;
                }
            }

            // No intersecting beacons found on valid position
            println!("Exercise 2: {}", x * 4_000_000 + y);
            break 'outer;
        }
    }

}