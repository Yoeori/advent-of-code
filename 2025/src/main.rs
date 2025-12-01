mod puzzle;

mod day01;

use day01::Day01;

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
    let days = days!(Day01);
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
