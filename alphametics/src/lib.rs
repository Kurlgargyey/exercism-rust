use itertools::Itertools;
use std::{ collections::HashMap, error::Error };

#[derive(Debug)]
enum Errors {
    LeadingZero,
    SumMismatch,
    EmptyMap,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let chars = unique_letters(input);
    if chars.len() > 10 {
        return None;
    }
    let mut components = input.split(" == ");
    let addends: Vec<&str> = components.next().unwrap().split(" + ").collect();
    let target = components.next().unwrap();

    let permutations = (0..=9).permutations(chars.len());
    let mut combinations = permutations.map(|permutation| {
        let mut combo_map = HashMap::new();
        let mut chars_iter = chars.iter();
        for value in permutation {
            combo_map.insert(*chars_iter.next().unwrap(), value as u8);
        }
        combo_map
    });

    loop {
        let result = check_map(combinations.next()?, &addends, target);
        if let Ok(combo) = result {
            return Some(combo);
        }
    }
}

fn check_map<'a>(
    combination: HashMap<char, u8>,
    addends: &Vec<&str>,
    target: &str
) -> Result<HashMap<char, u8>, Errors> {
    let parsed_target = parse_string(&combination, target)?;
    let combination_sum: i64 = parse_addends(&combination, &addends)?;

    if combination_sum == (parsed_target as i64) {
        return Ok(combination);
    }
    Err(Errors::SumMismatch)
}

fn parse_addends(combination: &HashMap<char, u8>, addends: &Vec<&str>) -> Result<i64, Errors> {
    let mut int_addends = vec![];
    for addend in addends {
        int_addends.push(parse_string(&combination, addend)?);
    }
    Ok(int_addends.into_iter().sum())
}

fn parse_string(combination: &HashMap<char, u8>, str: &str) -> Result<i64, Errors> {
    let mut digits = String::from(str);
    for (char, value) in combination {
        digits = digits.replace(*char, &value.to_string());
    }
    if digits.starts_with('0') {
        return Err(Errors::LeadingZero);
    }
    Ok(digits.parse::<i64>().unwrap())
}

fn unique_letters(input: &str) -> Vec<char> {
    let mut chars = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|char| char.to_ascii_uppercase())
        .collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars
}

fn possible_combinations(chars: &Vec<char>) -> () {
    todo!("make a function out of this!")
}
