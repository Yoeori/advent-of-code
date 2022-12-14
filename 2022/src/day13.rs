use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum List {
    Vec(Vec<List>), Value(u32)
}

impl List {
    fn push(&mut self, item: List) {
        match self {
            List::Vec(v) => v.push(item),
            _ => panic!()
        }
    }

    fn parse(inp: &str) -> Self {
        List::parse_range(&inp[1..]).0
    }

    fn parse_range(inp: &str) -> (Self, usize) {
        let mut output = List::Vec(Vec::new());
        let mut idx = 0;
        
        while idx <= inp.len() - 1 {
            match &inp[idx..idx+1] {
                "[" => {
                    let res = List::parse_range(&inp[idx+1..]);
                    output.push(res.0);
                    idx = res.1 + idx + 1;
                },
                "]" => {
                    return (output, idx + 1)
                }
                "," => {
                    idx += 1;
                }
                _ => {
                    let til = inp[idx+1..].find(|c: char| !c.is_numeric()).unwrap() + idx + 1;
                    output.push(List::Value(inp[idx..til].parse::<u32>().unwrap()));
                    idx = til;
                }
            }
        }

        (output, idx + 1)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Value(a), List::Value(b)) => a.cmp(b),
            
            (List::Value(a), b) => List::Vec(vec![List::Value(*a)]).cmp(b),
            (a, List::Value(b)) => a.cmp(&List::Vec(vec![List::Value(*b)])),

            (List::Vec(a), List::Vec(b)) => {
                let mut prev_res = Ordering::Equal;
                let mut i = 0;

                while prev_res == Ordering::Equal && (i < a.len() || i < b.len()) {
                    if i >= a.len() {
                        return Ordering::Less;
                    }

                    if i >= b.len() {
                        return Ordering::Greater;
                    }

                    prev_res = a[i].cmp(&b[i]);
                    i += 1;
                }

                prev_res
            }
        }
    }
}

pub fn main() {
    let mut acc = 0;
    let mut lists = vec![];

    for (i, (l1, l2)) in include_str!("../puzzles/13.txt").split("\n\n").map(|x| x.split_once('\n').unwrap()).enumerate() {
        let l1 = List::parse(l1);
        let l2 = List::parse(l2);

        if l1.cmp(&l2) == Ordering::Less {
            acc += i + 1;
        }

        lists.push(l1);
        lists.push(l2);
    }

    println!("Exercise 1: {}", acc);

    let c1 = List::parse("[[2]]");
    let c2 = List::parse("[[6]]");

    lists.push(c1.clone());
    lists.push(c2.clone());
    lists.sort();

    println!("Exercise 2: {}", (lists.iter().position(|x| x == &c1).unwrap() + 1) * (lists.iter().position(|x| x == &c2).unwrap() + 1));
}