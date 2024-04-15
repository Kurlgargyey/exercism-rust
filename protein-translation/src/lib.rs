use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result = Vec::new();
        for codon in rna
            .chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>()) {
            if let Some(name) = self.name_for(&codon) {
                match name {
                    "stop codon" => {
                        return Some(result);
                    }
                    _ => {
                        result.push(name);
                    }
                }
            } else {
                return None;
            }
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: {
            pairs.iter().fold(HashMap::<&'a str, &'a str>::new(), |mut map, (codon, name)| {
                map.insert(codon, name);
                map
            })
        },
    }
}
