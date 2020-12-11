use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const DELTA: &[(isize, isize)] = &[(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

#[derive(Debug, Clone)]
struct Pos {
    c: char,
    next: Option<char>,
    neighbours: HashMap<(isize, isize), Rc<RefCell<Pos>>>
}

impl Pos {
    fn new(c: char) -> Self {
        Pos {
            c: c,
            next: None,
            neighbours: HashMap::with_capacity(8)
        }
    }

    fn next(&self) -> char {
        if self.c == '.' {
            '.'
        } else {
            let count = self.neighbours.values().filter(|n| n.borrow().c == '#').count();
            if count >= 4 {
                'L'
            } else if count == 0 {
                '#'
            } else {
                self.c
            }
        }
    }

    fn next_sight(&self) -> char {
        if self.c == '.' {
            '.'
        } else {
            let count = self.neighbours.iter().filter(|(dir, elem)| elem.borrow().sight(dir) == '#').count();
            if count >= 5 {
                'L'
            } else if count == 0 {
                '#'
            } else {
                self.c
            }
        }
    }

    fn sight(&self, dir: &(isize, isize)) -> char {
        if self.c == '.' && self.neighbours.contains_key(dir) {
            self.neighbours.get(dir).unwrap().borrow().sight(dir)
        } else {
            self.c
        }
    }
}

pub fn main() {
    let file = fs::read_to_string("puzzles/11.txt").unwrap();

    let seats: Vec<Vec<Rc<RefCell<Pos>>>> = file.split("\n").map(|l| l.chars().map(|c| Rc::new(RefCell::new(Pos::new(c)))).collect()).collect();
    let seats2: Vec<Vec<Rc<RefCell<Pos>>>> = file.split("\n").map(|l| l.chars().map(|c| Rc::new(RefCell::new(Pos::new(c)))).collect()).collect();
    
    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            for (dy, dx) in DELTA {
                if ((y as isize) + dy) >= 0 && ((x as isize) + dx) >= 0 && (((y as isize) + dy) as usize) < seats.len() && (((x as isize) + dx) as usize) < seats[y].len()  {
                    seats[y][x].borrow_mut().neighbours.insert((*dy, *dx), Rc::clone(&seats[(y as isize + dy) as usize][(x as isize + dx) as usize]));
                    seats2[y][x].borrow_mut().neighbours.insert((*dy, *dx), Rc::clone(&seats2[(y as isize + dy) as usize][(x as isize + dx) as usize]));
                }
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for y in 0..seats.len() {
            for x in 0..seats.len() {
                let mut seat = seats[y][x].borrow_mut();
                seat.next = Some(seat.next());
            }
        }

        for y in 0..seats.len() {
            for x in 0..seats.len() {
                let mut seat = seats[y][x].borrow_mut();
                if seat.next.unwrap() != seat.c {
                    seat.c = seat.next.unwrap();
                    changed = true;
                }
                seat.next = None;
            }
        }
    }

    println!("Answer to exercise 1: {}", seats.iter().map(|l| l.iter().filter(|&x| x.borrow().c == '#').count()).sum::<usize>());

    let seats = seats2;
    let mut changed = true;
    
    while changed {
        changed = false;

        for y in 0..seats.len() {
            for x in 0..seats.len() {
                let mut seat = seats[y][x].borrow_mut();
                seat.next = Some(seat.next_sight());
            }
        }

        for y in 0..seats.len() {
            for x in 0..seats.len() {
                let mut seat = seats[y][x].borrow_mut();
                if seat.next.unwrap() != seat.c {
                    seat.c = seat.next.unwrap();
                    changed = true;
                }
                seat.next = None;
            }
        }
    }

    println!("Answer to exercise 2: {}", seats.iter().map(|l| l.iter().filter(|&x| x.borrow().c == '#').count()).sum::<usize>());
}