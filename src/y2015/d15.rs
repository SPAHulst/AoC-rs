use crate::y2015::io::get_contents;
use std::cmp::max;
use std::iter::zip;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredients(s: &str) -> Vec<Ingredient> {
    let lines = s.lines();

    let mut ingredients = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();

        let capacity = parts.nth(2).unwrap().trim_end_matches(',').parse().unwrap();
        let durability = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let flavor = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let texture = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let calories = parts.nth(1).unwrap().parse().unwrap();

        let ingredient = Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        };

        ingredients.push(ingredient);
    }

    ingredients
}

fn compute_cookie_score(amounts: &[i32], ingredients: &[Ingredient]) -> i32 {
    let mut capacity_score = 0;
    let mut durability_score = 0;
    let mut flavor_score = 0;
    let mut texture_score = 0;

    for (amount, ingredient) in zip(amounts.iter(), ingredients.iter()) {
        capacity_score += amount * ingredient.capacity;
        durability_score += amount * ingredient.durability;
        flavor_score += amount * ingredient.flavor;
        texture_score += amount * ingredient.texture;
    }

    max(capacity_score, 0) * max(durability_score, 0) * max(flavor_score, 0) * max(texture_score, 0)
}

fn find_best_cookie_score(
    amounts: &mut Vec<i32>,
    ingredients: &[Ingredient],
    max_cookie_score: &mut i32,
    ingredient: usize,
    remaining: i32,
) {
    if ingredient == amounts.len() - 1 {
        amounts[ingredient] = remaining;
        let cookie_score = compute_cookie_score(amounts, ingredients);
        *max_cookie_score = (*max_cookie_score).max(cookie_score);
        return;
    }

    for amount in 0..=remaining {
        amounts[ingredient] = amount;
        find_best_cookie_score(
            amounts,
            ingredients,
            max_cookie_score,
            ingredient + 1,
            remaining - amount,
        );
    }
}

fn compute_weighted_cookie_score(amounts: &[i32], ingredients: &[Ingredient]) -> i32 {
    let mut capacity_score = 0;
    let mut durability_score = 0;
    let mut flavor_score = 0;
    let mut texture_score = 0;
    let mut calories = 0;

    for (amount, ingredient) in zip(amounts.iter(), ingredients.iter()) {
        capacity_score += amount * ingredient.capacity;
        durability_score += amount * ingredient.durability;
        flavor_score += amount * ingredient.flavor;
        texture_score += amount * ingredient.texture;
        calories += amount * ingredient.calories;
    }

    if calories != 500 {
        return 0;
    }

    max(capacity_score, 0) * max(durability_score, 0) * max(flavor_score, 0) * max(texture_score, 0)
}

fn find_best_weighted_cookie_score(
    amounts: &mut Vec<i32>,
    ingredients: &[Ingredient],
    max_cookie_score: &mut i32,
    ingredient: usize,
    remaining: i32,
) {
    if ingredient == amounts.len() - 1 {
        amounts[ingredient] = remaining;
        let cookie_score = compute_weighted_cookie_score(amounts, ingredients);
        *max_cookie_score = (*max_cookie_score).max(cookie_score);
        return;
    }

    for amount in 0..=remaining {
        amounts[ingredient] = amount;
        find_best_weighted_cookie_score(
            amounts,
            ingredients,
            max_cookie_score,
            ingredient + 1,
            remaining - amount,
        );
    }
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d15.txt");
    let ingredients = parse_ingredients(&contents);

    let mut amounts = vec![0; ingredients.len()];
    let mut max_cookie_score = 0;

    find_best_cookie_score(&mut amounts, &ingredients, &mut max_cookie_score, 0, 100);
    println!("Best cookie score: {max_cookie_score}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d15.txt");
    let ingredients = parse_ingredients(&contents);

    let mut amounts = vec![0; ingredients.len()];
    let mut max_cookie_score = 0;

    find_best_weighted_cookie_score(&mut amounts, &ingredients, &mut max_cookie_score, 0, 100);
    println!("Best cookie score: {max_cookie_score}");
}
