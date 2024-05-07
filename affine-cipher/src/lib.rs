/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

use coprime::*;
use decrypt::*;
use encrypt::*;

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
    pub(crate) fn decrypt(letter: char, a: i32, b: i32) -> Option<char> {
        match letter {
            c if c.is_ascii_alphabetic() => Some(decipher_char(letter, a, b)),
            c if c.is_ascii_digit() => Some(c),
            _ => None,
        }
    }

    fn decipher_char(letter: char, a: i32, b: i32) -> char {
        let a = a as f64;
        println!("a is {a}");
        let b = b as i32;
        println!("b is {b}");
        let y = (letter.to_ascii_lowercase() as i32) - 97;
        println!("{letter}: {y}");
        let mmi = a.powi(-1);
        println!("mmi is {mmi}");
        let char_value = mmi * ((y - b) as f64) % 26.0;
        println!("char_value is {char_value}");
        println!("---------");
        char::from_u32(char_value as u32 + 97).unwrap()
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
        char::from_u32(((a * i + b) % 26) + 97).unwrap()
    }
    pub(crate) fn blocks_of_five(phrase: Vec<char>) -> String {
        let mut i = 0;
        let mut result = String::new();
        println!("breaking up {:?}", phrase);
        for letter in phrase {
            if i > 0 && i % 5 == 0 {
                println!("adding whitespace before {letter}, index {i}");
                result.push_str(" ");
            }
            result.push(letter);
            println!("pushing {letter}, index {i}");
            i += 1;
        }
        result
    }
}

mod coprime {
    use std::cmp::min;
    use std::mem::swap;

    pub(super) fn are_coprime(n: i32, m: i32) -> bool {
        let n = n as u64;
        let m = m as u64;
        gcd(n, m) == 1
    }

    fn gcd(mut n: u64, mut m: u64) -> u64 {
        // Stein's binary GCD algorithm
        // Base cases: gcd(n, 0) = gcd(0, n) = n
        if n == 0 {
            return m;
        } else if m == 0 {
            return n;
        }

        // Extract common factor-2: gcd(2ⁱ n, 2ⁱ m) = 2ⁱ gcd(n, m)
        // and reducing until odd gcd(2ⁱ n, m) = gcd(n, m) if m is odd
        let k = {
            let k_n = n.trailing_zeros();
            let k_m = m.trailing_zeros();
            n >>= k_n;
            m >>= k_m;
            min(k_n, k_m)
        };

        loop {
            // Invariant: n odd
            debug_assert!(n % 2 == 1, "n = {} is even", n);

            if n > m {
                swap(&mut n, &mut m);
            }
            m -= n;

            if m == 0 {
                return n << k;
            }

            m >>= m.trailing_zeros();
        }
    }
}

mod mmi {
    pub(crate) fn mmi(a: i32, m: i32) -> i32 {}
}
