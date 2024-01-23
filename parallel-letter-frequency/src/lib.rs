use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        x if x < 300 => slice_frequencies(input),
        _ => par_frequencies(input),
    }
}

fn count_letter(mut map: HashMap<char, usize>, letter: char, tally: usize) -> HashMap<char, usize> {
    map.entry(letter)
        .and_modify(|count| *count += tally)
        .or_insert(tally);
    map
}

fn slice_frequencies(slice: &[&str]) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    slice.iter().fold(root, |map, line| {
        line.chars()
            .filter(|char| char.is_alphabetic())
            .fold(map, |map, letter| count_letter(map, letter, 1))
    })
}

fn par_frequencies(slice: &[&str]) -> HashMap<char, usize> {
    slice
        .join("")
        .par_chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new, |map, letter| count_letter(map, letter, 1))
        .reduce(HashMap::new, |root, branch| merge_counts(root, branch))
}

fn merge_counts(root: HashMap<char, usize>, branch: HashMap<char, usize>) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |map, (letter, count)| {
        count_letter(map, letter, count)
    })
}
