#[derive(Debug)]
enum List {
    Vec(Vec<List>), Value(u32)
}

#[derive(Debug, PartialEq, Eq)]
enum CompResult {
    Unknown, InOrder, OutOfOrder
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

    fn compare(&self, other: &Self) -> CompResult {
        match (self, other) {
            (List::Value(a), List::Value(b)) => {
                if a < b {
                    CompResult::InOrder
                } else if a > b {
                    CompResult::OutOfOrder
                } else {
                    CompResult::Unknown
                }
            }
            (List::Vec(a), List::Vec(b)) => {
                let mut prev_res = CompResult::Unknown;
                let mut i = 0;

                while prev_res == CompResult::Unknown && (i < a.len() || i < b.len()) {
                    if i >= a.len() {
                        return CompResult::InOrder;
                    }

                    if i >= b.len() {
                        return CompResult::OutOfOrder;
                    }

                    prev_res = a[i].compare(&b[i]);
                    i += 1;
                }

                prev_res
            }
            (List::Value(a), b) => List::Vec(vec![List::Value(*a)]).compare(b),
            (a, List::Value(b)) => a.compare(&List::Vec(vec![List::Value(*b)]))
        }
    }
}

pub fn main() {
    let mut acc = 0;
    let mut lists = vec![];

    for (i, (l1, l2)) in include_str!("../puzzles/13.txt").split("\n\n").map(|x| x.split_once('\n').unwrap()).enumerate() {
        let l1 = List::parse(l1);
        let l2 = List::parse(l2);

        if l1.compare(&l2) == CompResult::InOrder {
            acc += i + 1;
        }

        lists.push(l1);
        lists.push(l2);
    }

    println!("Exercise 1: {}", acc);

    let c1 = List::parse("[[2]]");
    let c2 = List::parse("[[6]]");

    let c1_count = lists.iter().map(|x| x.compare(&c1)).filter(|x| x == &CompResult::InOrder).count() + 1;
    let c2_count = lists.iter().map(|x| x.compare(&c2)).filter(|x| x == &CompResult::InOrder).count() + 2;
    
    println!("Exercise 2: {}", c1_count * c2_count);
}