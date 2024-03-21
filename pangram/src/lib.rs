/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    ('a'..='z').all(|char| sentence.to_lowercase().contains(char))
}
