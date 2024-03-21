#![feature(test)]
/// Determine whether a sentence is a pangram.

const BITFIELD: bool = true;

pub fn is_pangram(sentence: &str) -> bool {
    is_pangram_bitfield_to_lower(sentence)
}

pub fn is_pangram_iter(sentence: &str) -> bool {
    let lowered_sentence = sentence.to_ascii_lowercase();
    ('a'..='z').all(|char| lowered_sentence.contains(char))
}

const A_LCASE: u8 = b'a';
const A_UCASE: u8 = b'A';
const ALL_26_BITS_SET: u32 = 0x3ffffff;

pub fn is_pangram_bitfield(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << ((letter as u8) - A_LCASE);
            //           ^ bitwise OR merges extant flags with the current letter flag
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << ((letter as u8) - A_UCASE);
            //                ^ leftshift generates one flag place for each letter ('a'-'a' -> idx 0, 'a'-'b' -> idx 1, etc..)
        }
    }

    ALL_26_BITS_SET == letter_flags
}

fn is_pangram_bitfield_to_lower(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter.is_ascii_alphabetic() {
            letter_flags |= 1 << ((letter.to_ascii_lowercase() as u8) - A_LCASE);
        }
    }

    ALL_26_BITS_SET == letter_flags
}

extern crate test;
mod tests {
    #[cfg(test)]
    use test::Bencher;
    use super::*;
    #[bench]
    fn bitfield(b: &mut Bencher) {
        let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
        b.iter(|| is_pangram_bitfield(test::black_box(sentence)));
    }

    #[bench]
    fn bitfield_to_lower(b: &mut Bencher) {
        let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
        b.iter(|| is_pangram_bitfield_to_lower(test::black_box(sentence)));
    }

    #[bench]
    fn iter(b: &mut Bencher) {
        let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
        b.iter(|| is_pangram_iter(test::black_box(sentence)));
    }
}
