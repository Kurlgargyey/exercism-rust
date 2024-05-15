pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    };
    let chars = &mut input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase());
    let mut sanitized_input: String = chars.clone().collect();
    println!("{}", sanitized_input);
    let (rows, cols) = find_rectangle(sanitized_input.len()).unwrap();
    println!("rows: {}, cols: {}", rows, cols);
    let difference = rows * cols - sanitized_input.len();
    println!("{}", difference);
    for i in 1..=difference {
        sanitized_input.push(' ');
    }
    let vec: Vec<Vec<char>> = sanitized_input
        .chars()
        .collect::<Vec<char>>()
        .chunks(rows)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut result: Vec<String> = Vec::<String>::new();
    for i in 0..rows {
        let row: String = vec.iter().fold(String::new(), |mut result, row| {
            result.push(row[i]);
            result
        });
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
