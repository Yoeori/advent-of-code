use std::fs;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Wall, Empty, Door(char), Key(char)
}

impl From<&Tile> for char {

    fn from(t: &Tile) -> Self {
        match t {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Door(c) => c.to_ascii_uppercase(),
            Tile::Key(c) => *c
        }
    }

}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    #[allow(dead_code)]
    fn diff(&self) -> Pos {
        match self {
            Direction::Up => Pos { x: 0, y: -1},
            Direction::Right => Pos { x: 1, y: 0},
            Direction::Down => Pos { x: 0, y: 1},
            Direction::Left => Pos { x: -1, y: 0},

        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
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

#[derive(Debug)]
struct Map {
    tiles: BTreeMap<Pos, Tile>,
    pos: Pos,

    has_keys: HashSet<char>,

    keys: HashMap<char, Pos>,
    doors: HashMap<char, Pos>
}

impl Map {

    fn from_string(map: &str) -> Map {

        let mut pos = None;
        let mut tiles: BTreeMap<Pos, Tile> = BTreeMap::new();

        let mut keys = HashMap::new();
        let mut doors = HashMap::new();

        for (y, line) in map.split('\n').enumerate().map(|(i, line)| (i as i32, line)) {
            for (x, c) in line.chars().enumerate().map(|(i, c)| (i as i32, c)) {
                tiles.insert(Pos {x: x, y: y}, match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    '@' => {
                        pos = Some(Pos { x: x, y: y });
                        Tile::Empty
                    },
                    l @ 'a'..='z' => {
                        keys.insert(l, Pos { x: x, y: y });
                        Tile::Key(l)
                    },
                    l @ 'A'..='Z' => {
                        doors.insert(l.to_ascii_lowercase(), Pos { x: x, y: y });
                        Tile::Door(l.to_ascii_lowercase())
                    },
                    _ => panic!("Unexpected character in Map input")
                });
            }
        };

        Map {
            tiles: tiles,
            pos: pos.expect("No start position found in Map input"),
            has_keys: HashSet::new(),

            keys: keys,
            doors: doors
        }
    }

    #[allow(dead_code)]
    fn move_with_direction(&mut self, dir: &Direction) -> Result<(), &'static str> {
        self.is_move_possible(&dir)?;

        self.pos = &self.pos + &dir.diff();

        if let Some(Tile::Key(c)) = self.tiles.get(&self.pos) {
            self.has_keys.insert(*c);
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn is_move_possible(&self, dir: &Direction) -> Result<(), &'static str> {
        match self.tiles.get(&(&self.pos + &dir.diff())) {
            None | Some(Tile::Wall) => Err("Possition does not exist"),

            Some(Tile::Key(_)) => {
                Ok(())
            },

            Some(Tile::Door(c)) if self.has_keys.contains(c) => Ok(()),

            Some(Tile::Empty) => Ok(()),

            Some(Tile::Door(_)) => Err("No key for door")
        }
    }


    #[allow(dead_code)]
    fn to_screen(&self) -> String {
        let mut output = String::new();
        let mut last_y = 0;

        for (pos, tile) in &self.tiles {
            if pos.y != last_y {
                last_y = pos.y;
                output.push('\n');
            }

            output.push(char::from(tile));
        }

        return output;
    }

}


pub fn main() {
    let file_contents = fs::read_to_string("puzzles/18.txt").unwrap();
    let map = Map::from_string(&file_contents);
    println!("{}", map.to_screen());
}