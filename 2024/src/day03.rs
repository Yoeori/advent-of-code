use std::fs;

use regex::Regex;

#[derive(Debug, Clone)]
enum Action {
    Nothing,
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn main() {
    let file = fs::read_to_string("puzzles/03.txt").unwrap();
    let re = Regex::new(r"(mul|do|don\'t)\((([0-9]+),([0-9]+))?\)").unwrap();

    let muls: Vec<Action> = re
        .captures_iter(&file)
        .map(|caps| match (&caps[1], caps.get(2).is_none()) {
            ("mul", false) => Action::Mul(
                caps[3].parse::<u32>().unwrap(),
                caps[4].parse::<u32>().unwrap(),
            ),
            ("do", true) => Action::Do,
            ("don't", true) => Action::Dont,
            _ => Action::Nothing,
        })
        .collect();

    let part1 = muls
        .iter()
        .map(|a| match a {
            Action::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<u32>();
    println!("Exercise 1: {}", part1);

    let (part2, _) = muls
        .iter()
        .fold((0, true), |(total, is_enabled), v| match v {
            Action::Nothing => (total, is_enabled),
            Action::Do => (total, true),
            Action::Dont => (total, false),
            Action::Mul(a, b) => {
                if is_enabled {
                    (total + (a * b), is_enabled)
                } else {
                    (total, is_enabled)
                }
            }
        });
    println!("Exercise 2: {}", part2);
}
