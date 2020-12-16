use std::{collections::HashSet, fs, collections::HashMap};
use regex::Regex;

pub fn main() {
    let re = Regex::new(r"([\w| ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    
    let file = fs::read_to_string("puzzles/16.txt").unwrap();
    let mut sections = file.split("\n\n");

    let ticket_rules: HashMap<String, HashSet<usize>> = re.captures_iter(&sections.next().unwrap()).map(|cap| {
        let mut valid_set: HashSet<usize> = HashSet::new();
        valid_set.extend((cap[2].parse::<usize>().unwrap())..=(cap[3].parse::<usize>().unwrap()));
        valid_set.extend((cap[4].parse::<usize>().unwrap())..=(cap[5].parse::<usize>().unwrap()));
        (cap[1].to_string(), valid_set)
    }).collect();

    // My ticket
    let my_ticket: Vec<usize> = sections.next().unwrap().split('\n').nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect();

    let nearby_tickets: Vec<Vec<usize>> = sections.next().unwrap().split('\n')
        .skip(1).map(|l| l.split(',').map(|x| x.parse::<usize>().unwrap()).collect()).collect();

    let mut error_rate = 0;
    for ticket in &nearby_tickets {
        for ticket_num in ticket {
            if !match_any(&ticket_rules, *ticket_num) {
                error_rate += ticket_num;
            }
        }   
    }

    println!("Solution to exercise 1: {}", error_rate);

    let nearby_tickets: Vec<Vec<usize>> = nearby_tickets.into_iter().filter(|rs| rs.iter().all(|&r| match_any(&ticket_rules, r))).collect();
    let mut class_map: HashMap<&String, HashSet<usize>> = HashMap::new();

    for (class, range) in &ticket_rules {
        class_map.insert(&class, HashSet::new());

        'innerloop: for i in 0..nearby_tickets[0].len() {
            for ticket in &nearby_tickets {
                if !range.contains(&ticket[i]) {
                    continue 'innerloop;
                }
            }

            class_map.get_mut(&class).unwrap().insert(i);
        }

    }

    let mut rem_set: HashSet<usize> = HashSet::new();
    while !class_map.iter().all(|(_, s)| s.len() == 1) {
        for (_, map) in class_map.iter_mut() {
            if map.len() == 1 {
                rem_set.extend(map.iter());
            } else {
                *map = map.difference(&rem_set).map(|x | *x).collect();
            }
        }
    }

    println!("Solution to exercise 2: {:?}", 
        class_map.iter().filter(|(class, _)| class.starts_with("departure")).map(|(_, map)| my_ticket[*map.iter().next().unwrap()]).product::<usize>());
}

fn match_any(rules: &HashMap<String, HashSet<usize>>, n: usize) -> bool {
    rules.values().find(|x| x.contains(&n)).is_some()
}