#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match from_base {
        0 | 1 => {
            return Err(Error::InvalidInputBase);
        }
        _ => {}
    }

    match to_base {
        0 | 1 => {
            return Err(Error::InvalidOutputBase);
        }
        _ => {}
    }

    match number {
        s if s.is_empty() || s == [0] => Ok(vec![0]),
        [1] => Ok(vec![1]),
        _ => {
            for (place, digit) in number.iter().enumerate() {
                if digit >= &from_base {
                    return Err(Error::InvalidDigit((place as u32) + 1));
                }
            }

            let mut result = 0;
            for digit in number.iter().rev().enumerate() {
                result += digit.1 * from_base.pow(digit.0 as u32);
            }
            let digits_iter = (0..)
                .map(|exp| to_base.pow(exp))
                .take_while(|place| place <= &result)
                .collect::<Vec<u32>>()
                .into_iter()
                .rev()
                .map(|place| {
                    let multiple = result / place;
                    result = result % place;
                    multiple
                });
            let result_digits: Vec<u32> = digits_iter.collect();
            if result_digits.is_empty() {
                return Ok(vec![0]);
            }
            Ok(result_digits)
        }
    }
}
