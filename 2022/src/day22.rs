use core::panic;
use std::{collections::HashMap, iter, fmt::Display, cmp};

const SIZE: isize = 50;

type Coord = (isize, isize); // x, y
type MapCoord = (usize, Coord);

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn turn(&self, c: char) -> Self {
        match c {
            'L' => match self {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            },
            'R' => match self {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
            _ => panic!()
        }
    }

    fn apply(&self, (x, y): Coord) -> Coord {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        }
    }

    fn value(&self) -> isize {
        match self {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3
        }
    }
}

enum Block {
    Portal {
        to: MapCoord,
        dir: Option<Dir>
    },
    Stone,
    Empty
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Portal { to, .. } => write!(f, "{}", to.0),
            Block::Stone => write!(f, "#"),
            Block::Empty => write!(f, "."),
        }
    }
}

struct Map {
    id: usize,
    offset: Coord,
    map: HashMap<Coord, Block>
}

impl Map {
    fn from_string(inp: &str, id: usize, offset: Coord) -> Self {
        let mut map: HashMap<Coord, Block> = HashMap::new();

        for (y, line) in inp.lines().skip((SIZE * offset.1) as usize).enumerate().take(SIZE as usize) {
            for x in 0..SIZE {
                let c = &line[(SIZE * offset.0 + x) as usize..(SIZE * offset.0 + x + 1) as usize];
                if c == "#" {
                    map.insert((x as isize, y as isize), Block::Stone);
                } else if c == "." {
                    map.insert((x as isize, y as isize), Block::Empty);
                } else {
                    panic!("Out of map bounds.")
                }
            }
        }

        Map {
            id,
            offset,
            map
        }
    }

    fn add_portal_line<T: Iterator<Item=Coord>, N: Iterator<Item=Coord>>(&mut self, map: usize, from: T, mut to: N, dir: Option<Dir>) {
        for a in from {
            let b = to.next().unwrap();
            self.map.insert(a, Block::Portal {
                to: (map, b),
                dir
            });
        }
    }

    #[allow(dead_code)]
    fn couple_simple(&mut self, up: usize, down: usize, left: usize, right: usize) {
        self.add_portal_line(up, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), None);
        self.add_portal_line(down, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), None);
        self.add_portal_line(left, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), iter::repeat(SIZE - 1).take(SIZE as usize).zip(0..SIZE), None);
        self.add_portal_line(right, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), iter::repeat(0).take(SIZE as usize).zip(0..SIZE), None);
    }

    fn apply(&self, maps: &Vec<Map>, old: Coord, dir: Dir) -> (MapCoord, Dir) {
        let (x, y) = dir.apply(old);

        // dbg!((x, y), old, dir);

        let block = self.map.get(&(x, y)).unwrap();

        match block {
            Block::Portal { to: (map, (tx, ty)), dir: newdir } => {
                if let Some(Block::Stone) = maps[*map].map.get(&(*tx, *ty)) {
                    ((self.id, old), dir)
                } else {
                    ((*map, (*tx, *ty)), newdir.unwrap_or(dir))
                }
            },
            Block::Stone => {
                ((self.id, old), dir)
            },
            Block::Empty => {
                ((self.id, (x, y)), dir)
            },
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Block: {}", self.id);
        for y in -1..=SIZE {
            for x in -1..=SIZE {
                if let Some(block) = self.map.get(&(x, y)) {
                    print!("{}", block);
                } else {
                    print!(" ");
                }
            }
            println!()
        }
        println!()
    }

    fn offset_coord(&self, (x, y): Coord) -> Coord {
        (x + 1 + self.offset.0 * SIZE, y + 1 + self.offset.1 * SIZE)
    }
}


