
use crate::puzzle::Puzzle;

pub(crate) struct Day02;
impl Puzzle for Day02 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let xss = inp
            .lines()
            .map(|l| {
                l.split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let safe_reports: usize = xss.iter().map(|xs| if is_safe(xs) { 1 } else { 0 }).sum();

        let partial_safe_reports: usize = xss
            .iter()
            .map(|xs| if is_partial_safe(xs) { 1 } else { 0 })
            .sum();

        (safe_reports, partial_safe_reports)
    }
}

fn check(xs: &[u32], f: fn(u32, u32) -> bool) -> bool {
    xs.windows(2)
        .all(|x| x[0].abs_diff(x[1]) <= 3 && f(x[0], x[1]))
}

fn is_safe(xs: &[u32]) -> bool {
    let is_incr = check(xs, |a, b| a > b);
    let is_decr = check(xs, |a, b| a < b);

    is_incr || is_decr
}

fn is_partial_safe(xs: &[u32]) -> bool {
    (0..(xs.len())).any(|i| is_safe(&[&xs[..i], &xs[(i + 1)..]].concat()))
}
