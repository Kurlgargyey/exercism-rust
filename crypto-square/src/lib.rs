pub fn encrypt(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| translate_char(c))
        .collect::<Vec<char>>()
        .blocks(5)
}

fn translate_char(letter: char) -> Option<char> {
    match letter {
        c if c.is_ascii_alphabetic() => Some(atbash_translation(letter)),
        c if c.is_ascii_digit() => Some(c),
        _ => None,
    }
}

fn atbash_translation(letter: char) -> char {
    let letter_index = (letter.to_ascii_lowercase() as u32) - 'a' as u32;
    let inverted_char = 'z' as u32 - letter_index;
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
