/// "Encipher" with the Atbash cipher.

pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| translate_char(c))
        .collect::<Vec<char>>()
        .blocks(5)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(|c| translate_char(c)).collect()
}

fn translate_char(letter: char) -> Option<char> {
    match letter {
        c if c.is_ascii_alphabetic() => Some(atbash_translation(letter)),
        c if c.is_ascii_digit() => Some(c),
        _ => None,
    }
}

fn atbash_translation(letter: char) -> char {
    let i = (letter.to_ascii_lowercase() as u32) - 'a' as u32;
    let inverted_char = 'z' as u32 - i;
    char::from_u32(inverted_char).unwrap()
}

trait Blocks {
    fn blocks(&self, size: usize) -> String;
}

impl Blocks for Vec<char> {
    fn blocks(&self, size: usize) -> String {
        self.chunks(size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
