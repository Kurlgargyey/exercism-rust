use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    //let mut handles = vec![];
    let frequencies = HashMap::<char, usize>::new();
    input
        .iter()
        .fold(frequencies, |acc, line| merge_line_frequencies(acc, line))
}

fn merge_line_frequencies(root: HashMap<char, usize>, line: &str) -> HashMap<char, usize> {
    //let frequencies = HashMap::<char, usize>::new();
    line.chars()
        .filter(|char| char.is_alphabetic())
        .fold(root, |mut acc, item| {
            *acc.entry(item.to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
}

/*
fn merge_frequencies(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |mut acc, pair| {
        *acc.entry(pair.0).or_insert(0) += pair.1;
        acc
    })
}*/
