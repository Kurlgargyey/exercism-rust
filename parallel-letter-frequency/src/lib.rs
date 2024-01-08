use spmc;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut root = HashMap::<char, usize>::new();

    if input.is_empty() {
        return root;
    }

    if input.len() <= 30 {
        return string_frequencies(input.join(""));
    }
    let (mut input_tx, input_rx) = spmc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    for chunk in input.chunks(worker_count) {
        input_tx.send(chunk.join("")).unwrap();
    }
    drop(input_tx);

    for _ in 0..worker_count {
        let tx = output_tx.clone();
        let rx = input_rx.clone();
        thread::spawn(move || {
            while let Ok(chunk) = rx.recv() {
                tx.send(string_frequencies(chunk)).unwrap();
            }
        });
    }

    drop(input_rx);
    drop(output_tx);

    while let Ok(branch) = output_rx.recv() {
        root = merge_frequency_maps(root, branch);
    }
    root
}

fn chunk_frequencies(chunk: &[&str]) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    chunk.iter().fold(root, |acc, line| {
        merge_frequency_maps(acc, line_frequencies(line))
    })
}

fn line_frequencies(line: &str) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    line.chars()
        .filter(|char| char.is_alphabetic())
        .fold(root, |mut acc, item| {
            *acc.entry(item.to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
}

fn string_frequencies(string: String) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    string
        .chars()
        .filter(|char| char.is_alphabetic())
        .fold(root, |mut acc, item| {
            *acc.entry(item.to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
}

fn merge_frequency_maps(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |mut acc, pair| {
        *acc.entry(pair.0).or_insert(0) += pair.1;
        acc
    })
}
