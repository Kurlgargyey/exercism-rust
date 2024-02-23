use combinations::Combinations;
use permutohedron::heap_recursive;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let chars = unique_letters(input);
    let value_combos = generate_permutations(&chars).unwrap();

    let mut possible_combinations: Vec<HashMap<_, _>> = Vec::new();

    for combo in value_combos {
        let mut combo_map = HashMap::new();
        let mut chars_iter = chars.iter();
        for value in combo {
            combo_map.insert(*chars_iter.next().unwrap(), value as u8);
        }
        possible_combinations.push(combo_map);
    }

    let mut components = input.split(" == ");
    let addends: Vec<&str> = components.next().unwrap().split(" + ").collect();
    let end_sum = components.next().unwrap();

    'combo: for combination in possible_combinations {
        let mut int_addends = vec![];
        for addend in &addends {
            if let Some(parsed_addend) = combine_map_with_str(*addend, &combination) {
                int_addends.push(parsed_addend);
            } else {
                continue 'combo;
            };
        }
        let combination_sum: i64 = int_addends
            .iter()
            .map(|i| *i as i64)
            .sum();
        if let Some(parsed_sum) = combine_map_with_str(&end_sum, &combination) {
            if combination_sum == (parsed_sum as i64) {
                return Some(combination);
            }
        };
    }

    None
}

fn combine_map_with_str(str: &str, map: &HashMap<char, u8>) -> Option<i64> {
    let mut addend_digits = String::from(str);
    for (char, value) in map {
        addend_digits = addend_digits.replace(*char, &value.to_string());
    }
    if addend_digits.starts_with('0') {
        return None;
    }
    Some(addend_digits.parse::<i64>().unwrap())
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

fn generate_permutations(chars: &Vec<char>) -> Option<Vec<Vec<i32>>> {
    let char_count = chars.len();

    match char_count {
        u if u > 10 => { None }
        u if u == 10 => {
            Some(
                vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]]
                    .into_iter()
                    .flat_map(|mut combination| {
                        let mut permutations = Vec::new();
                        heap_recursive(&mut combination, |permutation| {
                            permutations.push(permutation.to_vec());
                        });
                        permutations
                    })
                    .collect()
            )
        }
        _ => {
            Some(
                Combinations::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], char_count)
                    .flat_map(|mut combination| {
                        let mut permutations = Vec::new();
                        heap_recursive(&mut combination, |permutation| {
                            permutations.push(permutation.to_vec());
                        });
                        permutations
                    })
                    .collect()
            )
        }
    }
}
