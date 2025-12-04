use std::collections::HashMap;

use crate::puzzle::Puzzle;

const STEP: usize = 5;

pub(crate) struct Day11;
impl Puzzle for Day11 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let numbers = inp.split(' ').map(|x| x.parse::<u64>().unwrap());

        let mut found_prev = HashMap::new();
        let mut part_one = 0;

        for n in numbers {
            found_prev.insert(n, 1);
        }

        for n in (0..75).step_by(STEP) {
            let mut found: HashMap<u64, u64> = HashMap::new();

            for (&n, &mul) in found_prev.iter() {
                count_stones(n, STEP, &mut found, mul);
            }

            found_prev = found;

            if n == (25 - STEP) {
                part_one = found_prev.values().sum();
            }
        }

        (part_one, found_prev.values().sum::<u64>())
    }
}

fn count_stones(n: u64, rounds: usize, numbers: &mut HashMap<u64, u64>, mul: u64) {
    if rounds == 0 {
        *numbers.entry(n).or_insert(0) += mul;
    } else if n == 0 {
        count_stones(1, rounds - 1, numbers, mul)
    } else if (n.ilog10() % 2) == 1 {
        let div = 10_u64.pow(n.ilog10().div_ceil(2));
        let left = n / div;
        let right = n % div;

        count_stones(left, rounds - 1, numbers, mul);
        count_stones(right, rounds - 1, numbers, mul)
    } else {
        count_stones(n * 2024, rounds - 1, numbers, mul)
    }
}
