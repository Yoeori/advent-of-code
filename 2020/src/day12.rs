use std::fs;

pub fn main() {
    let file = fs::read_to_string("puzzles/12.txt").unwrap();
    let instr: Vec<(char, isize)> = file.lines().map(|l| (l.chars().nth(0).unwrap(), l[1..].parse().unwrap())).collect();

    let mut ns: isize = 0;
    let mut ew: isize = 0;

    let mut w_ns: isize = 0;
    let mut w_ew: isize = 1;

    for (d, step) in instr.iter() {
        match d {
            'F' => {
                ew += step * w_ew;
                ns += step * w_ns;
            },
            'N' => {
                ns += step;
            },
            'E' => {
                ew += step;
            },
            'S' => {
                ns -= step;
            },
            'W' => {
                ew -= step;
            },
            'R' => {
                if *step == 90 {
                    let tmp = w_ns;
                    w_ns = -w_ew;
                    w_ew = tmp;
                } else if *step == 180 {
                    w_ns = -w_ns;
                    w_ew = -w_ew;
                } else if *step == 270 {
                    let tmp = w_ns;
                    w_ns = w_ew;
                    w_ew = -tmp;
                }
            },
            'L' => {
                if *step == 90 {
                    let tmp = w_ns;
                    w_ns = w_ew;
                    w_ew = -tmp;
                } else if *step == 180 {
                    w_ns = -w_ns;
                    w_ew = -w_ew;
                } else if *step == 270 {
                    let tmp = w_ns;
                    w_ns = -w_ew;
                    w_ew = tmp;
                }
            },
            _ => panic!()
        }
    }

    println!("Solution to exercise 1: {}", ns.abs() + ew.abs());


    let mut s_ns: isize = 0;
    let mut s_ew: isize = 0;

    let mut w_ns: isize = 1;
    let mut w_ew: isize = 10;

    for (d, step) in instr.iter() {
        match d {
            'F' => {
                s_ns += w_ns * step;
                s_ew += w_ew * step;
            },
            'N' => {
                w_ns += step;
            },
            'E' => {
                w_ew += step;
            },
            'S' => {
                w_ns -= step;
            },
            'W' => {
                w_ew -= step;
            },
            'R' => {
                if *step == 90 {
                    let tmp = w_ns;
                    w_ns = -w_ew;
                    w_ew = tmp;
                } else if *step == 180 {
                    w_ns = -w_ns;
                    w_ew = -w_ew;
                } else if *step == 270{
                    let tmp = w_ns;
                    w_ns = w_ew;
                    w_ew = -tmp;
                }
            },
            'L' => {
                if *step == 90 {
                    let tmp = w_ns;
                    w_ns = w_ew;
                    w_ew = -tmp;
                } else if *step == 180 {
                    w_ns = -w_ns;
                    w_ew = -w_ew;
                } else if *step == 270 {
                    let tmp = w_ns;
                    w_ns = -w_ew;
                    w_ew = tmp;
                }
            },
            _ => panic!()
        }
    }

    println!("Solution to exercise 2: {}", s_ns.abs() + s_ew.abs())
    
}