#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);
const DNA_NUCLEOTIDES: &'static str = "GCTA";
const RNA_NUCLEOTIDES: &'static str = "CGAU";
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(bad_idx) = dna.find(|c| !crate::DNA_NUCLEOTIDES.contains(c)) {
            return Err(bad_idx);
        }

        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna(
            self.0
                .chars()
                .map(|ch|
                    crate::RNA_NUCLEOTIDES
                        .chars()
                        .nth(crate::DNA_NUCLEOTIDES.find(ch).unwrap())
                        .unwrap()
                )
                .collect()
        )
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(bad_idx) = rna.find(|c| !crate::RNA_NUCLEOTIDES.contains(c)) {
            return Err(bad_idx);
        }

        Ok(Rna(rna.to_string()))
    }
}
