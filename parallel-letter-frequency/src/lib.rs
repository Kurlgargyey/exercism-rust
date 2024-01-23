use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .join("")
        .par_chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new, |map, letter| count_letter(map, letter, 1))
        .reduce(HashMap::new, |root, branch| merge_counts(root, branch))
}

fn count_letter(mut map: HashMap<char, usize>, letter: char, tally: usize) -> HashMap<char, usize> {
    map.entry(letter)
        .and_modify(|count| *count += tally)
        .or_insert(tally);
    map
}

fn merge_counts(root: HashMap<char, usize>, branch: HashMap<char, usize>) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |map, (letter, count)| {
        count_letter(map, letter, count)
    })
}
