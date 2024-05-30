pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let cleaned_code = self.0.trim().split_whitespace().collect::<String>();
        if cleaned_code.len() <= 1 {
            return false;
        }
        let digits = Self::turn_into_digits(&cleaned_code);

        match digits {
            Some(vec) => {
                let mut tmp: u32 = 0;
                vec.into_iter()
                    .enumerate()
                    .fold(0, |chk, (idx, dgt)| match (idx + 1) % 2 {
                        0 => {
                            tmp = dgt * 2;
                            if tmp > 9 {
                                tmp -= 9
                            }
                            chk + tmp
                        }
                        _ => chk + dgt,
                    })
                    % 10
                    == 0
            }
            None => false,
        }
    }

    fn turn_into_digits(code: &String) -> Option<Vec<u32>> {
        let mut output = Vec::<u32>::with_capacity(code.len());
        for ch in code.chars().rev() {
            output.push(ch.to_digit(10)?)
        }
        Some(output)
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
