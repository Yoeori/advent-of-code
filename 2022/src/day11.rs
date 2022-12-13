#[derive(Debug, Default, Clone)]
enum Operation {
    Times(usize), Plus(usize), #[default] Square
}

impl Operation {
    fn parse(inp: &str) -> Self {
        if inp == "new = old * old" {
            Operation::Square
        } else if inp.starts_with("new = old + ") {
            let (_, right) = inp.rsplit_once(' ').unwrap();
            Operation::Plus(right.parse().unwrap())
        } else if inp.starts_with("new = old * ") {
            let (_, right) = inp.rsplit_once(' ').unwrap();
            Operation::Times(right.parse().unwrap())
        } else {
            panic!();
        }
    }

    fn perform(&self, inp: usize) -> usize {
        match self {
            Operation::Times(n) => inp * n,
            Operation::Plus(n) => inp + n,
            Operation::Square => inp * inp
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_test: (usize, usize, usize),
    inspections: u64
}

impl Monkey {
    fn parse(inp: &str) -> Self {
        let mut lines = inp.lines();
        lines.next().unwrap(); // id

        Monkey {
            items: lines.next().unwrap().split_once(": ").unwrap().1.split(", ").map(|x| x.parse().unwrap()).collect(),
            operation: Operation::parse(&lines.next().unwrap()[13..]),
            divisible_test: (
                lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap(),
                lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap(),
                lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap()
            ),
            inspections: 0
        }
    }
}

const EMPTY_MONKEY: Monkey = Monkey {
    items: Vec::new(), operation: Operation::Square, divisible_test: (0, 0, 0), inspections: 0
};

pub fn main() {
    let mut monkeys: Vec<Monkey> = include_str!("../puzzles/11.txt").split("\n\n").map(|m| Monkey::parse(m)).collect();
    let monkeys_clone = monkeys.clone();

    for _ in 0..20 {
        for id in 0..monkeys.len() {
            let mut monkey = std::mem::replace(&mut monkeys[id], EMPTY_MONKEY);
            for item in monkey.items.drain(..) {
                monkey.inspections += 1;
                let v = monkey.operation.perform(item) / 3;
                if v % monkey.divisible_test.0 == 0 {
                    monkeys[monkey.divisible_test.1].items.push(v);
                } else {
                    monkeys[monkey.divisible_test.2].items.push(v);
                }
            }
            monkeys[id] = monkey;
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    inspections.reverse();
    println!("Exercise 1: {}", inspections[0] * inspections[1]);

    monkeys = monkeys_clone;
    let divisible_test = monkeys.iter().map(|x| x.divisible_test.0).fold(1, |acc, x| acc * x);

    for _ in 0..10000 {
        for id in 0..monkeys.len() {
            let mut monkey = std::mem::replace(&mut monkeys[id], EMPTY_MONKEY);
            for item in monkey.items.drain(..) {
                monkey.inspections += 1;
                let v = monkey.operation.perform(item) % divisible_test;
                if v % monkey.divisible_test.0 == 0 {
                    monkeys[monkey.divisible_test.1].items.push(v);
                } else {
                    monkeys[monkey.divisible_test.2].items.push(v);
                }
            }
            monkeys[id] = monkey;
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    inspections.reverse();
    println!("Exercise 2: {}", inspections[0] * inspections[1]);
}