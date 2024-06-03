pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| match c.is_alphabetic() {
            true => map_char(c, key),
            false => c,
        })
        .collect()
}

fn map_char(c: char, key: u8) -> char {
    match c.is_lowercase() {
        true => char::from_u32(((c as u32 - 'a' as u32 + key as u32) % 26) + 'a' as u32),
        false => char::from_u32(((c as u32 - 'A' as u32 + key as u32) % 26) + 'A' as u32),
    }
    .expect("invalid char!")
}
