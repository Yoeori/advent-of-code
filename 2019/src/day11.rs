use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

enum Turn {
    Right, Left
}

impl Turn {
    fn from_num(num: i64) -> Turn {
        match num {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Unexpected color!")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Right, Turn::Left) => Direction::Up,
        }
    }

    fn rel(&self) -> Pos {
        match self {
            Direction::Up => Pos { x: 0, y: -1 },
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
            Direction::Down => Pos { x: 0, y: 1 }
        }
    }

    fn apply(&self, pos: &Pos) -> Pos {
        let rel = self.rel();
        Pos {
            x: pos.x + rel.x,
            y: pos.y + rel.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    Black, White
}

impl Color {
    fn to_num(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
    fn from_num(num: i64) -> Color {
        match num {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unexpected color!")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RobotAction {
    Paint, Turn
}

impl RobotAction {
    fn next(&self) -> RobotAction {
        match self {
            RobotAction::Paint => RobotAction::Turn,
            RobotAction::Turn => RobotAction::Paint
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    grid: HashMap<Pos, Color>,
    pos: Pos,
    direction: Direction,
    expected_action: RobotAction,
    painted_white: HashSet<Pos>
}

impl Robot {
    fn grid_to_string(&self) -> String {
        let mut out = String::new();

        let min_x = self.grid.keys().map(|pos| pos.x).min().unwrap();
        let max_x = self.grid.keys().map(|pos| pos.x).max().unwrap();

        let min_y = self.grid.keys().map(|pos| pos.y).min().unwrap();
        let max_y = self.grid.keys().map(|pos| pos.y).max().unwrap();

        for y in min_y..(max_y+1) {
            for x in min_x..(max_x+1) {
                if let Some(Color::White) = self.grid.get(&Pos {x: x, y: y}) {
                    out.push('x');
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }

        return out;
    }
}

impl super::intcode_computer::ReadWrite for Robot {
    fn input(&mut self) -> i64 {
        if let Some(color) = self.grid.get(&self.pos) {
            return color.to_num();
        } else {
            return 0;
        }
    }

    fn output(&mut self, output: i64) {
        match &self.expected_action {
            RobotAction::Paint => {
                if output == 1 {
                    self.painted_white.insert(self.pos.clone());
                }
                self.grid.insert(self.pos.clone(), Color::from_num(output));
            }
            RobotAction::Turn => {
                self.direction = self.direction.turn(Turn::from_num(output));
                self.pos = self.direction.apply(&self.pos);
            }
        }

        self.expected_action = self.expected_action.next();
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/11.txt").unwrap();
    let memory: Vec<i64> = file_contents.split(",").map(|x| x.parse().unwrap()).collect();

    let mut robot: Robot = Robot { 
        grid: HashMap::new(),
        pos: Pos { x: 0, y: 0 },
        direction: Direction::Up,
        expected_action: RobotAction::Paint,
        painted_white: HashSet::new()
    };
    super::intcode_computer::simulate_computer_with_read_write(memory.clone(), &mut robot);
    println!("Solution to part 1: {}", robot.painted_white.len());

    let mut grid = HashMap::new();
    grid.insert(Pos { x: 0, y: 0 }, Color::White);

    let mut robot: Robot = Robot { 
        grid: grid,
        pos: Pos { x: 0, y: 0 },
        direction: Direction::Up,
        expected_action: RobotAction::Paint,
        painted_white: HashSet::new()
    };
    super::intcode_computer::simulate_computer_with_read_write(memory.clone(), &mut robot);
    println!("Solution to part 2:\n{}", robot.grid_to_string());
}