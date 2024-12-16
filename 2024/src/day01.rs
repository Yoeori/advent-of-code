use std::collections::HashMap;

use crate::puzzle::Puzzle;

pub(crate) struct Day01;
impl Puzzle for Day01 {
    type Part1 = u32;
    type Part2 = u32;
    
    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (mut l1, mut l2): (Vec<u32>, Vec<u32>) = inp
            .lines()
            .map(|s| {
                let mut it = s.split("   ").map(|x| x.parse::<u32>().unwrap());
                (it.next().unwrap(), it.next().unwrap())
            })
            .unzip();
    
        l1.sort();
        l2.sort();
    
        let total_diff = l1
            .iter()
            .zip(l2.iter())
            .map(|(&v1, &v2)| v1.abs_diff(v2))
            .sum::<u32>();
    
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for n in l2 {
            counts.entry(n).and_modify(|k| *k += 1).or_insert(1);
        }
    
        let similarity_score = l1
            .iter()
            .map(|v| v * counts.get(v).unwrap_or(&0))
            .sum::<u32>();

        (total_diff, similarity_score)

    }
}