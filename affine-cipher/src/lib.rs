/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

use decrypt::*;
use encrypt::*;
use mmi::*;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(26, a) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(blocks_of_five(
        plaintext
            .chars()
            .filter_map(|c| encrypt(c, a, b))
            .collect::<Vec<char>>(),
    ))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(26, a) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(ciphertext
        .chars()
        .filter_map(|c| decrypt(c, a, b))
        .collect())
}

mod decrypt {
    use super::mmi::*;
    pub(crate) fn decrypt(letter: char, a: i32, b: i32) -> Option<char> {
        match letter {
            c if c.is_ascii_alphabetic() => Some(decipher_char(letter, a, b)),
            c if c.is_ascii_digit() => Some(c),
            _ => None,
        }
    }

    fn decipher_char(letter: char, a: i32, b: i32) -> char {
        let a = a;
        let b = b;
        let y = (letter.to_ascii_lowercase() as i32) - 97;
        let mmi = mmi(a, 26).unwrap();
        let char_value = (mmi * (y - b)).rem_euclid(26);

        char::from_u32((char_value + 97) as u32).unwrap()
    }
}

mod encrypt {
    pub(crate) fn encrypt(letter: char, a: i32, b: i32) -> Option<char> {
        match letter {
            c if c.is_ascii_alphabetic() => Some(cipher_char(letter, a, b)),
            c if c.is_ascii_digit() => Some(c),
            _ => None,
        }
    }

    fn cipher_char(letter: char, a: i32, b: i32) -> char {
        let a = a as u32;
        let b = b as u32;
        let i = (letter.to_ascii_lowercase() as u32) - 97;
        char::from_u32(((a * i + b).rem_euclid(26)) + 97).unwrap()
    }
    pub(crate) fn blocks_of_five(phrase: Vec<char>) -> String {
        phrase
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

mod mmi {

    pub(super) fn are_coprime(n: i32, m: i32) -> bool {
        mmi(n, m).is_some()
    }
    pub(crate) fn mmi(a: i32, n: i32) -> Option<i32> {
        let mut t = 0;
        let mut newt = 1;
        let mut r = n;
        let mut newr = a;

        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }
        if r > 1 {
            return None;
        }
        if t < 0 {
            t = t + n;
        }

        Some(t)
    }
}
