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
            |map, letter| tally_letter(map, letter, 1),
        )
        .reduce(
            || HashMap::new(),
            |root, branch| merge_frequency_maps(root, branch),
        )
}

fn tally_letter(mut map: HashMap<char, usize>, letter: char, tally: usize) -> HashMap<char, usize> {
    *map.entry(letter.to_ascii_lowercase()).or_default() += tally;
    map
}

fn slice_frequencies(slice: &[&str]) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    slice.iter().fold(root, |map, line| {
        line.chars()
            .filter(|char| char.is_alphabetic())
            .fold(map, |map, letter| tally_letter(map, letter, 1))
    })
}

/*
fn string_frequencies(string: String) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    string
    .chars()
    .filter(|char| char.is_alphabetic())
    .fold(root, |map, letter| tally_letter(map, letter, 1))
}
*/

fn merge_frequency_maps(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |map, (letter, count)| {
        tally_letter(map, letter, count)
    })
}
