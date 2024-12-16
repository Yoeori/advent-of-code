use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(isize, isize);

pub(crate) struct Day08;
impl Puzzle for Day08 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {

      let mut antennas_collection: HashMap<char, Vec<Pos>> = HashMap::new();

      let height = inp.lines().count() as isize;
      let width = inp.lines().next().unwrap().len() as isize;

      for (y, line) in inp.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
          match c {
            'A'..='Z' | '0'..='9' | 'a'..='z' => {
              antennas_collection.entry(c).or_default().push(Pos(y as isize, x as isize));
            },
            _ => {}
          }
        }
      }

      let mut antinodes: HashSet<Pos> = HashSet::new();
      let mut all_antinodes: HashSet<Pos> = HashSet::new();

      for (_, antennas) in antennas_collection.iter() {
        for i in 0..antennas.len() {
          for j in 0..antennas.len() {
            if i == j {
              continue;
            }

            let Pos(y1, x1) = antennas[i];
            let Pos(y2, x2) = antennas[j];

            'innerloop:
            for t in 0.. {
              let y = y1 - (t * (y2 - y1));
              let x = x1 - (t * (x2 - x1));

              if y >= 0 && y < height && x >= 0 && x < width {
                all_antinodes.insert(Pos(y, x));

                if t == 1 {
                  antinodes.insert(Pos(y, x));
                }

              } else {
                break 'innerloop;
              }
            }
          }
        }
      }

      (antinodes.len(), all_antinodes.len())
    }
}