use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::<char, i32>::new(), |mut acc, (value, letters)| {
        for letter in letters {
            acc.entry(letter.to_ascii_lowercase()).or_insert(*value);
        }
        acc
    })
}
