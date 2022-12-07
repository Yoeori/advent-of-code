use std::fs;

fn score(p1: char, p2: char) -> usize {
    match p1 {
        'A' => {
            match p2 {
                'X' => 1 + 3,
                'Y' => 2 + 6,
                'Z' => 3 + 0,
                _ => panic!()
            }
        }
        'B' => {
            match p2 {
                'X' => 1 + 0,
                'Y' => 2 + 3,
                'Z' => 3 + 6,
                _ => panic!()
            }
        }
        'C' => {
            match p2 {
                'X' => 1 + 6,
                'Y' => 2 + 0,
                'Z' => 3 + 3,
                _ => panic!()
            }
        },
        _ => panic!()
    }
}

fn score_2(p1: char, p2: char) -> usize {
    match p1 {
        'A' => {
            match p2 {
                'X' => 3 + 0,
                'Y' => 1 + 3,
                'Z' => 2 + 6,
                _ => panic!()
            }
        }
        'B' => {
            match p2 {
                'X' => 1 + 0,
                'Y' => 2 + 3,
                'Z' => 3 + 6,
                _ => panic!()
            }
        }
        'C' => {
            match p2 {
                'X' => 2 + 0,
                'Y' => 3 + 3,
                'Z' => 1 + 6,
                _ => panic!()
            }
        },
        _ => panic!()
    }
}



pub fn main() {
    let file = fs::read_to_string("puzzles/2.txt").unwrap();

    println!("Exercise 1: {}", file.lines().map(|l| {
        let mut l = l.split(' ');
        score(l.next().unwrap().chars().next().unwrap(), l.next().unwrap().chars().next().unwrap())
    }).sum::<usize>());

    println!("Exercise 2: {}", file.lines().map(|l| {
        let mut l = l.split(' ');
        score_2(l.next().unwrap().chars().next().unwrap(), l.next().unwrap().chars().next().unwrap())
    }).sum::<usize>());

}