use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn main() {
    let file = fs::read_to_string("puzzles/7.txt").unwrap();
    let re = Regex::new(r"((?:\w|\s)+) bags contain (?:(\d) ((?:\w|\s)+) bags?, )?(?:(\d) ((?:\w|\s)+) bags?, )?(?:(\d) ((?:\w|\s)+) bags?, )?(?:(\d) ((?:\w|\s)+) bags?|no other bags).\s?").unwrap();

    let bags: HashMap<String, Vec<(usize, String)>> = re.captures_iter(&file).map(|cap| {
        let mut bag_contents = Vec::new();

        for x in (2..cap.len()).step_by(2) {
            if cap.get(x).is_some() {
                bag_contents.push((cap[x].parse().unwrap(), cap[x+1].to_string()));
            }
        };

        (cap[1].to_string(), bag_contents)
    }).collect();

    let mut shiny_gold_set: HashSet<String> = HashSet::new();
    shiny_gold_set.insert("shiny gold".to_string());
    let mut old_count = 0;

    while old_count != shiny_gold_set.len() {
        old_count = shiny_gold_set.len();

        for (bag, bag_contents) in &bags {
            if shiny_gold_set.contains(bag) {
                continue;
            }

            for (_, bag_content) in bag_contents {
                if shiny_gold_set.contains(bag_content) {
                    shiny_gold_set.insert(bag.clone());
                    break;
                }
            }
        }
    }

    println!("Answer to exercise 1: {}", shiny_gold_set.len()-1);
    println!("Answer to exercise 2: {}", count(&bags, "shiny gold")-1);
}


fn count(bags: &HashMap<String, Vec<(usize, String)>>, bag: &str) -> usize {
    bags.get(bag).unwrap().iter().fold(1, |acc, (bag_count, bag)| acc + bag_count * count(bags, bag))
}