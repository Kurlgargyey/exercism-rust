pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            word.chars()
                .filter(|c| !c.is_ascii_punctuation())
                .collect::<String>()
        })
        .map(|word| {
            if word.chars().all(|c| c.is_ascii_uppercase()) {
                word.chars().next().unwrap().to_string()
            } else {
                word.chars()
                    .enumerate()
                    .filter(|(pos, c)| {
                        c.is_alphanumeric() && (*pos == 0usize || c.is_ascii_uppercase())
                    })
                    .map(|(_, c)| c.to_ascii_uppercase())
                    .collect::<String>()
            }
        })
        .collect::<String>()
}
