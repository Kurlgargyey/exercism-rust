/// Determine whether a sentence is a pangram.

const BITFIELD: bool = true;

pub fn is_pangram(sentence: &str) -> bool {
    match BITFIELD {
        false => ('a'..='z').all(|char| sentence.to_ascii_lowercase().contains(char)),
        true => { is_pangram_bitfield(sentence) }
    }
}

const A_LCASE: u8 = b'a';
//const A_UCASE: u8 = b'A';
const ALL_26_BITS_SET: u32 = 0x3ffffff;

fn is_pangram_bitfield(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter.is_ascii_alphabetic() {
            letter_flags |= 1 << ((letter.to_ascii_lowercase() as u8) - A_LCASE);
        }

        /*
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << ((letter as u8) - A_LCASE);
            //           ^ bitwise OR merges extant flags with the current letter flag
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << ((letter as u8) - A_UCASE);
            //                ^ leftshift generates one flag place for each letter ('a'-'a' -> idx 0, 'a'-'b' -> idx 1, etc..)
        }
        */
    }

    ALL_26_BITS_SET == letter_flags
}
