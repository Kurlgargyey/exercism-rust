#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

const SIGN_BIT: u8 = 0b10000000;
const VALUE_BITS: u8 = 0b01111111;
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

    let next_byte = number.to_ne_bytes().into_iter().next().unwrap();
    result.push(set_value_bits_only(next_byte));
    number >>= 7;
    while number != 0 {
        let next_byte = number.to_ne_bytes().into_iter().next().unwrap();
        result.push(set_sign_bit(next_byte));
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
    if subresult > 0 {
        return Err(Error::IncompleteNumber);
    }
    Ok(result)
}

fn extract_number_value(byte: &u8) -> u32 {
    (byte & self::VALUE_BITS) as u32
}

fn is_final_byte(byte: &u8) -> bool {
    (byte & self::SIGN_BIT) == 0
}

fn set_sign_bit(byte: u8) -> u8 {
    self::SIGN_BIT | byte
}

fn set_value_bits_only(byte: u8) -> u8 {
    self::VALUE_BITS & byte
}
