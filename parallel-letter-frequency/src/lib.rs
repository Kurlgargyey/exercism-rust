use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut root = HashMap::<char, usize>::new();

    if input.is_empty() {
        return root;
    }

    const MIN_INPUT_SIZE: usize = 30;

    if input.len() <= MIN_INPUT_SIZE {
        return string_frequencies(input.join(""));
    }

    let (output_tx, output_rx) = mpsc::channel();

    let mut workloads = input.chunks_exact(input.len().div_ceil(worker_count));
    //.map(|chunk| chunk.join(""));

    for _ in 0..worker_count {
        let work_tx = output_tx.clone();
        if let Some(chunk) = workloads.next() {
            let work_string = chunk.join("");
            thread::spawn(move || {
                let _ = work_tx.send(string_frequencies(work_string));
            });
        } else {
            let remainder = workloads.remainder();
            let work_remainder = remainder.join("");
            thread::spawn(move || {
                let _ = work_tx.send(string_frequencies(work_remainder));
            });
            break;
        }
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
        .fold(root, |mut map, letter| {
            *map.entry(letter.to_ascii_lowercase()).or_insert(0) += 1;
            map
        })
}

fn merge_frequency_maps(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |mut map, kv_pair| {
        *map.entry(kv_pair.0).or_insert(0) += kv_pair.1;
        map
    })
}
