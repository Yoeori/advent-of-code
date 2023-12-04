use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    fn parse(inp: &str) -> Self {
        fn parse_block(inp: &str, set: &mut HashSet<usize>) {
            for i in (0..inp.len()).step_by(3) {
                set.insert(inp[i..(i + 2)].trim().parse().unwrap());
            }
        }

        let (left, right) = inp.split_once(" | ").unwrap();
        let mut winning_numbers = HashSet::new();
        let mut numbers = HashSet::new();

        parse_block(right, &mut numbers);
        parse_block(left.split_once(": ").unwrap().1, &mut winning_numbers);

        Card {
            winning_numbers,
            numbers,
        }
    }

    fn value(&self) -> usize {
        let mut total = 0;
        for _ in 0..self.intersections() {
            if total == 0 {
                total = 1;
            } else {
                total *= 2;
            }
        }
        total
    }

    fn intersections(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

pub fn main() {
    let file = fs::read_to_string("puzzles/4.txt").unwrap();

    let games: Vec<Card> = file.lines().map(|line| Card::parse(line)).collect();

    println!(
        "Exercise 1: {}",
        games.iter().map(|game| game.value()).sum::<usize>()
    );

    let mut number_of: Vec<usize> = vec![1; games.len()];
    for (i, game) in games.iter().enumerate() {
        for j in (i + 1)..(i + 1 + game.intersections()) {
            if j >= games.len() {
                continue;
            }
            number_of[j] += number_of[i];
        }
    }
    println!("Exercise 2: {}", number_of.iter().sum::<usize>());
}
