#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    let mut max = 0;
    for chars in string_digits.chars().collect::<Vec<char>>().windows(span) {
        let curr = series_product(chars)?;
        if curr > max {
            max = curr
        }
    }
    Ok(max)
}

fn series_product(window: &[char]) -> Result<u64, Error> {
    let mut product = 1;
    for char in window.iter() {
        match char.to_digit(10) {
            Some(num) => product *= num as u64,
            None => return Err(Error::InvalidDigit(*char)),
        }
    }
    Ok(product)
}
