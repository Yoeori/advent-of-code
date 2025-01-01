use crate::puzzle::Puzzle;

pub(crate) struct Day25;
impl Puzzle for Day25 {
    type Part1 = i32;
    type Part2 = ();

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let mut keys: Vec<[u8; 5]> = vec![];
        let mut locks: Vec<[u8; 5]> = vec![];

        for (out, t) in inp.split("\n\n").map(parse_lock_key) {
            if t == Type::Lock {
                locks.push(out);
            } else {
                keys.push(out);
            }
        }

        let mut answer = 0;

        for key in keys.iter() {
            for lock in locks.iter() {
                // It fits if all elements in key >= lock
                if key.iter().zip(lock.iter()).all(|(x, y)| x >= y) {
                    answer += 1;
                }
            }
        }

        (answer, ())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Lock,
    Key,
}

fn parse_lock_key(inp: &str) -> ([u8; 5], Type) {
    let c = inp.chars().next().unwrap(); // either . or #
    let t = if c == '.' { Type::Key } else { Type::Lock };

    let mut output = [u8::MAX; 5];

    for (y, line) in inp.lines().enumerate() {
        for (x, lc) in line.chars().enumerate() {
            if lc != c {
                output[x] = output[x].min(y as u8);
            }
        }
    }

    (output, t)
}
