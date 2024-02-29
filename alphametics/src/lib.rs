use std::collections::{ HashMap, HashSet };
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let solver = Solver::new(input);
    if solver.char_count > 10 {
        return None;
    }

    for permutation in (0..=9).permutations(solver.char_count) {
        let checksum = solver.generate_checksum(&permutation);
        if checksum == 0 {
            let possible_solution = HashMap::from_iter(
                solver.weights.clone().into_keys().zip(permutation.into_iter())
            );
            if
                solver.leading_chars
                    .iter()
                    .any(|leading| *possible_solution.get(&leading).unwrap() == 0)
            {
                continue;
            }
            return Some(possible_solution);
        }
    }

    None
}

struct Solver {
    leading_chars: HashSet<char>,
    char_count: usize,
    weights: HashMap<char, i64>,
}

impl Solver {
    pub fn new(input: &str) -> Self {
        let words = input
            .split(|c| (c == '+' || c == '='))
            .filter(|word| !word.is_empty())
            .map(|word| word.trim());
        let leading_chars = words.clone().fold(HashSet::new(), |mut acc, word| {
            acc.insert(word.chars().nth(0).unwrap());
            acc
        });
        let char_count = words
            .clone()
            .fold(HashSet::new(), |acc, word|
                word.chars().fold(acc, |mut acc, char| {
                    acc.insert(char);
                    acc
                })
            )
            .len();
        let weights = Self::weigh_chars(&words.collect());
        Solver { leading_chars, char_count, weights }
    }

    pub fn generate_checksum(&self, values: &Vec<u8>) -> i64 {
        values
            .iter()
            .zip(self.weights.values())
            .fold(0_i64, |sum, (number, weight)| sum + (*number as i64) * *weight)
    }

    fn weigh_chars(words: &Vec<&str>) -> HashMap<char, i64> {
        let mut result = HashMap::new();
        for word in words.iter().take(words.len() - 1) {
            word.chars()
                .rev()
                .enumerate()
                .fold(&mut result, |acc, (i, c)| {
                    *acc.entry(c).or_default() += (10_i64).pow(i as u32);
                    acc
                });
        }
        words[words.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .fold(&mut result, |acc, (i, c)| {
                *acc.entry(c).or_default() -= (10_i64).pow(i as u32);
                acc
            });
        result
    }
}
