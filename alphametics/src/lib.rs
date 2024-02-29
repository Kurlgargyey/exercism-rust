use std::collections::{ HashMap, HashSet };
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let solver = Solver::new(input).ok()?;

    for permutation in (0..=9).permutations(solver.char_count) {
        if let Ok(solution) = solver.test_solution(permutation) {
            return Some(solution);
        }
    }

    None
}

struct Solver {
    leading_chars: HashSet<char>,
    char_count: usize,
    weights: HashMap<char, i64>,
}

#[derive(Debug)]
enum Error {
    LeadingZero,
    TooManyUniqueChars,
    OverlongAddend,
    FaultyChecksum,
}

impl Solver {
    pub fn new(input: &str) -> Result<Self, Error> {
        let terms = Self::split_terms(input)?;
        let char_count = Self::count_unique_chars(&terms)?;
        let leading_chars = Self::find_leading_chars(&terms);
        let weights = Self::weigh_chars(&terms);
        Ok(Solver { leading_chars, char_count, weights })
    }

    pub fn test_solution(&self, values: Vec<u8>) -> Result<HashMap<char, u8>, Error> {
        match Self::generate_checksum(&self, &values) {
            0 => self.test_for_leading_zero(values),
            _ => Err(Error::FaultyChecksum),
        }
    }

    fn generate_checksum(&self, values: &Vec<u8>) -> i64 {
        values
            .iter()
            .zip(self.weights.values())
            .fold(0_i64, |sum, (number, weight)| sum + (*number as i64) * *weight)
    }

    fn test_for_leading_zero(&self, values: Vec<u8>) -> Result<HashMap<char, u8>, Error> {
        let possible_solution: HashMap<char, u8> = HashMap::from_iter(
            self.weights.clone().into_keys().zip(values.into_iter())
        );
        if self.leading_chars.iter().any(|leading| *possible_solution.get(&leading).unwrap() == 0) {
            return Err(Error::LeadingZero);
        }
        Ok(possible_solution)
    }

    fn weigh_chars(terms: &Vec<&str>) -> HashMap<char, i64> {
        let mut result = HashMap::new();
        for word in terms.iter().take(terms.len() - 1) {
            word.chars()
                .rev()
                .enumerate()
                .fold(&mut result, |acc, (i, c)| {
                    *acc.entry(c).or_default() += (10_i64).pow(i as u32);
                    acc
                });
        }
        terms[terms.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .fold(&mut result, |acc, (i, c)| {
                *acc.entry(c).or_default() -= (10_i64).pow(i as u32);
                acc
            });
        result
    }

    fn split_terms(input: &str) -> Result<Vec<&str>, Error> {
        let terms: Vec<&str> = input
            .split(|c| (c == '+' || c == '='))
            .filter(|word| !word.is_empty())
            .map(|word| word.trim())
            .collect();
        if
            terms
                .iter()
                .take(terms.len() - 1)
                .any(|word| word.len() > terms[terms.len() - 1].len())
        {
            return Err(Error::OverlongAddend);
        }
        Ok(terms)
    }

    fn find_leading_chars(terms: &Vec<&str>) -> HashSet<char> {
        terms.iter().fold(HashSet::new(), |mut acc, word| {
            acc.insert(word.chars().nth(0).unwrap());
            acc
        })
    }

    fn count_unique_chars(terms: &Vec<&str>) -> Result<usize, Error> {
        let char_count = terms
            .iter()
            .fold(HashSet::new(), |acc, word|
                word.chars().fold(acc, |mut acc, char| {
                    acc.insert(char);
                    acc
                })
            )
            .len();
        if char_count > 10 {
            return Err(Error::TooManyUniqueChars);
        }
        Ok(char_count)
    }
}
