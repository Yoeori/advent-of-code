mod puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;

use ansi_term::Style;
use puzzle::PrintablePuzzle;
use std::{env, fs};

macro_rules! days {
    ($($y:ident),+) => (
        [
            $(Box::new($y) as Box<dyn PrintablePuzzle>),+
        ]
    )
}

fn main() {
    let days = days!(
        Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09
    );
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].parse::<usize>();

        if let Ok(n) = day
            && let Some(day) = days.get(n - 1)
        {
            let file = fs::read_to_string(format!("puzzles/{:02}.txt", n)).unwrap();
            println!("{}", Style::new().bold().paint(format!("Day {}", n)));
            day.exec(&file);
            return;
        }
    }

    println!("Please indicate a valid day to run.")
}
