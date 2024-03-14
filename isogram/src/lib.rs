use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .all(|c| letters.insert(c))
}
