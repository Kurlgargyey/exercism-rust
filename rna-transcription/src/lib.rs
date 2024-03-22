#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    const DNA_NUCLEOTIDES: String = "ACGT";
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let result = String::new();
        for ch in dna.chars() {
            result.push(ch);
        }
    }

    pub fn into_rna(self) -> Rna {
        todo!("Transform Dna {self:?} into corresponding Rna");
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        todo!(
            "Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"
        );
    }
}
