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
mod day11linked;
mod day12;
mod day13;

use std::env;
use ansi_term::Style;

fn main() {
    let days: &[fn()-> ()] = &[day01::main, day02::main, day03::main, day04::main, day05::main, day06::main, day07::main, day08::main,
                               day09::main, day10::main, day11::main, day12::main, day13::main];
                                    
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].parse::<usize>();

        if let Ok(n) = day {
            if let Some(day) = days.get(n-1) {
                println!("{}", Style::new().bold().paint(format!("Day {}", n)));
                return day()
            }
        }
    }

    println!("Please indicate a valid day to run.")
}