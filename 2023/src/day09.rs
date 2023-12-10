use std::fs;

pub fn main() {
    let file: String = fs::read_to_string("puzzles/9.txt").unwrap();

    let sequences: Vec<Vec<i64>> = file
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();

    println!(
        "Exercise 1: {}",
        sequences
            .iter()
            .map(|seq| predict(seq.clone()))
            .sum::<i64>()
    );

    println!(
        "Exercise 2: {}",
        sequences
            .iter()
            .map(|seq| {
                let mut seq = seq.clone();
                seq.reverse();
                predict(seq)
            })
            .sum::<i64>()
    );
}

fn predict(mut inp: Vec<i64>) -> i64 {
    let mut cursor = inp.len();

    loop {
        cursor -= 1;
        for i in 0..cursor {
            inp[i] = inp[i + 1] - inp[i];
        }

        if inp[0..cursor - 1].iter().all(|&x| x == 0) {
            break;
        }
    }

    inp[cursor..].iter().sum::<i64>()
}
