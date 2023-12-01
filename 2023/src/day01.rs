use std::fs;

pub fn main() {
    let file = fs::read_to_string("puzzles/1.txt").unwrap();

    println!("Exercise 1: {}", (&file).lines().map(|l| calibration_value(l, false)).sum::<u32>());
    println!("Exercise 2: {}", (&file).lines().map(|l| calibration_value(l, true)).sum::<u32>());
}

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calibration_value(line: &str, string_search: bool) -> u32 {
    let mut last_digit: Option<u32> = None;
    let mut first_digit: Option<u32> = None;

    'outerloop: for i in 0..line.len() {
        if string_search {
            for (k, &number) in NUMBERS.iter().enumerate() {
                if number.len() + i <= line.len() && &line[i..(i + number.len())] == number {
                    if first_digit == None {
                        first_digit = Some((k + 1) as u32);
                    }
                    last_digit = Some((k + 1) as u32);
                    continue 'outerloop;
                }
            }
        }

        let c = line.chars().nth(i).unwrap();

        if c.is_digit(10) {
            let number = c.to_digit(10);
            if first_digit == None {
                first_digit = number;
            }
            last_digit = number;
        }
    }

    return last_digit.unwrap() + first_digit.unwrap() * 10;
}
