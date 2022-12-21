use std::{collections::{HashMap, HashSet}, cmp};

#[derive(Default, Debug, Clone)]
struct Valve<T> {
    name: T,
    flow_rate: i32,
    dist: HashMap<T, i32>
}

pub fn main() {
    let mut valves: HashMap<&str, Valve<&str>> = include_str!("../puzzles/16.txt").lines().map(|l|
        (&l[6..8], Valve {
            name: &l[6..8],
            flow_rate: l[23..].split_once(';').unwrap().0.parse().unwrap(),
            dist: if let Some(tunnel) = l.split_once("leads to valve ").unzip().1 {
                HashMap::from([(&l[6..8], 0), (tunnel, 1)])
            } else {
                let mut m: HashMap<&str, i32> = l.split_once("to valves ").unwrap().1.split(", ").map(|t| (t, 1)).collect();
                m.insert(&l[6..8], 0);
                m
            },
            ..Default::default()
        })
    ).collect();

    let keys: Vec<&str> = valves.keys().map(|x| *x).collect();
    for i in valves.values_mut() {
        for j in keys.iter() {
            i.dist.entry(j).or_insert(i32::MAX / 2);
        }
    }

    // Floyd-warshall
    for k in keys.iter() {
        for i in keys.iter() {
            for j in keys.iter() {
                // tl/dr: fighting the borrow checker
                let ik = *valves.get(i).unwrap().dist.get(k).unwrap();
                let kj = *valves.get(k).unwrap().dist.get(j).unwrap();

                let ij = valves.get_mut(i).unwrap().dist.get_mut(j).unwrap();
                *ij = cmp::min(*ij, ik + kj);
            }
        }
    }

    let interesting_keys: Vec<&str> = valves.values().filter(|valve| valve.flow_rate > 0).map(|valve| valve.name).collect();
    println!("Exercise 1: {}", trace("AA", 30, &valves, &mut interesting_keys.iter().map(|x| *x).collect(), false));
    println!("Exercise 2: {}", trace("AA", 26, &valves, &mut interesting_keys.iter().map(|x| *x).collect(), true));
    
}


fn trace<'a>(cur: &'a str, t: i32, valves: &HashMap<&str, Valve<&str>>, visit: &mut HashSet<&'a str>, split: bool) -> i32 {
    let mut max_v = 0;
    let valve = valves.get(cur).unwrap();

    for v in visit.iter() {
        if *valve.dist.get(v).unwrap() > t {
            continue;
        }

        let mut n = visit.clone();
        n.remove(v);

        let c = t - (valve.dist.get(v).unwrap() + 1);
        max_v = cmp::max(max_v, c * valves.get(v).unwrap().flow_rate + trace(v, c, valves, &mut n, split))
    }

    if split {
        max_v = cmp::max(max_v, trace("AA", 26, valves, visit, false));
    }

    max_v
}