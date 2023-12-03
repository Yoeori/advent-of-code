use std::{fs, collections::HashMap};

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn parse(inp: &str) -> Self {
        let marbles: HashMap<&str, usize> = inp.split(", ").map(|v| {
            let r = v.split_once(' ').unwrap();
            (r.1, r.0.parse().unwrap())
        }).collect();

        Round {
            red: *marbles.get("red").unwrap_or(&0),
            green: *marbles.get("green").unwrap_or(&0),
            blue: *marbles.get("blue").unwrap_or(&0),
        }
    }
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(inp: &str) -> Self {
        let (left, rounds) = inp.split_once(": ").unwrap();
        let rounds: Vec<Round> = rounds.split("; ").map(|r| Round::parse(r)).collect();

        Game {
            id: left.split_once(' ').unwrap().1.parse().unwrap(),
            rounds
        }
    }

    fn possible_with_limited_marbles(&self) -> bool {
        self.rounds.iter().all(|round| {
            round.red <= 12 &&
            round.green <= 13 &&
            round.blue <= 14
        })
    }

    fn power(&self) -> usize {
        let red = self.rounds.iter().map(|r| r.red).max().unwrap();
        let green = self.rounds.iter().map(|r| r.green).max().unwrap();
        let blue = self.rounds.iter().map(|r| r.blue).max().unwrap();
        red * green * blue
    }
}

pub fn main() {
    let file: String = fs::read_to_string("puzzles/2.txt").unwrap();
    let games: Vec<Game> = file.lines().map(|l| Game::parse(l)).collect();

    println!("Exercise 1: {}", games.iter().filter(|game| game.possible_with_limited_marbles()).map(|game| game.id).sum::<usize>());
    println!("Exercise 2: {}", games.iter().map(|game| game.power()).sum::<usize>());
}

