pub fn abbreviate(phrase: &str) -> String {
    let components = phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            word.chars()
                .filter(|ch| !ch.is_ascii_punctuation())
                .collect::<String>()
        })
        .map(|word| {
            if word.chars().all(|ch| ch.is_ascii_uppercase()) {
                word.chars().next().unwrap().to_string()
            } else {
                word.chars()
                    .enumerate()
                    .filter(|(pos, char)| {
                        char.is_alphanumeric() && (*pos == 0usize || char.is_ascii_uppercase())
                    })
                    .map(|(_, char)| char.to_ascii_uppercase())
                    .collect::<String>()
            }
        })
        .collect::<String>();
    components
}
