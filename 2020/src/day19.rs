use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

#[derive(Debug)]
enum Rule {
    Many(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Letter(char),
}

lazy_static! {
    static ref RE_MANY: Regex = Regex::new(r"^(\d+): ((?:\d+ )*(?:\d+))$").unwrap();
    static ref RE_OR: Regex = Regex::new(r"^(\d+): ((?:\d+ )*(?:\d+)) \| ((?:\d+ )*(?:\d+))$").unwrap();
    static ref RE_LETTER: Regex = Regex::new(r#"^(\d+): "(\w)"$"#).unwrap();
}

pub fn main() {
    let mut file = include_str!("../puzzles/19.txt").split("\n\n");

    let rule_strings = file.next().unwrap();
    let messages = file.next().unwrap().split('\n');

    let rules: HashMap<usize, Rule> = rule_strings.split('\n').map(|rule_string| {
        if let Some(cap) = RE_MANY.captures(rule_string) {
            (cap[1].parse().unwrap(), Rule::Many(cap[2].split(' ').map(|x| x.parse().unwrap()).collect()))
        } else if let Some(cap) = RE_OR.captures(rule_string) {
            (cap[1].parse().unwrap(), Rule::Or(cap[2].split(' ').map(|x| x.parse().unwrap()).collect(), cap[3].split(' ').map(|x| x.parse().unwrap()).collect()))
        } else if let Some(cap) = RE_LETTER.captures(rule_string) {
            (cap[1].parse().unwrap(), Rule::Letter(cap[2].parse().unwrap()))
        } else {
            panic!("Invalid rule: {}", rule_string);
        }
    }).collect();

    let re1 = Regex::new(&format!("^{}$", generate_regex(&rules, &0, false))).unwrap();
    println!("Solution to exercise 1: {}", messages.clone().filter(|m| re1.is_match(m)).count());

    let re2 = Regex::new(&format!("^{}$", generate_regex(&rules, &0, true))).unwrap();
    println!("Solution to exercise 2: {}", messages.filter(|m| re2.is_match(m)).count());
}

fn generate_regex(rules: &HashMap<usize, Rule>, cur: &usize, special: bool) -> String {
    if special && cur == &8 {
        format!(r"({})+", generate_regex(rules, &42, true))
    } else if special && cur == &11 {
        // Haha Rust's Regex crate says no:
        // format!(r"(?'rec'{}(\g'rec')?{})", generate_regex(rules, &42, true), generate_regex(rules, &31, true))

        // Dirty hack which only allows for a max recursion depth of 10 due to Rust's Regex crate limits :(
        let r42 = generate_regex(rules, &42, true);
        let r31 = generate_regex(rules, &31, true);
        format!("({})", (1..10).map(|x| format!("({}){{{}}}({}){{{}}}", r42, x, r31, x)).intersperse("|".to_string()).collect::<String>())
    } else {
        match rules.get(&cur) {
            Some(Rule::Many(rs)) => rs.iter().map(|r| generate_regex(rules, r, special)).collect::<String>(),
            Some(Rule::Or(rs1, rs2)) => format!("({}|{})", 
                rs1.iter().map(|r| generate_regex(rules, r, special)).collect::<String>(), 
                rs2.iter().map(|r| generate_regex(rules, r, special)).collect::<String>()),
            Some(Rule::Letter(c)) => c.to_string(),
            None => panic!("Rule not found: {}", cur)
        }
    }    
}