use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(is_separator)
        .map(|word| {
            word.to_lowercase()
                .trim_matches('\'')
                .to_string()
        })
        .filter(|word| !word.is_empty())
        .fold(HashMap::<String, u32>::new(), |mut counts, word| {
            println!("counting {}", word);
            *counts.entry(word).or_default() += 1;
            counts
        })
}

fn is_separator(c: char) -> bool {
    match c {
        '\'' => false,
        c if char::is_whitespace(c) => true,
        c if char::is_ascii_punctuation(&c) => true,
        _ => false,
    }
}
