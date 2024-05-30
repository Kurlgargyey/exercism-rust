pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let cleaned_code = &self.0.trim().split_whitespace().collect::<String>();
        if !Self::correct_format(&cleaned_code) {
            return false;
        }

        cleaned_code
            .chars()
            .rev()
            .enumerate()
            .map(|(i, raw_digit)| match (i + 1) % 2 {
                0 => Self::double_digit(&raw_digit),
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
