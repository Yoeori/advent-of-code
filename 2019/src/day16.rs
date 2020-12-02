use std::fs;
use itertools::Itertools;
use std::ops::Range;
use std::cmp;

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/16.txt").unwrap();
    let puzzle: Vec<i32> = file_contents.chars().map(|c| c.to_digit(10).expect("Puzzle contents should only contain integers!") as i32).collect();

    let mut res1 = puzzle.clone();
    phase(&mut res1, 100);
    println!("Solution to exercise 1: {}", res1.iter().take(8).map(|x| x.to_string()).join(""));

    let mut res2: Vec<i32> = puzzle.iter().cycle().take(puzzle.len() * 10_000).map(|x| *x).collect();
    let calc_index = puzzle.iter().enumerate().map(|(i, x)| x * 10i32.pow((6-i) as u32)).take(7).sum::<i32>() as usize;

    phase_last_half(&mut res2, 100, calc_index);
    println!("Solution to exercise 2: {}", res2.iter().skip(calc_index).take(8).map(|x| x.to_string()).join(""));
}

fn phase(phase: &mut [i32], rounds: usize) {

    // Memory efficient! :D
    let plus:  Vec<Vec<Range<usize>>> = (1..=phase.len()).map(
        |i| (0..).map(|pos| pos * i * 4 + i - 1)
                 .map(|st_pos| (st_pos..cmp::min(st_pos+i, phase.len())))
                 .take_while(|range| range.start < phase.len()).collect()
    ).collect();

    let minus: Vec<Vec<Range<usize>>> = (1..=phase.len()).map(
        |i| (0..).map(|pos| pos * i * 4 + 3 * i - 1)
                 .map(|st_pos| (st_pos..cmp::min(st_pos+i, phase.len())))
                 .take_while(|range| range.start < phase.len()).collect()
    ).collect();

    for _ in 0..rounds {
        for i in 0..phase.len() {
            let mut total = 0;

            // plus
            for r in &plus[i] {
                for plus_index in r.start..r.end {
                    total += phase[plus_index];
                }
            }

            // minus
            for r in &minus[i] {
                for minus_index in r.start..r.end {
                    total -= phase[minus_index];
                }
            }

            phase[i] = (total % 10).abs();
        }
    }
}

// Function which only works for calculating phases in the last half of the phase
fn phase_last_half(phase: &mut [i32], rounds: usize, start: usize) {
    for _ in 0..rounds {
        let mut total: i32 = 0;

        for i in (start..phase.len()).rev() {
            total += phase[i];
            total %= 10;

            phase[i] = total;
        }
    }
}