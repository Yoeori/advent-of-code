use std::fs;
use std::collections::HashMap;

use std::ops::Mul;
use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ingredient {
    amount: u64,
    ingredient: String
}

impl From<&str> for Ingredient {
    fn from(inp: &str) -> Self {
        let inp: Vec<&str> = inp.split(" ").collect();

        Ingredient {
            amount: inp[0].parse().unwrap(),
            ingredient: String::from(inp[1])
        }
    }
}

impl Mul<u64> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Ingredient {
            amount: rhs * self.amount,
            ingredient: self.ingredient
        }
    }
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
    result: Ingredient
}

impl From<&str> for Recipe {
    fn from(inp: &str) -> Self {
        let inp: Vec<&str> = inp.split(" => ").collect();

        Recipe {
            ingredients: inp[0].split(", ").map(|ing| Ingredient::from(ing)).collect(),
            result: Ingredient::from(inp[1])
        }
    }
}

impl Mul<u64> for &Recipe {
    type Output = Recipe;
    
    fn mul(self, rhs: u64) -> Self::Output {
        Recipe {
            ingredients: self.ingredients.clone().into_iter().map(|ing| ing * rhs).collect(),
            result: self.result.clone() * rhs
        }
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/14.txt").unwrap();
    let recipes: Vec<Recipe> = file_contents.split("\n").map(|rec| Recipe::from(rec)).collect();

    let recipes: HashMap<String, Recipe> = recipes.into_iter().map(|rec| (rec.result.ingredient.clone(), rec)).collect();

    let res1 = calculate_fuel(&recipes, 1);
    println!("Answer to exercise 1: {}", res1);
    println!("Answer to exercise 2: {}", binary_search_fuel(&recipes, 0, (1000000000000 / res1) * 3));
}

fn calculate_fuel(recipes: &HashMap<String, Recipe>, amount_of_fuel: u64) -> u64 {
    let mut needed: HashMap<String, u64> = HashMap::new();
    let mut left_over: HashMap<String, u64> = HashMap::new();
    needed.insert(String::from("FUEL"), amount_of_fuel);

    loop {
        let ing = needed.keys().find(|&ing| ing != &"ORE");

        if let Some(ingredient) = ing {
            let ingredient = ingredient.clone();
            
            let (ingredient, amount) = needed.remove_entry(&ingredient).unwrap();
            let recipe = recipes.get(&ingredient).unwrap();

            // Check how many times we need the recipe
            let recipe = recipe * (if amount % recipe.result.amount == 0 {
                amount / recipe.result.amount
            } else {
                (amount / recipe.result.amount) + 1
            });

            for ing in recipe.ingredients {
                match needed.get_mut(&ing.ingredient[..]) {
                    Some(amount) => { *amount += ing.amount; },
                    None => { needed.insert(ing.ingredient.clone(), ing.amount); }
                };
            }

            // Add leftover to leftover
            if recipe.result.amount - amount != 0 {
                left_over.insert(ingredient, recipe.result.amount - amount);
            }

            // Remove leftovers
            for (ingredient, amount) in left_over.iter_mut() {
                if needed.contains_key(ingredient) {
                    let subtracting = cmp::min(needed[ingredient], *amount);
                    *amount -= subtracting;
                    needed.insert(ingredient.clone(), needed[ingredient] - subtracting);
                }
            }

            left_over = left_over.into_iter().filter(|(_, amount)| *amount != 0).collect();
        } else {
            break;
        }
    }

    return needed[&String::from("ORE")];
}

fn binary_search_fuel(recipes: &HashMap<String, Recipe>, min: u64, max: u64) -> u64 {
    if min == max {
        return min;
    } else {
        let halfway = (min + max) / 2;

        // let firsthalf = calculate_fuel(&recipes, halfway);
        let seconhalf = calculate_fuel(&recipes, halfway + 1);

        if seconhalf > 1000000000000  {
            return binary_search_fuel(recipes, min, halfway);
        } else {
            return binary_search_fuel(recipes, halfway + 1, max);
        }
    }
}