use crate::puzzle::Puzzle;

pub(crate) struct Day22;
impl Puzzle for Day22 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let monkeys: Vec<usize> = inp.lines().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut monkey_prices: Vec<Vec<usize>> = vec![];
        let mut monkey_diffs: Vec<Vec<i8>> = vec![];

        let mut part_one = 0;

        for &monkey in monkeys.iter() {
            let mut cur = monkey;
            let mut prices = Vec::with_capacity(2000);

            for _ in 0..2000 {
                cur = secret_evolve(cur);
                prices.push(cur % 10);
            }

            monkey_diffs.push(
                prices
                    .windows(2)
                    .map(|c| ((c[1] as isize) - (c[0] as isize)) as i8)
                    .collect(),
            );

            monkey_prices.push(prices);

            part_one += cur;
        }

        (part_one, find_best(&monkey_prices, &monkey_diffs).1)
    }
}

fn secret_evolve(mut cur: usize) -> usize {
    cur = (cur ^ (cur << 6)) & (16777216 - 1);
    cur = (cur ^ (cur >> 5)) & (16777216 - 1);
    (cur ^ (cur << 11)) & (16777216 - 1)
}

fn find_best(prices: &[Vec<usize>], diffs: &[Vec<i8>]) -> ([i8; 4], usize) {
    let mut best: Option<[i8; 4]> = None;
    let mut best_v = 0;

    // I'm not going to optimize it, only takes around 5 mins
    for one in -9..=9 {
        for two in -9..=9 {
            for three in -9..=9 {
                for four in -9..=9 {
                    let t: [i8; 4] = [one, two, three, four];
                    if test_hypothesis(prices, diffs, &t) > best_v {
                        best = Some(t);
                        best_v = test_hypothesis(prices, diffs, &t);
                    }
                }
            }
        }
    }

    (best.unwrap(), best_v)
}

fn test_hypothesis(prices: &[Vec<usize>], diffs: &[Vec<i8>], check: &[i8]) -> usize {
    let mut total = 0;
    for (i, monkey ) in diffs.iter().enumerate() {
        if let Some((idx, _) ) = monkey.windows(check.len()).enumerate().find(|(_idx,  c)| c == &check) {
            total += prices[i].get(idx + check.len()).copied().unwrap_or(0);
        }
    }

    total
}