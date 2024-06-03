pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| match c.is_alphabetic() {
            true => cipher_char(c, key),
            false => c,
        })
        .collect()
}

fn cipher_char(c: char, key: u8) -> char {
    match c.is_lowercase() {
        true => rotate_char(c, key, 'a'),
        false => rotate_char(c, key, 'A'),
    }
}

fn rotate_char(c: char, key: u8, offset: char) -> char {
    char::from_u32(((c as u32 - offset as u32 + key as u32) % 26) + offset as u32)
        .expect("invalid char!")
}
