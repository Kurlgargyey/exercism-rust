use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    const MIN_INPUT_SIZE: usize = 500;
    match input.len() {
        0 => HashMap::new(),
        x if x < MIN_INPUT_SIZE => slice_frequencies(input),
        _ => par_frequencies(input, worker_count),
    }
}

fn slice_frequencies(slice: &[&str]) -> HashMap<char, usize> {
    slice.iter().fold(HashMap::new(), |map, line| {
        line.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .fold(map, |map, letter| insert_count(map, letter, 1))
    })
}

fn string_frequencies(string: String) -> HashMap<char, usize> {
    string
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new(), |map, letter| insert_count(map, letter, 1))
}

fn par_frequencies(slice: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (output_tx, output_rx) = mpsc::channel();

    let mut workloads = slice.chunks(slice.len().div_ceil(worker_count));

    for _ in 0..worker_count {
        let work_tx = output_tx.clone();
        if let Some(chunk) = workloads.next() {
            let work_string = chunk.join("");
            thread::spawn(move || {
                let _ = work_tx.send(string_frequencies(work_string));
            });
        } else {
            break;
        }
    }

    drop(output_tx);

    let mut result = HashMap::new();
    while let Ok(partial) = output_rx.recv() {
        result = merge_counts(result, partial);
    }
    result
}

fn merge_counts(root: HashMap<char, usize>, branch: HashMap<char, usize>) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |map, (letter, count)| {
        insert_count(map, letter, count)
    })
}

fn insert_count(mut map: HashMap<char, usize>, letter: char, count: usize) -> HashMap<char, usize> {
    *map.entry(letter).or_default() += count;
    map
}
