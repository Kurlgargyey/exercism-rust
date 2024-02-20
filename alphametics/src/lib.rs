use combinations::Combinations;
use permutohedron::heap_recursive;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut chars = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    let char_count = chars.len();
    if char_count > 10 {
        return None;
    }
    let value_combos = Combinations::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], char_count).flat_map(
        |mut combination| {
            let mut permutations = Vec::new();
            heap_recursive(&mut combination, |permutation| {
                permutations.push(permutation.to_vec());
            });
            permutations
        }
    );

    let mut possible_combinations: Vec<HashMap<_, _>> = Vec::new();

    for combo in value_combos {
        //println!("mapping combination {combo:?}");
        let mut combo_map = HashMap::new();
        let mut chars_iter = chars.iter();
        for value in combo {
            combo_map.insert(*chars_iter.next().unwrap(), value as u8);
        }
        possible_combinations.push(combo_map);
    }

    let no_whitespace = input.split_ascii_whitespace().collect::<String>();
    let addends: Vec<_> = no_whitespace.split("==").nth(0)?.split("+").collect();
    let sum = no_whitespace.split("==").last().unwrap().to_string();

    'combo: for combination in possible_combinations {
        let mut int_addends = vec![];
        for addend in &addends {
            if let Some(parsed_addend) = combine_map_with_str(*addend, &combination) {
                int_addends.push(parsed_addend);
            } else {
                continue 'combo;
            };
        }
        let combination_sum: i32 = int_addends.iter().sum();
        if let Some(parsed_sum) = combine_map_with_str(&sum.as_str(), &combination) {
            if combination_sum == parsed_sum {
                return Some(combination);
            }
        };
    }

    None
}

fn combine_map_with_str(str: &str, map: &HashMap<char, u8>) -> Option<i32> {
    let mut addend_digits = String::from(str);
    for (char, value) in map {
        addend_digits = addend_digits.replace(*char, &value.to_string());
    }
    if addend_digits.starts_with('0') {
        return None;
    }
    Some(addend_digits.parse::<i32>().unwrap())
}