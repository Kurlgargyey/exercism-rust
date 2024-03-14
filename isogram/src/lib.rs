use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::<char>::new();
    for char in candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase()) {
        if !letters.insert(char) {
            return false;
        }
    }
    true
}
