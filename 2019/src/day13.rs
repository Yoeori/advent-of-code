use std::fs;
use std::collections::BTreeMap;
use std::collections::VecDeque;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Pos {
    x: i64,
    y: i64
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty, Wall, Block, HorizontalPaddle, Ball
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            x => x
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<i64> for Tile {
    fn from(id: i64) -> Self {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Unexpected input for tile type.")
        }
    }
}

impl From<&Tile> for String {
    fn from(tile: &Tile) -> Self {
        String::from(match tile {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "X",
            Tile::HorizontalPaddle => "-",
            Tile::Ball => "O"
        })
    }
}

#[derive(Default)]
struct Game {
    program: Vec<i64>,
    tiles: BTreeMap<Pos, Tile>,
    partial_input: VecDeque<i64>,

    last_ball: Pos,
    last_paddle: Pos,
    score: i64
}

impl From<&str> for Game {
    fn from(program: &str) -> Self {
        Game {
            program: program.split(",").map(|x| x.parse().unwrap()).collect(),
            ..Default::default()
        }
    }
}

impl super::intcode_computer::ReadWrite for Game {
    fn input(&mut self) -> i64 {
        // println!("{}", self.to_screen());

        match self.last_paddle.x.cmp(&self.last_ball.x) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1
        }
    }
    fn output(&mut self, output: i64) {
        self.partial_input.push_back(output);

        if self.partial_input.len() == 3 {
            if self.partial_input[0] == -1 && self.partial_input[1] == 0 {
                // println!("Score: {}", self.partial_input[2]);
                self.score = self.partial_input[2];
                self.partial_input.clear();
                return;
            }
            
            let pos = Pos {
                x: self.partial_input.pop_front().unwrap(),
                y: self.partial_input.pop_front().unwrap()
            };

            let tile = Tile::from(self.partial_input.pop_front().unwrap());
            
            if tile == Tile::Ball {
                self.last_ball = pos.clone();
            } else if tile == Tile::HorizontalPaddle {
                self.last_paddle = pos.clone();
            }

            self.tiles.insert(pos, tile);
        }
    }
}

impl Game {
    fn run(&mut self) {
        super::intcode_computer::simulate_computer_with_read_write(self.program.clone(), self);
    }

    fn play(&mut self) {
        self.program[0] = 2;
        self.run();
    }

    fn reset(&mut self) {
        self.partial_input.clear();
        self.tiles.clear();
    }
    
    #[allow(dead_code)]
    fn to_screen(&self) -> String {
        let mut output = String::new();
        let mut last_y = -1;

        for (pos, tile) in &self.tiles {
            if pos.y != last_y {
                last_y = pos.y;
                output.push('\n');
            }

            output.push_str(&String::from(tile));
        }

        return output;
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/13.txt").unwrap();
    let mut game = Game::from(&file_contents[..]);
    game.run();
    println!("Solution to exercise 1: {:?}", game.tiles.iter().filter(|&(_, tile)| tile == &Tile::Block).count());

    game.reset();
    game.play();

    println!("Solution to exercise 2: {}", game.score);
}