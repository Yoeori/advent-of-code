use crate::puzzle::Puzzle;

pub(crate) struct Day02;
impl Puzzle for Day02 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        // Calculate all invalid ids,
        let invalid_ids: Vec<u64> = (1..99_999)
            .map(|n| repeat_number(n).next().unwrap())
            .collect();

        let mut invalid_ids_two: Vec<u64> = (1..99_999)
            .flat_map(|n| repeat_number(n).take(8))
            .collect();

        invalid_ids_two.sort();
        invalid_ids_two.dedup();

        let ranges: Vec<(u64, u64)> = inp
            .split(',')
            .map(|l| l.split_once('-').unwrap())
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .collect();

        let mut count = 0;

        for &(start, end) in &ranges {
            let start_idx = binary_search(start, &invalid_ids);
            let end_idx = binary_search(end + 1, &invalid_ids);

            count += &invalid_ids[start_idx..end_idx].iter().sum::<u64>();
        }

        let mut count_2 = 0;

        for &(start, end) in &ranges {
            let start_idx = binary_search(start, &invalid_ids_two);
            let end_idx = binary_search(end + 1, &invalid_ids_two);

            count_2 += &invalid_ids_two[start_idx..end_idx].iter().sum::<u64>();
        }

        (count, count_2)
    }
}

/// Find first index higher or equal to needle in haystack
fn binary_search(needle: u64, haystack: &[u64]) -> usize {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let mid = (low + high) / 2;
        let val = haystack[mid];

        if val < needle {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

fn repeat_number(n: u64) -> impl Iterator<Item = u64> {
    let modifier = 10u64.pow(n.ilog10() + 1);

    (0..).scan(n, move |state, _| {
        *state = state.checked_mul(modifier)?.checked_add(n)?;
        Some(*state)
    })
}