use std::{collections::HashMap};

#[derive(Debug)]
struct Cup {
    val: usize,
    next: usize
}

pub fn main() {
    let numbers: Vec<usize> = include_str!("../puzzles/23.txt").chars().map(|x| x.to_string().parse().unwrap()).collect();

    let mut lookup = create_cups_list(&numbers.clone());
    let mut cur = numbers[0];
    for _ in 0..100 {
        cur = move_cups(&mut lookup, cur);
    }

    print!("Solution to exercise 1: ");
    let mut pointer = lookup[&1].next;
    while pointer != 1 {
        print!("{}", pointer);
        pointer = lookup[&pointer].next;
    }
    println!();


    let mut cups = numbers.clone();
    cups.extend(cups.len()+1..1_000_000+1);
    let mut lookup = create_cups_list(&cups);
    let mut cur = cups[0];
    for _ in 0..10_000_000 {
        cur = move_cups(&mut lookup, cur);
    }

    let n1 = lookup[&1].next;
    println!("Solution to exercise 2: {}", n1 * lookup[&n1].next);

}

fn create_cups_list(cups: &Vec<usize>) -> HashMap<usize, Cup> {
    let mut lookup = HashMap::with_capacity(cups.len());
    for (i, &cup) in cups.iter().enumerate() {
        lookup.insert(cup, Cup {
            val: cup,
            next: cups[(i + 1) % cups.len()]
        });
    }

    lookup
}

fn move_cups(lookup: &mut HashMap<usize, Cup>, cur: usize) -> usize {
    // Get cups
    let r1 = lookup[&cur].next;
    let r2 = lookup[&r1].next;
    let r3 = lookup[&r2].next;

    // Find place to place back in
    let dest = {
        let mut dest = lookup[&cur].val - 1;
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
    lookup.get_mut(&cur).unwrap().next = lookup[&r3].next;

    // Insert after dest
    lookup.get_mut(&r3).unwrap().next = lookup[&dest].next;
    lookup.get_mut(&dest).unwrap().next = r1;

    // Set cursor to next number
    lookup.get_mut(&cur).unwrap().next
}