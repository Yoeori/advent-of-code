use itertools::Itertools;
use std::{cell::{RefCell}, collections::HashMap, slice::Iter};
use std::collections::VecDeque;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up, Right, Down, Left
}

impl Dir {
    fn iter() -> Iter<'static, Dir> {
        [Dir::Up, Dir::Right, Dir::Down, Dir::Left].iter()
    }

    fn next_rotate(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up
        }
    }

    fn next_mirror_x(&self) -> Self {
        match self {
            Dir::Up => Dir::Up,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Down,
            Dir::Left => Dir::Right
        }
    }

    fn next_mirror_y(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Right,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Left
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    id: usize,
    content: Vec<Vec<char>>,
    neighbours: HashMap<Dir, Option<usize>>
}

impl Tile {
    fn rotate(&mut self) {
        let mut res = vec![];

        for y in 0..self.content.len() {
            res.push(vec![]);
            for x in 0..self.content.len() {
                res[y].push(self.content[self.content.len() - x - 1][y]);
            }
        }

        self.content = res;
        self.neighbours = self.neighbours.clone().into_iter().map(|(r, neighbour)| (r.next_rotate(), neighbour)).collect();
    }

    fn mirror_x(&mut self) {
        for y in 0..self.content.len() {
            self.content[y].reverse();
        }

        self.neighbours = self.neighbours.clone().into_iter().map(|(r, neighbour)| (r.next_mirror_x(), neighbour)).collect();
    }

    fn mirror_y(&mut self) {
        for y in 0..self.content.len() / 2 {
            let index = self.content.len() - y - 1;
            let tmp = self.content[index].clone();
            self.content[index] = self.content[y].clone();
            self.content[y] = tmp;
        }
        
        self.neighbours = self.neighbours.clone().into_iter().map(|(r, neighbour)| (r.next_mirror_y(), neighbour)).collect();
    }

    fn side(&self, direction: &Dir) -> Vec<char> {
        match direction {
            Dir::Up => self.content[0].clone(),
            Dir::Right => self.content.iter().map(|l| l[l.len() - 1]).collect(),
            Dir::Down => self.content[self.content.len() - 1].clone(),
            Dir::Left => self.content.iter().map(|l| l[0]).collect()
        }
    }

    fn find_matches(&mut self, tiles: &VecDeque<Tile>) {
        let mut res = HashMap::new();

        'outerloop: for dir in Dir::iter() {
            let side = self.side(dir);

            for tile in tiles {
                for dir2 in Dir::iter() {
                    if tile.side(dir2).into_iter().rev().collect::<Vec<char>>() == side {
                        res.insert(*dir, Some(tile.id));
                        continue 'outerloop;
                    } else if tile.side(dir2) == side {
                        res.insert(*dir, Some(tile.id));
                        continue 'outerloop;
                    }
                }

                res.insert(*dir, None);
            }
        }

        self.neighbours = res;
    }

    fn normalize(&mut self, tiles: &HashMap<usize, RefCell<Tile>>) {
        if let Some(n_right) = &self.neighbours.get(&Dir::Right).unwrap() {
            let self_right = self.side(&Dir::Right);
            let mut n_right = tiles.get(n_right).unwrap().borrow_mut();

            while n_right.side(&Dir::Left) != self_right {
                n_right.rotate();
                
                if n_right.side(&Dir::Left).into_iter().rev().collect::<Vec<char>>() == self_right {
                    n_right.mirror_y();
                }
            }

            n_right.normalize(tiles);
        }

        if let None = &self.neighbours.get(&Dir::Left).unwrap() {
            if let Some(n_down) = &self.neighbours.get(&Dir::Down).unwrap() {
                let self_down = self.side(&Dir::Down);
                let mut n_down = tiles.get(n_down).unwrap().borrow_mut();

                while n_down.side(&Dir::Up) != self_down {
                    n_down.rotate();
                    
                    if n_down.side(&Dir::Up).into_iter().rev().collect::<Vec<char>>() == self_down {
                        n_down.mirror_x();
                    }
                }

                n_down.normalize(tiles);
            }
        }
    }

    fn fill(&self, vect: &mut Vec<Vec<char>>, dy: usize, dx: usize, tiles: &HashMap<usize, RefCell<Tile>>) {
        for (y, l) in (1..self.content.len() - 1).map(|y| (1..self.content.len() - 1).map(move |x| self.content[y][x])).enumerate() {
            for (x, c) in l.enumerate() {
                vect[dy * 8 + y][dx * 8 + x] = c;
            }
        }

        if let Some(n_right) = &self.neighbours.get(&Dir::Right).unwrap() {
            tiles.get(n_right).unwrap().borrow().fill(vect, dy, dx + 1, tiles);
        }

        if let None = &self.neighbours.get(&Dir::Left).unwrap() {
            if let Some(n_down) = &self.neighbours.get(&Dir::Down).unwrap() {
                tiles.get(n_down).unwrap().borrow().fill(vect, dy + 1, dx, tiles);
            }
        }
    }

    #[allow(dead_code)]
    fn tile_to_string(&self) -> String {
        self.content.iter().map(|l| l.iter().collect::<String>()).intersperse("\n".to_string()).collect::<String>()
    }

    fn is_corner(&self) -> bool {
        self.neighbours.values().filter(|x| x.is_none()).count() == 2
    }
}

