use std::collections::HashMap;

enum Formula<'a> {
    Number(i64),
    Comp(&'a str, &'a str, fn(i64, i64) -> i64)
}

impl<'a> Formula<'a> {
    fn from_string(inp: &'a str) -> Self {
        if inp.len() == 11 {
            Formula::Comp(
                &inp[0..4], &inp[7..11], 
                match &inp[5..6] {
                    "+" => |a, b| a + b,
                    "-" => |a, b| a - b,
                    "*" => |a, b| a * b,
                    "/" => |a, b| a / b,
                    _ => panic!()
                }
            )
        } else {
            Formula::Number(inp.parse().unwrap())
        }
    }

    fn exec(&self, formulas: &HashMap<&str, Formula>) -> i64 {
        match self {
            Formula::Number(n) => *n,
            Formula::Comp(a, b, f) => f(formulas.get(a).unwrap().exec(formulas), formulas.get(b).unwrap().exec(formulas))
        }
    }

    fn set_num(&mut self, new: i64) {
        match self {
            Formula::Number(n) => *n = new,
            _ => panic!()
        }
    }
}

pub fn main() {
    let mut formulas: HashMap<&str, Formula> = include_str!("../puzzles/21.txt").lines().map(|line| {
        (&line[0..4], Formula::from_string(&line[6..]))
    }).collect();

    println!("Exercise 1: {}", formulas["root"].exec(&formulas));

    // Which monkeys
    let (left, right) = match formulas["root"] {
        Formula::Comp(a, b, _) => (a, b),
        _ => panic!()
    };

    // Check which one contains "humn"
    let l1 = formulas[left].exec(&formulas);
    formulas.get_mut("humn").unwrap().set_num(0);
    let l2 = formulas[left].exec(&formulas);

    let res = if l1 != l2 {
        let r = formulas[right].exec(&formulas);
        binary_search(&mut formulas, left, r)
    } else {
        let l = formulas[left].exec(&formulas);
        binary_search(&mut formulas, right, l)
    };

    // There can be multiple answers, take lowest.
    for x in (res-10)..(res) {
        formulas.get_mut("humn").unwrap().set_num(x);

        if formulas[left].exec(&formulas) == formulas[right].exec(&formulas) {
            println!("Exercise 2: {}", x);
            break;
        }
    }

}

fn binary_search(formulas: &mut HashMap<&str, Formula>, search: &str, value: i64) -> i64 {
    let mut min = 0;
    let mut max = 100_000_000_000_000;

    while min != max {
        let mid = (min + max) / 2;

        formulas.get_mut("humn").unwrap().set_num(mid);
        let res = formulas[search].exec(formulas);

        if res == value {
            return mid;
        } else if res > value {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    min
}