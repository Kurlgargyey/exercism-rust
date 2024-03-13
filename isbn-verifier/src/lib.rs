/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut result = 0;
    let digit_iter = isbn
        .chars()
        .filter(|c| *c != '-')
        .enumerate();

    for (idx, char) in digit_iter {
        if idx > 9 {
            return false;
        }
        match char {
            c if c.is_digit(10) => {
                result += (10 - idx) * (char.to_digit(10).unwrap() as usize);
            }
            'X' => {
                if idx != 9 {
                    return false;
                }
                result += (10 - idx) * 10;
            }
            _ => {
                return false;
            }
        }
    }

    result % 11 == 0
}
