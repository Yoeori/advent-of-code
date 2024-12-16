use crate::puzzle::Puzzle;

pub(crate) struct Day07;
impl Puzzle for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut total = 0;
        let mut total_with_or = 0;

        for (answer, numbers) in inp.lines().map(|l| l.split_once(": ").unwrap()) {
            let answer = answer.parse::<u64>().unwrap();
            let numbers = numbers
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u64>>();

            if is_possible(numbers[0], &numbers[1..], answer, false) {
                total += answer;
            }

            if is_possible(numbers[0], &numbers[1..], answer, true) {
                total_with_or += answer;
            }
        }

        (total, total_with_or)
    }
}

fn concat(left: u64, right: u64) -> u64 {
    ((10u64.pow((right.ilog10()) + 1)) * left) + right
}

fn is_possible(intermediate: u64, inp: &[u64], answer: u64, enable_or: bool) -> bool {
    if intermediate > answer {
        return false
    }

    if inp.is_empty() {
        return answer == intermediate;
    }

    is_possible(intermediate * inp[0], &inp[1..], answer, enable_or)
        || is_possible(intermediate + inp[0], &inp[1..], answer, enable_or)
        || (enable_or && is_possible(concat(intermediate, inp[0]), &inp[1..], answer, enable_or))
}
