mod day01;
mod day02;

use std::env;
use ansi_term::Style;

fn main() {
    let days: &[fn()-> ()] = &[day01::main, day02::main];
                                    
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