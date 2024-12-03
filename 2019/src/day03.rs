use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Direction {
    fn from_string(input: &str) -> Result<Direction, &str>  {
        let direction = input.chars().next().unwrap();
        let length = (&input[direction.len_utf8()..]).parse::<i32>();

        if let Err(_) = length {
            return Err("Unable to parse direction")
        }

        let length = length.unwrap();

        match direction {
            'U' => Ok(Direction::Up(length)),
            'D' => Ok(Direction::Down(length)),
            'L' => Ok(Direction::Left(length)),
            'R' => Ok(Direction::Right(length)),
            _ => Err("Unable to parse direction")
        }
    }

    fn from_string_array(input: &str) -> Result<Vec<Direction>, &str> {
        input.split(",").map(|x| Direction::from_string(x)).collect()
    }

    fn delta(&self) -> Pos {
        match self {
            Direction::Up(_) => Pos { x: 0, y: 1 },
            Direction::Down(_) => Pos { x: 0, y: -1 },
            Direction::Left(_) => Pos { x: -1, y: 0 },
            Direction::Right(_) => Pos { x: 1, y: 0 },
        }
    }

    fn size(&self) -> i32 {
        match self {
            Direction::Up(x) => *x,
            Direction::Down(x) => *x,
            Direction::Left(x) => *x,
            Direction::Right(x) => *x
        }
    }

    fn fill(&self, mut cur: Pos, taken: &mut HashMap<Pos, usize>) -> Pos {
        for _ in 0..self.size() {
            cur = cur.add(&self.delta());
            taken.insert(cur.clone(), taken.len() + 1);
        }
        cur
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/03.txt").unwrap();
    let file_contents: Vec<&str> = file_contents.split("\n").collect();

    let direction1 = Direction::from_string_array(&file_contents[0]).unwrap();
    let direction2 = Direction::from_string_array(&file_contents[1]).unwrap();

    let mut cur = Pos{x: 0, y: 0};
    let mut taken1: HashMap<Pos, usize> = HashMap::new();

    for direction in &direction1 {
        cur = direction.fill(cur, &mut taken1);
    }

    let mut cur = Pos{x: 0, y: 0};
    let mut taken2: HashMap<Pos, usize> = HashMap::new();

    for direction in &direction2 {
        cur = direction.fill(cur, &mut taken2);
    }

    let taken1_set: HashSet<&Pos> = HashSet::from_iter(taken1.keys());
    let taken2_set: HashSet<&Pos> = HashSet::from_iter(taken2.keys());
    let intersection = taken1_set.intersection(&taken2_set);

    // Problem 1
    println!("Solution to problem 1: {}", intersection.clone().map(|x| x.manhattan()).min().unwrap());

    // Problem 2
    let mut dist_costs: HashSet<usize> = HashSet::new();
    for pos in intersection {
        dist_costs.insert(taken1.get(pos).unwrap() + taken2.get(pos).unwrap());
    }

    println!("Solution to problem 2: {}", dist_costs.iter().min().unwrap())

}