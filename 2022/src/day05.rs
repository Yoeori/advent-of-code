pub fn main() {
    let (initial_state, commands) = include_str!("../puzzles/5.txt").split_once("\n\n").unwrap();

    let mut stacks_1: Vec<Vec<char>> = vec![vec![]; initial_state.lines().next().unwrap().len() / 4 + 1];

    for line in initial_state.lines().rev().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        for idx in 0..(chars.len()/4+1) {
            if chars[(idx * 4) + 1] != ' ' {
                stacks_1[idx].push(chars[(idx * 4) + 1]);
            }
        }
    }

    let mut stacks_2 = stacks_1.clone();

    for command in commands.lines().map(|l| l.split(' ').collect::<Vec<&str>>()) {
        let (amount, from, to) = (command[1].parse::<usize>().unwrap(), command[3].parse::<usize>().unwrap(), command[5].parse::<usize>().unwrap());

        let l = stacks_2[from - 1].len();

        for _ in 0..amount {
            let v = stacks_1[from - 1].pop().unwrap();
            stacks_1[to - 1].push(v);

            let v = stacks_2[from - 1].remove( l - amount);
            stacks_2[to - 1].push(v);
        }
    }

    print!("Exercise 1: ");
    for stack in &stacks_1 {
        if !stack.is_empty() {
            print!("{}", stack.last().unwrap());
        } else {
            break;
        }
    }
    println!();

    print!("Exercise 2: ");
    for stack in &stacks_2 {
        if !stack.is_empty() {
            print!("{}", stack.last().unwrap());
        } else {
            break;
        }
    }
    println!();
    
}