use core::panic;

use regex::Regex;

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Problem {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prizex: i64,
    prizey: i64,
}

pub(crate) struct Day13;
impl Puzzle for Day13 {
    type Part1 = i64;
    type Part2 = i64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        let problems: Vec<Problem> = re
            .captures_iter(inp)
            .map(|c| c.extract())
            .map(|(_, [ax, ay, bx, by, prizex, prizey])| Problem {
                ax: ax.parse().unwrap(),
                ay: ay.parse().unwrap(),
                bx: bx.parse().unwrap(),
                by: by.parse().unwrap(),
                prizex: prizex.parse::<i64>().unwrap(),
                prizey: prizey.parse::<i64>().unwrap(),
            })
            .collect();

        let total_cost = problems
            .iter()
            .filter_map(find_solution)
            .filter(|&(a, b)| a <= 100 && b <= 100)
            .map(cost)
            .sum::<i64>();

        let total_cost_with_unit_conversion = problems
            .iter()
            .map(|&problem| Problem {
                prizex: problem.prizex + 10_000_000_000_000,
                prizey: problem.prizey + 10_000_000_000_000,
                ..problem
            })
            .filter_map(|p| find_solution(&p))
            .map(cost)
            .sum::<i64>();

        (total_cost, total_cost_with_unit_conversion)
    }
}

fn cost((a, b): (i64, i64)) -> i64 {
    a * 3 + b
}

fn find_solution(problem: &Problem) -> Option<(i64, i64)> {
    let div = (problem.ay * problem.bx) - (problem.ax * problem.by);

    // Is the problem linear independent?
    if div == 0 {
        // Doesn't seem necessary to implement
        panic!();
    }

    let a = (problem.prizey * problem.bx) - (problem.prizex * problem.by);
    let b = (problem.prizex * problem.ay) - (problem.prizey * problem.ax);

    // There's no integer solution
    if b % div != 0 || a % div != 0 {
        return None;
    }

    Some((a / div, b / div))
}
