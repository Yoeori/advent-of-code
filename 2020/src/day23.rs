use std::collections::HashMap;

pub fn main() {
    let numbers: Vec<usize> = include_str!("../puzzles/23.txt").chars().map(|x| x.to_string().parse().unwrap()).collect();

    let mut lookup = create_cups_list(&numbers.clone());
    let mut cur = numbers[0];
    for _ in 0..100 {
        cur = move_cups(&mut lookup, cur);
    }

    print!("Solution to exercise 1: ");
    let mut pointer = lookup[&1];
    while pointer != 1 {
        print!("{}", pointer);
        pointer = lookup[&pointer];
    }
    println!();


    let mut cups = numbers.clone();
    cups.extend(cups.len()+1..1_000_000+1);
    let mut lookup = create_cups_list(&cups);
    let mut cur = cups[0];
    for _ in 0..10_000_000 {
        cur = move_cups(&mut lookup, cur);
    }

    let n1 = lookup[&1];
    println!("Solution to exercise 2: {}", n1 * lookup[&n1]);

}

fn create_cups_list(cups: &Vec<usize>) -> HashMap<usize, usize> {
    let mut lookup = HashMap::with_capacity(cups.len());
    for (i, &cup) in cups.iter().enumerate() {
        lookup.insert(cup, cups[(i + 1) % cups.len()]);
    }

    lookup
}

fn move_cups(lookup: &mut HashMap<usize, usize>, cur: usize) -> usize {
    // Get cups
    let r1 = lookup[&cur];
    let r2 = lookup[&r1];
    let r3 = lookup[&r2];

    // Find place to place back in
    let dest = {
        let mut dest = cur - 1;
        while dest == r1 || dest == r2 || dest == r3 || dest == 0 {
            if dest == 0 {
                dest = lookup.keys().len();
            } else {
                dest -= 1;
            }
        }

        dest
    };

    // Remove from list
    *lookup.get_mut(&cur).unwrap() = lookup[&r3];

    // Insert after dest
    *lookup.get_mut(&r3).unwrap() = lookup[&dest];
    *lookup.get_mut(&dest).unwrap() = r1;

    // Set cursor to next number
    lookup[&cur]
}