pub fn main() {
    let commands = include_str!("../puzzles/10.txt").split('\n').map(|x| x.split(' '));
    let mut xs: Vec<i32> = vec![1];

    for mut command in commands {
        match command.next().unwrap() {
            "addx" => {
                xs.push(*xs.last().unwrap());
                xs.push(*xs.last().unwrap() + command.next().unwrap().parse::<i32>().unwrap());
            },
            "noop" => {
                xs.push(*xs.last().unwrap());
            },
            _ => panic!()
        }
    }

    println!("Exercise 1: {}", (20..xs.len()).step_by(40).map(|i| xs[i - 1] * i as i32).sum::<i32>());

    let mut crt = vec![vec![false; 40]; 6];
    for y in 0..crt.len() {
        for x in 0..crt[y].len() {
            if (xs[y * crt[y].len() + x] - x as i32).abs() <= 1 {
                crt[y][x] = true;
            }
        }
    }

    println!("Exercise 2:");
    for row in crt {
        for c in row {
            print!("{}", if c { '#' } else { ' ' });
        }
        println!("");
    }

}