use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::puzzle::Puzzle;

#[derive(Default, Debug, Clone)]
struct Rule {
    before: HashSet<u32>,
    after: HashSet<u32>,
}

impl Rule {
    fn add_before(&mut self, n: u32) {
        self.before.insert(n);
    }

    fn add_after(&mut self, n: u32) {
        self.after.insert(n);
    }

    fn check(&self, before: &[u32], after: &[u32]) -> Ordering {
        if !before.iter().all(|v| !self.before.contains(v)) {
            return Ordering::Less;
        }

        if !after.iter().all(|v| !self.after.contains(v)) {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

pub(crate) struct Day05;
impl Puzzle for Day05 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (rules_text, problems) = inp.split_once("\n\n").unwrap();

        // Could be an array [Rule; 100] since input is quite limited
        let mut rules: HashMap<u32, Rule> = HashMap::new();

        for (left, right) in rules_text
            .lines()
            .map(|l| l.split_once('|').unwrap())
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        {
            rules.entry(left).or_default().add_before(right);
            rules.entry(right).or_default().add_after(left);
        }

        let problems = problems
            .lines()
            .map(|l| l.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
            .collect::<Vec<Vec<u32>>>();


        let mut correct = 0;
        let mut incorrect = 0;

        for problem in problems {
            let sorted = bubble_sort(problem.clone(), &rules);

            if sorted == problem {
                correct += sorted[sorted.len() / 2];
            } else {
                incorrect += sorted[sorted.len() / 2];
            }
        }

        (correct, incorrect)
    }
}

fn bubble_sort(mut inp: Vec<u32>, rules: &HashMap<u32, Rule>) -> Vec<u32> {
    loop {
        let mut is_sorted: bool = true;
        for i in 0..(inp.len()) {
            let v = inp[i];
            if let Some(rule) = rules.get(&v) {
                match rule.check(&inp[..i], &inp[(i+1)..]) {
                    Ordering::Less => {
                        inp.swap(i, i - 1);
                        is_sorted = false;
                    },
                    
                    Ordering::Greater => {
                        inp.swap(i, i + 1);
                        is_sorted = false;
                    },

                    Ordering::Equal => {},
                }
            }
        }

        if is_sorted {
            break;
        }
    }

    inp
}
