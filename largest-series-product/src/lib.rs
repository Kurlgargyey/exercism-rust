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
    window.iter().fold(Ok(1_u64), |product, digit| {
        if product.is_ok() {
            if let Some(num) = digit.to_digit(10) {
                Ok(product.unwrap() * num as u64)
            } else {
                Err(Error::InvalidDigit(*digit))
            }
        } else {
            product
        }
    })
}
