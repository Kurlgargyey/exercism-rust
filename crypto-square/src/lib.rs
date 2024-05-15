pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    };

    let chars = &mut input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    let mut sanitized_input: String = chars.clone().collect();

    let (rows, cols) = find_rectangle(sanitized_input.len()).unwrap();

    let difference = rows * cols - sanitized_input.len();

    for _i in 1..=difference {
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
        let row: String = vec.iter().fold(String::new(), |mut result, col| {
            result.push(col[i]);
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
