pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-')
        .flat_map(|word| {
            let word_iter = word.chars().skip_while(|c| !c.is_alphanumeric());
            word_iter.clone().take(1).chain(
                word_iter
                    .skip(1)
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
