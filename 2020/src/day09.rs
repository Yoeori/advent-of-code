use std::fs;

pub fn main() {
    let file = fs::read_to_string("puzzles/9.txt").unwrap();
    
    let numbers: Vec<usize> = file.split("\n").map(|x| x.parse().unwrap()).collect();

    let mut prev25 = 0;
    let mut invalid = 0;

    for n in numbers.iter().skip(25) {
        let mut r = false;

        'outerloop: for x in prev25..(prev25+25) {
            for y in prev25..(prev25+25) {
                if numbers[x] + numbers[y] == *n {
                    r = true;
                    break 'outerloop;
                }
            }
        }

        prev25 += 1;

        if !r {
            println!("Solution to exercise 1: {}", n);
            invalid = *n;
            prev25 -= 1;
            break;
        }
    }

    // Can be improved by 'scrolling' through the list, and only subtracting item c+0 + adding item c+max_item
    let max_item = prev25;
    for size in 2..max_item {
        for i in 0..(max_item-size) {
            if numbers[i..(i+size)].iter().sum::<usize>() == invalid {
                println!("Solution to exercise 2: {}", numbers[i..(i+size)].iter().max().unwrap() + numbers[i..(i+size)].iter().min().unwrap());
            }
        }
    }

}