pub fn main() {
    let (map_text, instructions) = include_str!("../puzzles/22.txt").split_once("\n\n").unwrap();

    // compile 6 maps
    // .04
    // .1.
    // 52.
    // 3..

    let mut maps: Vec<Map> = Vec::with_capacity(6);

    maps.push(Map::from_string(map_text, 0, (1, 0)));
    maps.push(Map::from_string(map_text, 1, (1, 1)));
    maps.push(Map::from_string(map_text, 2, (1, 2)));
    maps.push(Map::from_string(map_text, 3, (0, 3)));
    maps.push(Map::from_string(map_text, 4, (2, 0)));
    maps.push(Map::from_string(map_text, 5, (0, 2)));

    maps[0].couple_simple(2, 1, 4, 4);
    maps[1].couple_simple(0, 2, 1, 1);
    maps[2].couple_simple(1, 0, 5, 5);
    maps[3].couple_simple(5, 5, 3, 3);
    maps[4].couple_simple(4, 4, 0, 0);
    maps[5].couple_simple(3, 3, 2, 2);

    let (pos, dir) = follow_path(&maps, instructions);
    let (x, y) = maps[pos.0].offset_coord(pos.1);
    println!("Exercise 1: {}", dir.value() + x * 4 + y * 1000);

    // Setup paths cube:
    // 0
    maps[0].add_portal_line(3, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), iter::repeat(0).take(SIZE as usize).zip(0..SIZE), Some(Dir::Right));
    maps[0].add_portal_line(1, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), None);
    maps[0].add_portal_line(5, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), iter::repeat(0).take(SIZE as usize).zip((0..SIZE).rev()), Some(Dir::Right));
    maps[0].add_portal_line(4, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), iter::repeat(0).take(SIZE as usize).zip(0..SIZE), None);
    
    // 1
    maps[1].add_portal_line(0, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), None);
    maps[1].add_portal_line(2, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), None);
    maps[1].add_portal_line(5, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), Some(Dir::Down));
    maps[1].add_portal_line(4, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), Some(Dir::Up));

    // 2
    maps[2].add_portal_line(1, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), None);
    maps[2].add_portal_line(3, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), iter::repeat(SIZE - 1).take(SIZE as usize).zip(0..SIZE), Some(Dir::Left));
    maps[2].add_portal_line(5, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), iter::repeat(SIZE - 1).take(SIZE as usize).zip(0..SIZE), None);
    maps[2].add_portal_line(4, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), iter::repeat(SIZE - 1).take(SIZE as usize).zip((0..SIZE).rev()), Some(Dir::Left));

    // 3
    maps[3].add_portal_line(5, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), None);
    maps[3].add_portal_line(4, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), None);
    maps[3].add_portal_line(0, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), Some(Dir::Down));
    maps[3].add_portal_line(2, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), Some(Dir::Up));

    // 4
    maps[4].add_portal_line(3, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(SIZE - 1).take(SIZE as usize)), None);
    maps[4].add_portal_line(1, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), iter::repeat(SIZE - 1).take(SIZE as usize).zip(0..SIZE), Some(Dir::Left));
    maps[4].add_portal_line(0, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), iter::repeat(SIZE - 1).take(SIZE as usize).zip(0..SIZE), None);
    maps[4].add_portal_line(2, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), iter::repeat(SIZE - 1).take(SIZE as usize).zip((0..SIZE).rev()), Some(Dir::Left));

    // 5
    maps[5].add_portal_line(1, (0..SIZE).zip(iter::repeat(-1).take(SIZE as usize)), iter::repeat(0).take(SIZE as usize).zip(0..SIZE), Some(Dir::Right));
    maps[5].add_portal_line(3, (0..SIZE).zip(iter::repeat(SIZE).take(SIZE as usize)), (0..SIZE).zip(iter::repeat(0).take(SIZE as usize)), None);
    maps[5].add_portal_line(0, iter::repeat(-1).take(SIZE as usize).zip(0..SIZE), iter::repeat(0).take(SIZE as usize).zip((0..SIZE).rev()), Some(Dir::Right));
    maps[5].add_portal_line(2, iter::repeat(SIZE).take(SIZE as usize).zip(0..SIZE), iter::repeat(0).take(SIZE as usize).zip(0..SIZE), None);

    let (pos, dir) = follow_path(&maps, instructions);
    let (x, y) = maps[pos.0].offset_coord(pos.1);
    println!("Exercise 2: {}", dir.value() + x * 4 + y * 1000);

}

fn follow_path(maps: &Vec<Map>, instructions: &str) -> (MapCoord, Dir) {
    let mut pos: MapCoord = (0, (0, 0));
    let mut dir: Dir = Dir::Right;

    let mut i = 0;
    while i < instructions.len() {
        if &instructions[i..(i+1)] == "L" {
            dir = dir.turn('L');
            i += 1;
        } else if &instructions[i..(i+1)] == "R" {
            dir = dir.turn('R');
            i += 1;
        } else {
            let steps: usize = if let Ok(n) = instructions[i..cmp::min(instructions.len(), i+2)].parse() {
                i += 2;
                n
            } else {
                let n =instructions[i..(i+1)].parse().unwrap();
                i += 1;
                n
            };

            for _ in 0..steps {
                (pos, dir) = maps[pos.0].apply(&maps, pos.1, dir);
                // println!("{:?}", maps[pos.0].offset_coord(pos.1));
            }
        }
    }

    (pos, dir)

}