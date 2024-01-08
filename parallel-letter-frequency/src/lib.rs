use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = vec![];
    //let counter = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let mut root = HashMap::<char, usize>::new();
    //let input = Arc::new(input);
    for line in input.iter() {
        //let clone_counter = Arc::clone(&counter);
        let thread_line = Arc::new(line.to_string());
        //                                              ^ take ownership of the string, so that there is no potential for a dangling reference!!
        let handle = thread::spawn(move || {
            /*
            let mut thread_counter = clone_counter.lock().unwrap();
            for char in thread_line.chars().filter(|char| char.is_alphabetic()) {
                *thread_counter.entry(char.to_ascii_lowercase()).or_insert(0) += 1;
            }
            */
            line_frequencies(&thread_line)
        });
        handles.push(handle);
    }

    for handle in handles {
        root = merge_frequencies(root, handle.join().unwrap());
    }
    //consume_refs(counter)
    root
}

fn line_frequencies(line: &String) -> HashMap<char, usize> {
    let root = HashMap::<char, usize>::new();
    line.chars()
        .filter(|char| char.is_alphabetic())
        .fold(root, |mut acc, item| {
            *acc.entry(item.to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
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

fn merge_frequencies(
    root: HashMap<char, usize>,
    branch: HashMap<char, usize>,
) -> HashMap<char, usize> {
    branch.into_iter().fold(root, |mut acc, pair| {
        *acc.entry(pair.0).or_insert(0) += pair.1;
        acc
    })
}

fn consume_refs<T>(reference: Arc<Mutex<T>>) -> T {
    Arc::into_inner(reference).unwrap().into_inner().unwrap()
}
