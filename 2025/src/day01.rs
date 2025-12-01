use crate::puzzle::Puzzle;

pub(crate) struct Day01;
impl Puzzle for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let rotations: Vec<i32> = inp
            .lines()
            .map(|l| (if &l[0..1] == "L" { -1 } else { 1 }) * l[1..].parse::<i32>().unwrap())
            .collect();

        let mut n: i32 = 50;

        let mut count_one: u32 = 0;
        let mut count_two: u32 = 0;

        for &rot in &rotations {
            if rot < 0 && n == 0 {
                n = 100;
            }
            n += rot;
            
            count_two += (n / 100).unsigned_abs() + if n <= 0 { 1 } else { 0 };

            n = n.rem_euclid(100);
            
            if n == 0 {
                count_one += 1;
            }
        }

        (count_one, count_two)
    }
}
