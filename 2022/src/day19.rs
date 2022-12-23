use std::cmp;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

type Coll = (u32, u32, u32, u32);

pub fn main() {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();

    let blueprints: Vec<Blueprint> = re.captures_iter(include_str!("../puzzles/19.txt")).map(|cap| {
        Blueprint {
            ore: cap[2].parse().unwrap(),
            clay: cap[3].parse().unwrap(),
            obsidian: (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
            geode: (cap[6].parse().unwrap(), cap[7].parse().unwrap()),
        }
    }).collect();

    println!("Exercise 1: {}", blueprints.iter().enumerate().map(|(i, blueprint)| simulate_blueprint(blueprint, 24) * (i as u32 + 1)).sum::<u32>());
    println!("Exercise 2: {}", blueprints[0..cmp::min(3, blueprints.len())].iter().map(|blueprint| simulate_blueprint(blueprint, 32)).product::<u32>());
}

fn simulate_blueprint(bp: &Blueprint, mins: u32) -> u32 {
    let mut max = 0;
    dfs_blueprint(bp, mins, (1, 0, 0, 0), (0, 0, 0, 0), &mut max);
    max
}

fn dfs_blueprint(bp: &Blueprint, mins: u32, rb:Coll, rs_old: Coll, max_geode: &mut u32) {
    // Simulate minute
    let rs = (rs_old.0 + rb.0, rs_old.1 + rb.1, rs_old.2 + rb.2, rs_old.3 + rb.3);
    *max_geode = cmp::max(*max_geode, rs.3);

    // Check if we're finished _or_ if it's impossible to improve in this branch
    if mins == 1 || (mins - 1) * rb.3 + (mins*mins)/2 < (*max_geode - rs.3) {
        return;
    }

    // Buy something or don't buy anything
    if rs_old.0 >= bp.geode.0 && rs_old.2 >= bp.geode.1 {
        dfs_blueprint(bp, mins - 1, (rb.0, rb.1, rb.2, rb.3 + 1), (rs.0 - bp.geode.0, rs.1, rs.2 - bp.geode.1, rs.3), max_geode)
    }

    if rb.2 < bp.geode.1 && rs_old.0 >= bp.obsidian.0 && rs_old.1 >= bp.obsidian.1 {
        dfs_blueprint(bp, mins - 1, (rb.0, rb.1, rb.2 + 1, rb.3), (rs.0 - bp.obsidian.0, rs.1 - bp.obsidian.1, rs.2, rs.3), max_geode)
    }
    
    if rb.1 < bp.obsidian.1 && rs_old.0 >= bp.clay {
        dfs_blueprint(bp, mins - 1, (rb.0, rb.1 + 1, rb.2, rb.3), (rs.0 - bp.clay, rs.1, rs.2, rs.3), max_geode)
    }

    if rb.0 < cmp::max(cmp::max(bp.ore, bp.clay), cmp::max(bp.geode.0, bp.obsidian.0)) && rs_old.0 >= bp.ore {
        dfs_blueprint(bp, mins - 1, (rb.0 + 1, rb.1, rb.2, rb.3), (rs.0 - bp.ore, rs.1, rs.2, rs.3), max_geode)
    }

    // Don't buy anything
    dfs_blueprint(bp, mins - 1, rb, rs, max_geode);
}