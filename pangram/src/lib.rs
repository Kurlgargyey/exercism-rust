/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = 'a'..='z';

    letters.all(|char| sentence.to_lowercase().contains(char))
}
