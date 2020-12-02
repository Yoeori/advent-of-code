use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::fmt;

use std::slice::Iter;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North, East, South, West
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl From<i64> for Direction {
    fn from(dir: i64) -> Self {
        match dir {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Unknown direction!")
        }
    }
}

impl From<Direction> for i64 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }
}

impl Direction {
    fn position_change(&self) -> Pos {
        match self {
            Direction::North => Pos { x: 0, y: -1 },
            Direction::East => Pos { x: -1, y: 0 },
            Direction::South => Pos { x: 0, y: 1 },
            Direction::West => Pos { x: 1, y: 0 }
        }
    }

    fn iter() -> Iter<'static, Direction> {
        [Direction::North, Direction::South, Direction::East, Direction::West].iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn direction(&self, other: &Pos) -> Option<Direction> {
        for &dir in Direction::iter() {
            if &self.add(&dir.position_change()) == other {
                return Some(dir);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty, Wall, OxygenSystem, Droid
}

impl From<i64> for Tile {
    fn from(id: i64) -> Self {
        match id {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::OxygenSystem,
            3 => Tile::Droid,
            _ => panic!("Unknown tile type!")
        }
    }
}

impl From<&Tile> for String {
    fn from(tile: &Tile) -> Self {
        String::from(match tile {
            Tile::Empty => ".",
            Tile::Wall => "#",
            Tile::OxygenSystem => "O",
            Tile::Droid => "D"
        })
    }
}

#[derive(Default)]
struct RepairDroid {
    program: Vec<i64>,
    map: HashMap<Pos, Tile>,
    pos: Pos,
    oxygen_location: Option<Pos>,

    // Internal values
    travel_direction: Direction,
    undiscovered: Vec<Pos>,
    current_path: Vec<Direction>,

    is_finished: bool,
    is_oxygen_finished: bool,
    oxygen_count: i64
}

impl From<&str> for RepairDroid {
    fn from(program: &str) -> Self {
        let mut undiscovered = Vec::new();
        undiscovered.push(Pos { x: -1, y: 0 });
        undiscovered.push(Pos { x: 1, y: 0 });
        undiscovered.push(Pos { x: 0, y: 1 });
        undiscovered.push(Pos { x: 0, y: -1 });
            
        RepairDroid {
            program: program.split(",").map(|x| x.parse().unwrap()).collect(),
            undiscovered: undiscovered,
            ..Default::default()
        }
    }
}

impl super::intcode_computer::ReadWrite for RepairDroid {
    fn input(&mut self) -> i64 {
        println!("{}", self);

        if self.undiscovered.is_empty() && !self.is_finished {
            if let Some(oxygen_location) = &self.oxygen_location {
                self.is_finished = true;
                println!("Answer to exercise 1: {}", self.path_to_pos(&Pos{ x: 0, y: 0}, oxygen_location).unwrap().len())
            }
        }

        // Recalculate new path
        if self.current_path.is_empty() {
            if let Some(undis_pos) = self.undiscovered.pop() {
                // println!("Trying to find path from: {:?} to {:?}", self.pos, undis_pos);
                if let Some(path) = self.path_to_pos(&self.pos, &undis_pos) {
                    self.current_path = path;
                    // println!("{:?}", self.current_path);
                    // println!("FOUND PATH");
                } else {
                    self.undiscovered.push(undis_pos);
                }
            }
        }

        // Spread oxygen
        if self.is_finished && !self.is_oxygen_finished {
            let oxygen_pos: Vec<Pos> = {self.map.iter().filter(|(_, tile)| tile == &&Tile::OxygenSystem).map(|(pos, _)| pos.clone()).collect()};
            for pos in oxygen_pos {
                let neighbours: Vec<Pos> = self.neighbours(&pos).iter().map(|&x| x.clone()).collect();
                for neighbour in neighbours {
                    self.map.insert(neighbour.clone(), Tile::OxygenSystem);
                }
            }

            self.oxygen_count += 1;

            if self.map.iter().filter(|(_, tile)| tile == &&Tile::Empty).count() == 0 {
                println!("Answer to exercise 2: {}", self.oxygen_count);
                self.is_oxygen_finished = true;
            }
        }
        

        self.travel_direction = self.current_path.pop().unwrap_or(Direction::North);
        return i64::from(self.travel_direction);
    }

    fn output(&mut self, status_code: i64) {
        match status_code {
            0 => {
                let tile_pos = self.pos.add(&self.travel_direction.position_change());
                self.undiscovered.retain(|x| x != &tile_pos);
                self.map.insert(tile_pos, Tile::Wall);
            },
            1 => {
                self.pos = self.pos.add(&self.travel_direction.position_change());
                self.map.insert(self.pos.clone(), Tile::Empty);

                {
                    for dir in Direction::iter() {
                        let to_discover = self.pos.add(&dir.position_change());
                        if !self.map.contains_key(&to_discover) {
                            self.undiscovered.push(to_discover);
                        }
                    }
                }
                
                let to_delete = &self.pos;
                self.undiscovered.retain(|x| x != to_delete);
            },
            2 => {
                let tile_pos = self.pos.add(&self.travel_direction.position_change());
                self.undiscovered.retain(|x| x != &tile_pos);
                self.pos = tile_pos.clone();
                self.oxygen_location = Some(tile_pos.clone());
                self.map.insert(tile_pos, Tile::OxygenSystem);
            },
            _ => panic!("Unknown status code!")
        }
    }
}

impl fmt::Display for RepairDroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        let min_x = self.map.keys().map(|pos| pos.x).min().unwrap() - 1;
        let max_x = self.map.keys().map(|pos| pos.x).max().unwrap() + 1;
        let min_y = self.map.keys().map(|pos| pos.y).min().unwrap() - 1;
        let max_y = self.map.keys().map(|pos| pos.y).max().unwrap() + 1;

        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                if (Pos { x: x, y: y }) == self.pos {
                    out.push('D');
                } else if let Some(tile) = self.map.get(&Pos {x: x, y: y}) {
                    out.push_str(String::from(tile).as_str());
                } else if self.undiscovered.contains(&Pos {x: x, y: y}) {
                    out.push('?');
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

impl RepairDroid {
    fn run(&mut self) {
        super::intcode_computer::simulate_computer_with_read_write(self.program.clone(), self);
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.map.clear();
        self.pos = Pos {x: 0, y: 0};
    }

    fn neighbours(&self, pos: &Pos) -> Vec<&Pos> {
        let mut res = Vec::with_capacity(4);
        for dir in Direction::iter() {
            if let Some((pos, tile)) = self.map.get_key_value(&pos.add(&dir.position_change())) {
                if tile == &Tile::Empty || tile == &Tile::OxygenSystem {
                    res.push(pos);
                }
            }
        }
        res
    }
}

// Implement A* for the RepairDroid
#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: i64,
    position: &'a Pos,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RepairDroid {
    fn path_to_pos(&self, start: &Pos, to: &Pos) -> Option<Vec<Direction>> {
        let to_neighbours = self.neighbours(to);

        let mut dist: HashMap<&Pos, i64> = HashMap::with_capacity(self.map.len() + 1);
        let mut prev: HashMap<&Pos, &Pos> = HashMap::with_capacity(self.map.len() + 1);

        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(State { cost: 0, position: start });

        while let Some(State { cost, position }) = heap.pop() {
            if position == to {
                let mut route = vec![];

                let mut prev_pos = to;
                while let Some(pos) = prev.get(prev_pos) {
                    route.push(pos.direction(prev_pos).unwrap());
                    prev_pos = pos;
                }

                return Some(route);
            }

            if let Some(&d) = dist.get(position) {
                if cost > d { continue; }
            }

            let mut neighbours = self.neighbours(&position);

            if to_neighbours.contains(&position) {
                neighbours.push(to);
            }

            for neighbour in neighbours {
                let next = State { cost: cost + 1, position: neighbour };

                if &next.cost < dist.get(next.position).unwrap_or(&i64::max_value()) {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                    prev.insert(next.position, position);
                }
            }
        }
        
        None
    }

}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/15.txt").unwrap();
    let mut repair_droid = RepairDroid::from(&file_contents[..]);
    repair_droid.map.insert(Pos {x: 0, y: 0}, Tile::Empty);
    repair_droid.run();
}