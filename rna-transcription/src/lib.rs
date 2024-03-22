#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);
const DNA_NUCLEOTIDES: &'static str = "GCTA";
const RNA_NUCLEOTIDES: &'static str = "CGAU";
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut result = String::new();
        for (idx, ch) in dna.chars().enumerate() {
            if crate::DNA_NUCLEOTIDES.contains(ch) {
                result.push(ch);
            } else {
                return Err(idx);
            }
        }
        Ok(Dna(result))
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
        let mut result = String::new();
        for (idx, ch) in rna.chars().enumerate() {
            if crate::RNA_NUCLEOTIDES.contains(ch) {
                result.push(ch);
            } else {
                return Err(idx);
            }
        }
        Ok(Rna(result))
    }
}
