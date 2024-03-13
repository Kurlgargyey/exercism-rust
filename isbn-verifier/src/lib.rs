/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut checksum = 0;
    let mut digit_iter = isbn.chars().filter(|c| *c != '-');

    for i in 0..10 {
        if let Some(char) = digit_iter.next() {
            match char {
                c if c.is_digit(10) => {
                    checksum += (10 - i) * (char.to_digit(10).unwrap() as usize);
                }
                'X' => {
                    if i != 9 {
                        return false;
                    }
                    checksum += 10;
                }
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    if let Some(_) = digit_iter.next() {
        return false;
    }

    checksum % 11 == 0
}
