use std::ops::Range;

use crate::puzzle::Puzzle;

pub(crate) struct Day05;
impl Puzzle for Day05 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (text_ranges, text_ingredients) = inp.split_once("\n\n").unwrap();

        let mut ranges: Vec<Range<u64>> = text_ranges
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .map(|(start, end)| start.parse::<u64>().unwrap()..(end.parse::<u64>().unwrap() + 1))
            .collect();

        ranges.sort_by_key(|range| range.start);

        let ingredients: Vec<u64> = text_ingredients
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        // Simplify ranges
        let mut composite_ranges: Vec<Range<u64>> = Vec::new();

        for range in ranges.iter() {
            if let Some(last_range) = composite_ranges.last_mut()
                && last_range.contains(&range.start)
            {
                last_range.end = last_range.end.max(range.end);
            } else {
                composite_ranges.push(range.clone());
            }
        }

        // Part one
        let mut fresh_ingredients_count = 0;
        for ingredient in &ingredients {
            'innerloop: for range in &composite_ranges {
                if range.contains(ingredient) {
                    fresh_ingredients_count += 1;
                    break 'innerloop;
                }
            }
        }

        // Part two
        let total_count = composite_ranges.iter().map(|r| r.end - r.start).sum();

        (fresh_ingredients_count, total_count)
    }
}
