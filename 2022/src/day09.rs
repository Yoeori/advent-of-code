use std::collections::HashSet;

const LENGTH: usize = 10;

pub fn main() {
    let mut passed_positions_1: HashSet<(i32, i32)> = HashSet::new();
    let mut passed_positions_9: HashSet<(i32, i32)> = HashSet::new();

    let positions = &mut [(0i32, 0i32); LENGTH];

    for (command, n) in include_str!("../puzzles/9.txt").split('\n').map(|x| x.split_once(' ').unwrap()).map(|(x0, x1)| (x0, x1.parse::<i32>().unwrap())) {
        for _ in 0..n {
            match command {
                "R" => positions[0].0 += 1,
                "U" => positions[0].1 -= 1,
                "L" => positions[0].0 -= 1,
                "D" => positions[0].1 += 1,
                _ => panic!() // Invalid move
            }

            for i in 1..LENGTH {
                if (positions[i - 1].0 - positions[i].0).pow(2) + (positions[i - 1].1 - positions[i].1).pow(2) >= 4 {
                    positions[i].1 = if (positions[i - 1].1 - positions[i].1).abs() == 1 {
                        positions[i - 1].1
                    } else {
                        (positions[i - 1].1 + positions[i].1) / 2
                    };

                    positions[i].0 = if (positions[i - 1].0 - positions[i].0).abs() == 1 {
                        positions[i - 1].0
                    } else {
                        (positions[i - 1].0 + positions[i].0) / 2
                    };
                }
            }

            passed_positions_1.insert(positions[1]);
            passed_positions_9.insert(positions[9]);
        }
    }

    println!("Exercise 1: {}", passed_positions_1.len());
    println!("Exercise 2: {}", passed_positions_9.len());

}