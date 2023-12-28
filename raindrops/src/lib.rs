use std::collections::BTreeMap;

pub fn raindrops(n: u32) -> String {
    let factors = BTreeMap::from([(3, "Pling"), (5, "Plang"), (7, "Plong")]);
    let result = factors
        .keys()
        .map(|factor| if n % factor == 0 { factors[factor] } else { "" })
        .collect::<String>();
    if result.is_empty() {
        return n.to_string();
    }
    result
}
