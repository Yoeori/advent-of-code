use std::{collections::HashMap, fs};

pub fn main() {
    let file: String = fs::read_to_string("puzzles/8.txt").unwrap();
    let (steps, network_str) = file.split_once("\n\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in network_str.lines() {
        network.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    let mut cur: &str = "AAA";
    for (i, step) in steps.chars().cycle().enumerate() {
      if step == 'L' {
        cur = network.get(cur).unwrap().0;
      } else if step == 'R' {
        cur = network.get(cur).unwrap().1;
      }

      if cur == "ZZZ" {
        println!("Exercise 1: {}", i + 1);
        break;
      }
    }

    let cursors: Vec<&str> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|&x| x)
        .collect();

    println!(
        "Exercise 2: {}, ",
        cursors
            .iter()
            .map(|&cursor| single_cursor_pattern(cursor, &network, steps))
            .fold(1, |a, b| lcm(a, b))
    );
}

fn single_cursor_pattern<'a>(
    mut cur: &'a str,
    network: &HashMap<&'a str, (&'a str, &'a str)>,
    steps: &str,
) -> usize {
    let mut last_v = 0;
    let mut prev_d = 0;

    for (i, step) in steps.chars().cycle().enumerate() {
        if step == 'L' {
            cur = network.get(cur).unwrap().0;
        } else if step == 'R' {
            cur = network.get(cur).unwrap().1;
        }

        if cur.ends_with('Z') {
            if i - last_v == prev_d {
                break;
            }
            prev_d = i - last_v;
            last_v = i;
        }
    }

    prev_d
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
