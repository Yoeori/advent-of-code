use std::fmt::Debug;

pub(crate) trait PrintablePuzzle {
    fn exec(&self, inp: &str);
}

pub(crate) trait Puzzle {
    type Part1;
    type Part2;

    fn part1(&self, inp: &str) -> Self::Part1 {
        self.solve(inp).0
    }

    fn part2(&self, inp: &str) -> Self::Part2 {
        self.solve(inp).1
    }

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        (self.part1(inp), self.part2(inp))
    }
}

impl<A, B, T> PrintablePuzzle for T where A: Debug, B: Debug, T: Puzzle<Part1 = A, Part2 = B> {
    fn exec(&self, inp: &str) {
        let (a, b) = self.solve(inp);
        println!("Exercise 1: {:?}", a);
        println!("Exercise 2: {:?}", b);
    }
}

#[macro_export]
macro_rules! test_puzzle {
    ($struct: ident) => {
        #[cfg(test)]
        mod test {
            // use super::$struct;
            // use $crate::puzzle::Puzzle;

            #[test]
            fn test() {
                // TODO
            }
        }
    };
}