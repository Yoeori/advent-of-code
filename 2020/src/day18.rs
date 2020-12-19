pub fn main() {
    let file = include_str!("../puzzles/18.txt")
        .split('\n')
        .map(|l| l.split(' ').map(|c| c.chars()).flatten().map(|c| c.to_string()));

    println!("Solution to exercise 1: {}", file.clone().map(|mut l| eval(&mut l)).sum::<usize>());
    println!("Solution to exercise 2: {}", file.clone().map(|mut l| eval_addition(&mut l)).sum::<usize>());
}

fn eval(seq: &mut dyn Iterator<Item=String>) -> usize {
    let mut total = num(seq, &eval).unwrap();

    while let Some(operator) = seq.next() {
        match &operator[..] {
            "*" => total *= num(seq, &eval).unwrap(),
            "+" => total += num(seq, &eval).unwrap(),
            ")" => break,
            _   => panic!("Invalid operator: {}", operator)
        }
    }

    total
}

/// First evaluates all '+' operators and than evaluates the resulting vec
fn eval_addition(seq: &mut dyn Iterator<Item=String>) -> usize {
    let mut res: Vec<String> = vec![];

    let mut prev = num(seq, &eval_addition).unwrap();
    while let Some(operator) = seq.next() {
        match &operator[..] {
            ")" => break,
            "+" => prev += num(seq, &eval_addition).unwrap(),
            op => {
                res.push(prev.to_string());
                res.push(op.to_string());
                prev = num(seq, &eval_addition).unwrap()
            }
        }
    }

    res.push(prev.to_string());
    eval(&mut res.into_iter())
}

fn num(seq: &mut dyn Iterator<Item=String>, evaluater: &impl Fn(&mut dyn Iterator<Item=String>) -> usize) -> Option<usize> {
    match seq.next().as_deref() {
        Some("(") => Some(evaluater(seq)),
        Some(n) => Some(n.parse().unwrap()),
        None => None
    }
}