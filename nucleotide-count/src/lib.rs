use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    str.chars()
        .filter(|c| c == nucleotide)
        .count()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    todo!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
}
