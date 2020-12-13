use std::fs;
use std::collections::HashSet;

pub fn main() {
    let file = fs::read_to_string("puzzles/8.txt").unwrap();
    
    let mut instructions: Vec<(&str, isize)> = file.split("\n").map(|x| {
        let mut item = x.split(" ");
        let s = item.next().unwrap();

        let n = {
            let r = item.next().unwrap();
            if r.starts_with('+') {
                r[1..].parse().unwrap()
            } else {
                r.parse::<isize>().unwrap()
            }
        };

        (s, n)
    }).collect();

    println!("Answer to exercise 1: {}", execute(&instructions).unwrap_err());

    for i in 0..(instructions.len()) {
        let tmp = instructions[i];

        if let ("nop", x) = tmp {
            let _ = std::mem::replace(&mut instructions[i], ("jmp", x));
        } else if let ("jmp", x) = tmp {
            let _ = std::mem::replace(&mut instructions[i], ("nop", x));
        } else {
            continue;
        }

        let res = execute(&instructions);
        let _ = std::mem::replace(&mut instructions[i], tmp);
        
        if let Ok(res) = res {
            println!("Answer to exercise 2: {:?}", res);
            break;
        }
    }
}

fn execute(instructions: &Vec<(&str, isize)>) -> Result<isize, isize> {
    let mut i: isize = 0;
    let mut acc: isize = 0;
    let mut visited: HashSet<isize> = HashSet::new();
    
    while !visited.contains(&i) && (i as usize) < instructions.len() {
        visited.insert(i);

        match instructions.get(i as usize).unwrap() {
            ("nop", _) => {
                i += 1;
            }
            ("acc", n) => {
                acc += n;
                i += 1;
            }
            ("jmp", n) => {
                i += *n;
            }
            _ => {
                println!("Unknown instruction!")
            }
        }
    }

    if (i as usize) < instructions.len() {
        Err(acc)
    } else {
        Ok(acc)
    }
}