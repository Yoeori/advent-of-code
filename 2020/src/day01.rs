use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::SystemTime;

pub fn main() {
    let file = fs::read_to_string("puzzles/1.txt").unwrap();

    // n^2 and n^3 way
    {
        let start = SystemTime::now();
        let numbers: Vec<usize> = file.split("\n").map(|x| x.parse().unwrap()).collect();

        let mut result = 0;
        for n1 in &numbers {
            for n2 in &numbers {
                if n1 + n2 == 2020 {
                    result = n1 * n2;
                }
            }
        }
        let end = start.elapsed().unwrap();
        println!("The answer for exercise 1 is: {}, which took {:?} (Vector with for-loops)", result, end);
    }

    {
        let start = SystemTime::now();
        let numbers: Vec<usize> = file.split("\n").map(|x| x.parse().unwrap()).collect();

        let mut result = 0;
        for n1 in &numbers {
            for n2 in &numbers {
                for n3 in &numbers {
                    if n1 + n2 + n3 == 2020 {
                        result = n1 * n2 * n3;
                    }
                }
            }
        }
        let end = start.elapsed().unwrap();
        println!("The answer for exercise 2 is: {}, which took {:?} (Vector with for-loops)", result, end);
    }

    // Answer with a HashSet
    {
        let start = SystemTime::now();
        let number_set: HashSet<usize> = HashSet::from_iter(file.split("\n").map(|x| x.parse().unwrap()));
        let mut result = 0;
        for n1 in &number_set {
            if number_set.contains(&(2020 - n1)) {
                result = n1 * (2020 - n1);
            }
        }
        let end = start.elapsed().unwrap();
        println!("The answer for exercise 1 is: {}, which took {:?} (HashSet)", result, end);
    }

    {
        let start = SystemTime::now();
        let number_set: HashSet<usize> = HashSet::from_iter(file.split("\n").map(|x| x.parse().unwrap()));
        let mut result = 0;
        for n1 in &number_set {
            for n2 in &number_set {
                if n1 + n2 <= 2020 && number_set.contains(&(2020 - n1 - n2)) {
                    result = n1 * (2020 - n1 - n2) * n2;
                }
            }
        }
        let end = start.elapsed().unwrap();
        println!("The answer for exercise 2 is: {}, which took {:?} (HashSet)", result, end);
    }

    


}