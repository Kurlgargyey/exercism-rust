pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window| window.join(""))
        .collect()
}
