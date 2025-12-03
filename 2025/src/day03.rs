use crate::puzzle::Puzzle;

pub(crate) struct Day03;
impl Puzzle for Day03 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let banks: Vec<Vec<u8>> = inp
            .lines()
            .map(|line| {
                line.chars()
                    .map(|s| s.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let part_one: u64 = banks.iter().map(|bank| find_largest_joltage(bank, 2)).sum();
        let part_two: u64 = banks
            .iter()
            .map(|bank| find_largest_joltage(bank, 12))
            .sum();

        (part_one, part_two)
    }
}

fn find_largest_joltage(bank: &[u8], size: usize) -> u64 {
    if size == 0 {
        return 0;
    }

    let (index, &value) = bank[0..(bank.len() - size + 1)]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();

    find_largest_joltage(&bank[(index + 1)..], size - 1)
        + (value as u64 * 10_u64.pow(size as u32 - 1))
}
