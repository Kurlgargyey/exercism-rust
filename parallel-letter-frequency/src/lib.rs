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

    let (output_tx, output_rx) = mpsc::channel();

    let mut workloads = input.chunks(input.len().div_ceil(worker_count));

    for _ in 0..worker_count {
        let tx = output_tx.clone();
        if let Some(chunk) = workloads.next() {
            let work_string = chunk.join("");
            thread::spawn(move || {
                tx.send(string_frequencies(work_string)).unwrap();
            });
        };
    }

    drop(output_tx);

    while let Ok(branch) = output_rx.recv() {
        root = merge_frequency_maps(root, branch);
    }
    root
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
