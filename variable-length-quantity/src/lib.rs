#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

const PRECEDING_BYTE: u8 = 0b10000000;
const FINAL_BYTE: u8 = 0b01111111;
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    for value in values {
        result.append(&mut encode(*value));
    }
    result
}

fn encode(number: u32) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    let mut number = number;

    result.push(self::FINAL_BYTE & number.to_ne_bytes().iter().next().unwrap());
    number >>= 7;
    while number != 0 {
        result.push(self::PRECEDING_BYTE | number.to_ne_bytes().iter().next().unwrap());
        number >>= 7;
    }
    result.into_iter().rev().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::<u32>::new();
    if bytes.len() == 0 {
        return Ok(result);
    }
    let mut subresult: u32 = 0;
    for byte in bytes {
        subresult += extract_number_value(byte);
        if is_final_byte(byte) {
            result.push(subresult);
            subresult = 0;
        }
        subresult <<= 7;
    }
    Ok(result)
}

fn extract_number_value(byte: &u8) -> u32 {
    (byte & self::FINAL_BYTE) as u32
}

fn is_final_byte(byte: &u8) -> bool {
    (byte & self::PRECEDING_BYTE) == 0
}
