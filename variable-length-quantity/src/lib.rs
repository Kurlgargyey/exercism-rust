#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

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
    let preceding_byte: u8 = 0b10000000;
    let final_byte: u8 = 0b01111111;
    result.push(final_byte & number.to_ne_bytes().iter().next().unwrap());
    number = number >> 7;
    while number != 0 {
        result.push(preceding_byte | number.to_ne_bytes().iter().next().unwrap());
        number = number >> 7;
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    todo!("Convert the list of bytes {bytes:?} to a list of numbers")
}
