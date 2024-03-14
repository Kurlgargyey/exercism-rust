use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::<char>::new();
    for char in candidate.to_ascii_lowercase().chars() {
        match char {
            c if !c.is_alphabetic() => {
                continue;
            }
            _ => if !letters.insert(char) {
                return false;
            }
        }
    }
    true
}
