
pub fn main() {
    let mut file = include_str!("../puzzles/25.txt").split('\n').map(|x| x.parse::<usize>().unwrap());
    let room_public_key: usize = file.next().unwrap();
    let card_public_key: usize = file.next().unwrap();

    println!("Solution to exercise 1: {}", transform(card_public_key, calculate_rounds(room_public_key)));
}

fn calculate_rounds(key: usize) -> usize {
    let subject_number = 7;
    let mut start = 1;
    let mut round: usize = 0;
    
    loop {
        start *= subject_number;
        start %= 20201227;
        round += 1;

        if start == key {
            break;
        }
    }

    round
}

fn transform(key: usize, rounds: usize) -> usize {
    let subject_number = key;
    let mut start = 1;

    for _ in 0..rounds {
        start *= subject_number;
        start %= 20201227;
    }

    start
}