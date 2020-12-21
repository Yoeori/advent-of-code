use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

lazy_static! {
    static ref RE_FOOD: Regex = Regex::new(r"^((?:\w+ )*(?:\w+))(?: \(contains ((?:\w+, )*(?:\w+))\))?$").unwrap();
}

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    alergens: HashSet<&'a str>
}

pub fn main() {
    let file = include_str!("../puzzles/21.txt").split('\n');

    let foods: Vec<Food> = file.map(|l| RE_FOOD.captures(l).unwrap()).map(|cap| {
        Food {
            ingredients: cap.get(1).unwrap().as_str().split(' ').collect(),
            alergens: cap.get(2).unwrap().as_str().split(", ").collect()
        }
    }).collect();

    let mut all_alergens: Vec<&str> = foods.iter().map(|list| &list.alergens).flatten().unique().map(|s| *s).collect();
    all_alergens.sort();

    let mut possible_alergens: HashSet<&str> = HashSet::new();
    let mut alergens_to_ing = HashMap::new();
    for &alergen in &all_alergens {

        // Do intersection of each ingredient list which contains this alergen
        // Sadly Iterator.fold_first is not stable yet :(
        alergens_to_ing.insert(alergen, foods.iter().filter(|list| list.alergens.contains(alergen))
            .fold(None, |cur: Option<HashSet<&str>>, food| {
                if let Some(cur) = cur {
                    Some(cur.intersection(&food.ingredients).map(|x| *x).collect())
                } else {
                    Some(food.ingredients.clone())
                }
            }).unwrap());
            
        possible_alergens.extend(&alergens_to_ing[&alergen]);
    }

    println!("Solution to exercise 1: {}", foods.iter()
        .map(|food| food.ingredients.iter().filter(|ingr| !possible_alergens.contains(*ingr)).count())
        .sum::<usize>());

    let mut rem_set: HashSet<&str> = HashSet::new();
    while !alergens_to_ing.iter().all(|(_, s)| s.len() == 1) {
        for (_, map) in alergens_to_ing.iter_mut() {
            if map.len() == 1 {
                rem_set.extend(map.iter().map(|x| *x));
            } else {
                *map = map.difference(&rem_set).map(|x | *x).collect();
            }
        }
    }

    println!("Solution to exercise 2: {}", all_alergens.iter()
        .map(|&alergen| alergens_to_ing.get(alergen).unwrap().iter().next().unwrap().clone())
        .intersperse(",").collect::<String>());

}