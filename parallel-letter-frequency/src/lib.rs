use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    const MIN_INPUT_SIZE: usize = 500;

    if input.len() <= MIN_INPUT_SIZE {
        return slice_frequencies(input);
    }

    input
        .par_iter()
        .flat_map(|str| str.par_chars())
        .filter(|char| char.is_alphabetic())
        .fold(
            || HashMap::new(),
            |mut map, letter| {
                *map.entry(letter.to_ascii_lowercase()).or_insert(0) += 1;
                map
            },
        )
        .reduce(
            || HashMap::new(),
            |root, branch| merge_frequency_maps(root, branch),
        )
}

fn slice_frequencies(slice: &[&str]) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    slice.iter().fold(root, |map, line| {
        line.chars()
            .filter(|char| char.is_alphabetic())
            .fold(map, |mut map, letter| {
                *map.entry(letter.to_ascii_lowercase()).or_insert(0) += 1;
                map
            })
    })
}

fn string_frequencies(string: String) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    string
        .chars()
        .filter(|char| char.is_alphabetic())
        .fold(root, |mut map, letter| {
            *map.entry(letter.to_ascii_lowercase()).or_insert(0) += 1;
            map
        })
}

fn merge_frequency_maps(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |mut map, (letter, count)| {
        *map.entry(letter).or_insert(0) += count;
        map
    })
}
