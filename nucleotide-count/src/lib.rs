use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    dna.chars().try_fold(0, |count, candidate| { checked_count(count, candidate, nucleotide) })
}

fn checked_count(count: usize, candidate: char, target: char) -> Result<usize, char> {
    let valid_nucs = "ACGT";
    match (valid_nucs.contains(candidate), valid_nucs.contains(target)) {
        (false, _) => Err(candidate),
        (_, false) => Err(target),
        _ => if candidate == target { Ok(count + 1) } else { Ok(count) }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let result = HashMap::<char, usize>::from([
        ('A', 0),
        ('C', 0),
        ('G', 0),
        ('T', 0),
    ]);
    dna.chars().try_fold(result, |mut acc, cand| {
        let nuc = acc.get_mut(&cand).ok_or(cand)?;
        *nuc += 1;
        Ok(acc)
    })
}
