/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned_code = code.trim().split_whitespace().collect::<String>();
    if !correct_format(&cleaned_code) {
        return false;
    }

    cleaned_code
        .chars()
        .rev()
        .enumerate()
        .map(|(i, raw_digit)| match (i + 1) % 2 {
            0 => double_digit(&raw_digit),
            _ => raw_digit,
        })
        .fold(0, |acc, final_digit| {
            acc + final_digit.to_digit(10).unwrap()
        })
        % 10
        == 0
}

fn double_digit(digit: &char) -> char {
    vec!['0', '2', '4', '6', '8', '1', '3', '5', '7', '9'][digit.to_digit(10).unwrap() as usize]
}

fn correct_format(code: &String) -> bool {
    !(code.len() < 2 || code.chars().any(|c| !c.is_digit(10)))
}
