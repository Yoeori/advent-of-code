use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Moon {
    name: usize,

    x: i64,
    y: i64,
    z: i64,

    x_velo: i64,
    y_velo: i64,
    z_velo: i64
}

impl Moon {
    fn parse(inp: &str, name: usize) -> Self {
        let inp: HashMap<&str, i64> = inp[1..(inp.len() - 1)].split(", ").map(|inp| inp.split("=").collect())
                                                             .map(|inp: Vec<&str>| (inp[0], inp[1].parse().unwrap())).collect();
        
        Moon {
            name: name,
            x: inp["x"],
            y: inp["y"],
            z: inp["z"],

            ..Default::default()
        }
    }

    fn potential_energy(&self) -> i64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }

    fn kinetic_energy(&self) -> i64 {
        return self.x_velo.abs() + self.y_velo.abs() + self.z_velo.abs();
    }

    fn total_energy(&self) -> i64 {
        return self.kinetic_energy() * self.potential_energy();
    }

    fn update_velocity(&self, moons: &Vec<Moon>) -> Self {
        let mut res = self.clone();

        for moon in moons {
            if moon != self {
                Moon::update_velocity_once(&mut res, moon);
            }
        }

        return res;
    }

    fn update_velocity_once(moon: &mut Moon, other: &Moon) {
        moon.x_velo += if moon.x == other.x { 0 } else if moon.x < other.x { 1 } else { -1 };
        moon.y_velo += if moon.y == other.y { 0 } else if moon.y < other.y { 1 } else { -1 };
        moon.z_velo += if moon.z == other.z { 0 } else if moon.z < other.z { 1 } else { -1 };
    }

    fn velocity_apply(&mut self) {
        self.x += self.x_velo;
        self.y += self.y_velo;
        self.z += self.z_velo;
    }

    fn no_velocity(&self) -> bool {
        return self.x_velo == 0 && self.y_velo == 0 && self.z_velo == 0;
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/12.txt").unwrap();
    let actual_moons: Vec<Moon> = file_contents.split("\n").enumerate().map(|(i, moon)| Moon::parse(moon, i)).collect();

    let mut moons = actual_moons.clone();
    let mut no_velo: HashMap<usize, i64> = HashMap::new();

    for i in 0..1000 {
        moons = moons.iter().map(|x| x.update_velocity(&moons)).collect();
        moons.iter_mut().for_each(|moon| moon.velocity_apply());

        moons.iter().for_each(|moon| {
            if moon.no_velocity() && !no_velo.contains_key(&moon.name) {
                no_velo.insert(moon.name, i);
            }
        });
    }

    println!("Solution to exercise 1: {}", moons.iter().map(|moon| moon.total_energy()).sum::<i64>());

    let moons = actual_moons.clone();
    let x = simulate_dir(moons.iter().map(|moon| moon.x).collect());
    let y = simulate_dir(moons.iter().map(|moon| moon.y).collect());
    let z = simulate_dir(moons.iter().map(|moon| moon.z).collect());

    println!("Solution to exercise 2: {}", lcm(x, lcm(y, z)));
}

fn simulate_dir(mut positions: Vec<i64>) -> i64 {
    let mut iterations = 0;
    let mut velocities = vec![0; positions.len()];

    loop {
        let mut diff = vec![0; positions.len()];

        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in positions.iter() {
                if pos1 != pos2 {
                    diff[i] += if pos1 < pos2 { 1 } else { -1 }
                }
            }
        }

        for (v, v_diff) in velocities.iter_mut().zip(diff) {
            *v += v_diff;
        }

        for (p, a) in positions.iter_mut().zip(&velocities) {
            *p += a;
        }

        iterations += 1;
        
        if velocities.iter().all(|v| *v == 0) {
            break;
        }
    }

    // After one iteration we should be at the exact inverse condition, after doing it twice we should be back at start
    return iterations * 2;
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}