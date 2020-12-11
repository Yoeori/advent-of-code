use std::fs;

const DELTA: &[(isize, isize)] = &[(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

pub fn main() {
    let file = fs::read_to_string("puzzles/11.txt").unwrap();
    
    let mut seats: Vec<Vec<char>> = file.split("\n").map(|l| l.chars().collect()).collect();
    let orig = seats.clone();
    let mut next_seats;

    let mut changed = true;
    while changed == true {
        changed = false;
        next_seats = seats.clone();

        for y in 0..seats.len() {
            for x in 0..seats[y].len() {

                let new = determine(&seats, x as isize, y as isize);

                if seats[y][x] != new {
                    changed = true;
                }

                next_seats[y][x] = new;

            }
        }

        std::mem::swap(&mut seats, &mut next_seats);
    }

    println!("Answer to exercise 1: {}", seats.iter().map(|l| l.iter().filter(|&x| x == &'#').count()).sum::<usize>());


    let mut changed = true;
    let mut seats = orig;
    while changed == true {
        changed = false;
        next_seats = seats.clone();

        for y in 0..seats.len() {
            for x in 0..seats[y].len() {
                let new = determine_impr(&seats, x as isize, y as isize);

                if seats[y][x] != new {
                    changed = true;
                }

                next_seats[y][x] = new;

            }
        }

        std::mem::swap(&mut seats, &mut next_seats);
    }

    println!("Answer to exercise 2: {}", seats.iter().map(|l| l.iter().filter(|&x| x == &'#').count()).sum::<usize>());
}

fn determine_impr(seats: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    let cur = seats[y as usize][x as usize];

    if cur == '.' {
        '.'
    } else {
        let mut count = 0;
        for (dy, dx) in DELTA {
            if try_see(seats, x as isize, y as isize, dy, dx) == '#' {
                count += 1;
            }
        };

        if count >= 5 {
            'L'
        } else if count == 0 {
            '#'
        } else {
            cur
        }
    }
}

fn try_see(seats: &Vec<Vec<char>>, mut x: isize, mut y: isize, dx: &isize, dy: &isize) -> char {
    x += dx;
    y += dy;

    while x >= 0 && y >= 0 && seats.get(y as usize).is_some() && seats[y as usize].get(x as usize).is_some() {
        if seats[y as usize][x as usize] == '#' {
            return '#';
        } else if seats[y as usize][x as usize] == 'L' {
            return 'L';
        }

        x += dx;
        y += dy;

    };

    return '.';
}


fn determine(seats: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    let cur = seats[y as usize][x as usize];

    if cur == '.' {
        '.'
    } else {
        let mut count = 0;
        for (dy, dx) in DELTA {
            if (y + dy) >= 0 && (x + dx) >= 0 && seats.get((y + dy) as usize).is_some() && seats[(y + dy) as usize].get((x + dx) as usize).is_some() && seats[(y + dy) as usize][(x + dx) as usize] == '#' {
                count += 1;
            }
        };

        if count >= 4 {
            'L'
        } else if count == 0 {
            '#'
        } else {
            cur
        }
    }
}