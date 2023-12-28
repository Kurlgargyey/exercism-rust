use std::collections::{BinaryHeap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_sorted_chars = word.chars().collect::<BinaryHeap<_>>().into_sorted_vec();
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate = candidate.to_lowercase();
            word != candidate
                && (word_sorted_chars
                    == candidate
                        .chars()
                        .collect::<BinaryHeap<_>>()
                        .into_sorted_vec())
        })
        .copied()
        .collect()
}
