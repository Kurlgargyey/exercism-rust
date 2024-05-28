#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .map(|chars| chars.iter().collect::<String>())
        .fold(Ok(0_u64), |acc, series| {
            let series_product = series_product(&series)?;
            println!("series product is: {}", series_product);
            if let Ok(prev) = acc {
                if series_product > prev {
                    Ok(series_product)
                } else {
                    Ok(prev)
                }
            } else {
                acc
            }
        })
}

fn series_product(string: &str) -> Result<u64, Error> {
    string.chars().fold(Ok(1_u64), |product, digit| {
        if product.is_ok() {
            if let Some(num) = digit.to_digit(10) {
                Ok(product.unwrap() * num as u64)
            } else {
                Err(Error::InvalidDigit(digit))
            }
        } else {
            product
        }
    })
}
