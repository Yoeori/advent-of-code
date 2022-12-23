#[derive(Debug, Clone)]
struct Entry {
    n: i64,

    left: usize,
    right: usize
}

fn scroll(entries: &Vec<Entry>, i: usize, n: i64) -> usize {
    let mut cur = i;
    for _ in 0..n {
        cur = entries[cur].right;
    }
    cur
}

fn simulate_roll(entries: &mut Vec<Entry>) {
    let k = entries.len() as i64 - 1;
    
    for i in 0..entries.len() {
        // Remove i
        let left_index = entries[i].left;
        entries[left_index].right = entries[i].right;
        let right_index = entries[i].right;
        entries[right_index].left = entries[i].left;

        // Move steps in the ring
        let cur = scroll(&entries, entries[i].right, ((entries[i].n % k) + k) % k);

        // Place left of cursor
        entries[i].right = cur;
        entries[i].left = entries[cur].left;

        entries[cur].left = i;
        let left_index = entries[i].left;
        entries[left_index].right = i;
    }
}

pub fn main() {

    // Poor man's double-linked-list
    let mut entries: Vec<Entry> = include_str!("../puzzles/20.txt").lines().enumerate().map(|(i, x)| Entry {
        n: x.parse().unwrap(),
        left: if i == 0 { 0 } else { i - 1 },
        right: i + 1
    }).collect();

    entries[0].left = entries.len() - 1;
    entries.last_mut().unwrap().right = 0;

    let mut entries2: Vec<Entry>= entries.iter().map(|entry| entry.clone()).map(|mut entry| {
        entry.n = entry.n * 811589153;
        entry
    }).collect();

    simulate_roll(&mut entries);

    println!("Exercise 1: {}", (0..3).fold((entries.iter().position(|r| r.n == 0).unwrap(), 0), |(mut cur, mut t), _| {
        cur = scroll(&entries, cur, 1000);
        t += entries[cur].n;
        (cur, t)
    }).1);

    for _ in 0..10 {
        simulate_roll(&mut entries2);
    }

    println!("Exercise 2: {}", (0..3).fold((entries.iter().position(|r| r.n == 0).unwrap(), 0), |(mut cur, mut t), _| {
        cur = scroll(&entries2, cur, 1000);
        t += entries2[cur].n;
        (cur, t)
    }).1);

}