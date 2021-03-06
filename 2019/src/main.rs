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

mod intcode_computer;

use std::env;

fn main() {
    let days: &[fn()-> ()] = &[day01::main, day02::main, day03::main, day04::main, 
                               day05::main, day06::main, day07::main, day08::main, 
                               day09::main, day10::main, day11::main, day12::main,
                               day13::main, day14::main, day15::main, day16::main,
                               day17::main, day18::main];
                                    
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let day = args[1].parse::<usize>();

        if let Ok(n) = day {
            if let Some(day) = days.get(n-1) {
                return day()
            }
        }
    }

    println!("Please indicate a valid day to run.")
}