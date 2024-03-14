use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        c if "ACGT".contains(c) =>
            Ok(
                dna
                    .chars()
                    .try_fold(0, |count, candidate| {
                        checked_count(count, candidate, nucleotide)
                    })?
            ),
        _ => Err(nucleotide),
    }
}

fn checked_count(count: usize, candidate: char, target: char) -> Result<usize, char> {
    match candidate {
        c if "ACGT".contains(c) => {
            if candidate == target { Ok(count + 1) } else { Ok(count) }
        }
        _ => { Err(candidate) }
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
        if let Some(nuc) = acc.get_mut(&cand) {
            *nuc += 1;
            Ok(acc)
        } else {
            Err(cand)
        }
    })
}
