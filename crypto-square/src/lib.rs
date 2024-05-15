pub fn encrypt(input: &str) -> String {
    let chars = &mut input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase());
    let mut sanitized_input: String = chars.clone().collect();
    println!("{}", sanitized_input);
    let dimensions = find_rectangle(sanitized_input.len()).unwrap();
    println!("{:?}", dimensions);
    let difference = dimensions.0 * dimensions.1 - sanitized_input.len();
    println!("{}", difference);
    for i in 1..=difference {
        sanitized_input.push(' ');
    }
    let vec = sanitized_input
        .chars()
        .collect::<Vec<char>>()
        .chunks(dimensions.0);
    let mut result: Vec<String> = Vec::<String>::new();
    for i in 1..=dimensions.0 - difference {
        let row: String = chars.take(dimensions.1).collect();
        println!("{}", row);
        result.push(row);
    }

    result.join(" ")
}

fn find_rectangle(length: usize) -> Option<(usize, usize)> {
    let floor = (length as f64).sqrt().floor() as usize;

    for i in floor..=length {
        if are_valid_dimensions(i, floor, length) {
            return Some((i, floor));
        }
    }
    None
}

fn are_valid_dimensions(side1: usize, side2: usize, length: usize) -> bool {
    side1 * side2 >= length && side1 >= side2 && side1 - side2 <= 1
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
