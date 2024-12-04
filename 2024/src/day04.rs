use std::{collections::HashSet, ops::Add};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Delta(isize, isize);

impl Add<Delta> for Pos {
    type Output = Pos;

    fn add(self, rhs: Delta) -> Self::Output {
        Pos(
            ((self.0 as isize) + rhs.0) as usize,
            ((self.1 as isize) + rhs.1) as usize,
        )
    }
}

const PATTERN: &str = "XMAS";
const PATTERN_CROSS: &str = "MAS";

const EMPTY_ELEMENT: [(char, Pos); 1] = [('-', Pos(usize::MAX, usize::MAX))];

pub(crate) struct Day04;
impl Puzzle for Day04 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let grid = inp.lines().collect::<Vec<&str>>();

        // Wait it's all iterators? 🔫 Always has been
        let rows = (0..grid.len()).map(|i| iter_grid(&grid, Pos(i, 0), Delta(0, 1)));
        let columns = (0..grid[0].len()).map(|i| iter_grid(&grid, Pos(0, i), Delta(1, 0)));
        let diag_plus = (0..grid.len())
            .map(|i| iter_grid(&grid, Pos(i, 0), Delta(1, 1)))
            .chain((1..grid[0].len()).map(|i| iter_grid(&grid, Pos(0, i), Delta(1, 1))));
        let diag_min = (0..grid[0].len())
            .map(|i| iter_grid(&grid, Pos(0, i), Delta(1, -1)))
            .chain(
                (1..grid.len()).map(|i| iter_grid(&grid, Pos(i, grid[i].len() - 1), Delta(1, -1))),
            );

        let occurrances = matches(
            rows.chain(columns)
                .chain(diag_plus.clone())
                .chain(diag_min.clone())
                .flat_map(|it| it.chain(EMPTY_ELEMENT)),
            PATTERN,
        );

        let crosses = cross(
            diag_plus.flat_map(|it| it.chain(EMPTY_ELEMENT)),
            diag_min.flat_map(|it| it.chain(EMPTY_ELEMENT)),
        );

        (occurrances.len(), crosses.len())
    }
}

fn iter_grid<'a>(
    grid: &'a [&'a str],
    start: Pos,
    delta: Delta,
) -> impl DoubleEndedIterator<Item = (char, Pos)> + use<'a> + Clone {
    (0..(grid.len() as isize))
        .map(move |v| Delta(delta.0 * v, delta.1 * v))
        .map(move |delta| start + delta)
        .filter(|&Pos(x, y)| x < grid.len() && y < grid[x].len())
        .map(|pos: Pos| (grid[pos.0].chars().nth(pos.1).unwrap(), pos))
}

fn matches(iter: impl DoubleEndedIterator<Item = (char, Pos)> + Clone, pattern: &str) -> Vec<Pos> {
    let (str, pos): (String, Vec<Pos>) =
        iter.clone().chain(EMPTY_ELEMENT).chain(iter.rev()).unzip();

    str.match_indices(pattern)
        .map(|(index, _)| pos[index + 1])
        .collect()
}

fn cross(
    left: impl DoubleEndedIterator<Item = (char, Pos)> + Clone,
    right: impl DoubleEndedIterator<Item = (char, Pos)> + Clone,
) -> Vec<Pos> {
    HashSet::<Pos>::from_iter(matches(left, PATTERN_CROSS))
        .intersection(&HashSet::<Pos>::from_iter(matches(right, PATTERN_CROSS)))
        .copied()
        .collect()
}
