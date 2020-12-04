use std::fs;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use std::error::Error;


pub fn main() {
    let file = fs::read_to_string("puzzles/4.txt").unwrap();
    let passports: Vec<HashMap<String, String>> = file.split("\n\n").map(|x|
        x.replace("\n", " ").split(" ").map(|y| {
            let mut kv = y.split(":");
            (kv.next().unwrap().to_string(), kv.next().unwrap().to_string())
        }).collect()
    ).collect();

    println!("Solution to exercise 1: {}", passports.iter().filter(|pass| check(&pass)).count());
    println!("Solution to exercise 2: {}", passports.iter().filter(|pass| {
        let result = check_strict(&pass);
        result.is_ok() && result.unwrap()
    }).count());
}

lazy_static! {
    static ref COLOR: Regex = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn check(to_check: &HashMap<String, String>) -> bool {
    return to_check.contains_key("byr")
        && to_check.contains_key("iyr")
        && to_check.contains_key("eyr")
        && to_check.contains_key("hgt")
        && to_check.contains_key("hcl")
        && to_check.contains_key("ecl")
        && to_check.contains_key("pid");
}

fn check_strict(to_check: &HashMap<String, String>) -> Result<bool, Box<dyn Error + 'static>> {

    if !check(to_check) {
        return Ok(false);
    }

    let byr: usize = to_check.get("byr").unwrap().parse()?;
    let iyr: usize = to_check.get("iyr").unwrap().parse()?;
    let eyr: usize = to_check.get("eyr").unwrap().parse()?;
    let ecl: &str = to_check.get("ecl").unwrap();

    if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030 {
        return Ok(false);
    }

    if to_check.get("hgt").unwrap().ends_with("cm") {
        let height: usize = to_check.get("hgt").unwrap().replace("cm", "").parse()?;
        if height < 150 ||  height > 193 {
            return Ok(false);
        }
    } else {
        let height: usize = to_check.get("hgt").unwrap().replace("in", "").parse()?;
        if height < 59 || height > 76 {
            return Ok(false);
        }
    }

    if !(ecl == "amb" || ecl == "blu" || ecl == "brn" || ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth") {
        return Ok(false);
    }

    if !COLOR.is_match(to_check.get("hcl").unwrap()) || !PID.is_match(to_check.get("pid").unwrap()) {
        return Ok(false);
    }

    Ok(true)
}