pub fn main() {
    let tiles_string = include_str!("../puzzles/20.txt").split("\n\n");

    let mut tiles: VecDeque<Tile> = tiles_string.map(|t| t.split('\n')).map(|mut tile| Tile {
        id: tile.next().unwrap()[5..9].parse().unwrap(),
        content: tile.map(|l| l.chars().collect()).collect(),
        neighbours: HashMap::new()
    }).collect();

    for _ in 0..tiles.len() {
        let mut tile = tiles.pop_front().unwrap();
        tile.find_matches(&tiles);
        tiles.push_back(tile);
    }

    println!("Solution to exercise 1: {:?}", tiles.iter().filter(|t| t.is_corner()).map(|t| t.id).product::<usize>());

    let tiles: HashMap<usize, RefCell<Tile>> = tiles.into_iter().map(|t| (t.id, RefCell::new(t))).collect();

    // Get possible top left corner
    let mut tile = tiles.values().find(|v| {
        let keys = &v.borrow().neighbours;
        keys[&Dir::Right].is_some() && keys[&Dir::Down].is_some() && keys[&Dir::Left].is_none() && keys[&Dir::Up].is_none()
    }).unwrap().borrow_mut();

    // Align every neighbour to match to this tile
    tile.normalize(&tiles);

    let mut grid = vec![vec![' '; ((tiles.len() as f64).sqrt() as usize) * 8]; ((tiles.len() as f64).sqrt() as usize) * 8];
    tile.fill(&mut grid, 0, 0, &tiles);

    let mut full_tile = Tile {
        content: grid,
        neighbours: HashMap::new(),
        id: 0
    };

    for i in 0..8 {
        let count = find_seamonsters(&full_tile.content);

        if count != 0 {
            // Apparently none of the sea monsters overlap thus we can calculate the number of #'s
            println!("Solution to exercise 2: {}", full_tile.content.iter().map(|l| l.iter().filter(|&c| c == &'#').count()).sum::<usize>() - 15 * count);
            break;
        }
        
        if i == 3 {
            full_tile.mirror_x();
        }
        full_tile.rotate();
    }
}

fn find_seamonsters(map: &Vec<Vec<char>>) -> usize {
    lazy_static! {
        static ref SEA_MONSTER: Vec<(usize, usize)> = include_str!("day20_sea_monster.txt").split('\n').enumerate()
            .map(|(y, l)| l.chars().enumerate().filter(|(_, c)| c == &'#').map(|(x, _)| (y, x)).collect::<Vec<(usize, usize)>>())
            .flatten().collect();

        static ref Y_MAX: usize = SEA_MONSTER.iter().max_by(|(y1, _), (y2, _)| y1.cmp(y2)).unwrap().0;
        static ref X_MAX: usize = SEA_MONSTER.iter().max_by(|(_, x1), (_, x2)| x1.cmp(x2)).unwrap().1;
    }

    let mut count = 0;
    for y in 0..map.len() - *Y_MAX {
        'next: for x in 0..map.len() - *X_MAX {
            for (dy, dx) in SEA_MONSTER.iter() {
                if map[y + dy][x + dx] != '#' && map[y + dy][x + dx] != 'O' {
                    continue 'next;
                }
            }

            count += 1;
        }
    }

    count
}