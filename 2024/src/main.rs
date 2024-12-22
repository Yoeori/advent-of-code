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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;

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
        Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10, Day11, Day12, Day13,
        Day14, Day15, Day16, Day17, Day18, Day19, Day20
    );
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].parse::<usize>();

        if let Ok(n) = day {
            if let Some(day) = days.get(n - 1) {
                let file = fs::read_to_string(format!("puzzles/{:02}.txt", n)).unwrap();
                println!("{}", Style::new().bold().paint(format!("Day {}", n)));
                day.exec(&file);
                return;
            }
        }
    }

    println!("Please indicate a valid day to run.")
}
