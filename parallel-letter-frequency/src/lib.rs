use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    const MIN_INPUT_SIZE: usize = 500;

    if input.len() <= MIN_INPUT_SIZE {
        return slice_frequencies(input);
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

    let mut root = HashMap::<char, usize>::new();
    while let Ok(branch) = output_rx.recv() {
        root = merge_frequency_maps(root, branch);
    }
    root
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
