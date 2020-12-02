use std::fs;

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/02.txt").unwrap();
    let initial_memory: Vec<i64> = file_contents.split(",").map(|x| x.parse().unwrap()).collect();

    let mut memory = initial_memory.clone();
    memory[1] = 12;
    memory[2] = 2;
    println!("Initial solution: {}", super::intcode_computer::simulate_computer(memory));

    // So sorry but too lazy to build a solver
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = initial_memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let result = super::intcode_computer::simulate_computer(memory);

            if result == 19690720 {
                println!("100 * noun + verb = {}", (100 * noun) + verb);
                break;
            }
        }
    }
}