use std::{collections::HashMap, cmp};

#[derive(Default, Debug, Clone)]
struct Valve {
    flow_rate: i32,
    dist: Vec<i32>
}

pub fn main() {
    let input = include_str!("../puzzles/16.txt");
    let lut: HashMap<&str, usize> = input.lines().map(|l| &l[6..8]).enumerate().map(|(i, n)| (n, i)).collect();

    let mut valves: Vec<Valve> = input.lines().map(|l| {
        let mut v = vec![i32::MAX / 2; lut.len()];
        v[lut[&l[6..8]]] = 0;

        Valve {
            flow_rate: l[23..].split_once(';').unwrap().0.parse().unwrap(),
            dist: if let Some(tunnel) = l.split_once("leads to valve ").unzip().1 {
                v[lut[tunnel]] = 1;
                v
            } else {
                for t in l.split_once("to valves ").unwrap().1.split(", ") {
                    v[lut[t]] = 1;
                }
                v
            }
        }
    }).collect();

    // Floyd-warshall
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                valves[i].dist[j] = cmp::min(valves[i].dist[j], valves[i].dist[k] + valves[k].dist[j]);
            }
        }
    }

    let to_visit: u64 = valves.iter().enumerate().filter(|(_, v)| v.flow_rate == 0).map(|(i, _)| 1 << i).sum();
    let mut trace_lut: HashMap<(usize, i32, u64, bool), i32> = HashMap::new();

    println!("Exercise 1: {}", trace(lut["AA"], lut["AA"], 30, to_visit, &valves, &mut trace_lut, false));
    println!("Exercise 2: {}", trace(lut["AA"], lut["AA"], 26, to_visit, &valves, &mut trace_lut, true));
}


fn trace(start: usize, cur: usize, t: i32, visit: u64, valves: &Vec<Valve>, lut: &mut HashMap<(usize, i32, u64, bool), i32>, elephant: bool) -> i32 {
    if let Some(v) = lut.get(&(cur, t, visit, elephant)) {
        return *v;
    }

    let mut max_v = 0;

    for i in 0..valves.len() {
        if (visit >> i) & 0b1 == 1 || valves[cur].dist[i] > t {
            continue;
        }

        let c = t - (valves[cur].dist[i] + 1);
        max_v = cmp::max(max_v, c * valves[i].flow_rate + trace(start, i, c, visit + (1 << i), valves, lut, elephant));
    }

    if elephant {
        max_v = cmp::max(max_v, trace(start, start, 26, visit, valves, lut, false))
    }

    lut.insert((cur, t, visit, elephant), max_v);
    max_v
}