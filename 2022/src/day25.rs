pub fn main() {
    println!("Exercise: {}", int_to_double_minus(include_str!("../puzzles/25.txt").lines().map(|l| double_minus_to_int(l)).sum()));
}

fn double_minus_to_int(inp: &str) -> i64 {
    if inp == "" {
        return 0;
    }

    double_minus_to_int(&inp[0..(inp.len() - 1)]) * 5 + match &inp[(inp.len() - 1)..(inp.len())] {
        "2" => 2,
        "1" => 1,
        "0" => 0,
        "-" => -1,
        "=" => -2,
        _ => panic!()
    }
}

fn int_to_double_minus(inp: i64) -> String {
    if inp == 0 {
        return String::new();
    }

    let rem = inp.rem_euclid(5);

    if rem <= 2 {
        let mut out = int_to_double_minus(inp / 5);
        out.push(char::from_digit(rem as u32, 10).unwrap());
        out
    } else {
        let mut out = int_to_double_minus(inp / 5 + 1);
        out.push(if rem == 3 {'='} else {'-'});
        out
    }
}