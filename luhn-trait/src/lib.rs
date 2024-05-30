pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let cleaned_code = self
            .to_string()
            .trim()
            .split_whitespace()
            .collect::<String>();
        if cleaned_code.len() <= 1 {
            return false;
        }
        fn turn_into_digits(code: &String) -> Option<Vec<u32>> {
            let mut output = Vec::<u32>::with_capacity(code.len());
            for ch in code.chars().rev() {
                output.push(ch.to_digit(10)?)
            }
            Some(output)
        }
        let digits = turn_into_digits(&cleaned_code);

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
}
